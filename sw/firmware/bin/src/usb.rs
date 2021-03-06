use crate::{
    hal::stm32::{CRS, RCC},
    system::SystemHardwareImpl,
};
use core::convert::TryFrom;
use kroneum_api::usb::{
    endpoint::{EndpointDirection, EndpointStatus, EndpointType},
    Transaction, TransactionFlags, USBHardware, UsbInterrupt,
};

const BTABLE_ADDRESS: usize = 0x4000_6000;

fn status_bits(current_bits: u8, status: EndpointStatus) -> u8 {
    current_bits ^ status as u8
}

impl USBHardware for SystemHardwareImpl {
    /// Setups USB hardware, but doesn't activate it.
    fn setup(&self) {
        start_clock(&self.rcc.regs, &self.crs);

        self.rcc.regs.apb1enr.modify(|_, w| w.usben().enabled());

        // Reset the peripheral.
        self.usb
            .cntr
            .modify(|_, w| w.pdwn().disabled().fres().reset());
        self.usb.cntr.modify(|_, w| w.fres().no_reset());

        // Reset any pending interrupts.
        self.usb.istr.reset();

        // Set interrupt mask.
        self.usb
            .cntr
            .modify(|_, w| w.ctrm().enabled().errm().enabled().resetm().enabled());

        self.usb.bcdr.modify(|_, w| w.dppu().enabled());
    }

    /// Tears down USB hardware.
    fn teardown(&self) {
        // Disable all interrupts and force the USB reset.
        self.usb.cntr.write(|w| w.fres().reset());

        // Clear the interrupt status register.
        self.usb.istr.reset();

        // Switch-off the USB device.
        self.usb.cntr.write(|w| w.pdwn().enabled().fres().reset());

        // Tell the host that we're gone by disabling pull-up on DP.
        self.usb.bcdr.modify(|_, w| w.dppu().disabled());

        // USB clock off.
        self.rcc.regs.apb1enr.modify(|_, w| w.usben().disabled());

        stop_clock(&self.rcc.regs, &self.crs);
    }

    fn enable(&self) {
        self.usb.daddr.write(|w| w.ef().enabled());
    }

    fn btable_address(&self) -> usize {
        BTABLE_ADDRESS
    }

    fn transaction(&self) -> Transaction {
        // This bit is written by the hardware according to the direction of the successful transaction,
        // which generated the interrupt request. If DIR bit=0, CTR_TX bit is set in the USB_EPnR register
        // related to the interrupting endpoint. The interrupting transaction is of IN type (data
        // transmitted by the USB peripheral to the host PC).
        //
        // If DIR bit=1, CTR_RX bit or both CTR_TX/CTR_RX are set in the USB_EPnR register related to
        // the interrupting endpoint. The interrupting transaction is of OUT type (data received by the
        // USB peripheral from the host PC) or two pending transactions are waiting to be processed.
        let istr_reg = self.usb.istr.read();
        let endpoint_index = istr_reg.ep_id().bits();
        let endpoint = if let Ok(endpoint) = EndpointType::try_from(endpoint_index) {
            endpoint
        } else {
            panic!("Unknown endpoint");
        };

        let ep_reg = self.usb.epr[endpoint_index as usize].read();
        Transaction {
            endpoint,
            direction: if istr_reg.dir().is_from() {
                EndpointDirection::Receive
            } else {
                EndpointDirection::Transmit
            },
            flags: TransactionFlags {
                setup: ep_reg.setup().bit_is_set(),
                rx: ep_reg.ctr_rx().bit_is_set(),
                tx: ep_reg.ctr_tx().bit_is_set(),
            },
        }
    }

    fn set_endpoint_status(
        &self,
        endpoint: EndpointType,
        direction: EndpointDirection,
        status: EndpointStatus,
    ) {
        self.usb.epr[Into::<u8>::into(endpoint) as usize].modify(|r, w| {
            let (rx, tx) = match direction {
                EndpointDirection::Receive => (status_bits(r.stat_rx().bits(), status), 0b00),
                EndpointDirection::Transmit => (0b00, status_bits(r.stat_tx().bits(), status)),
            };

            // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
            w.ctr_tx()
                .set_bit()
                .ctr_rx()
                .set_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_rx()
                .bits(rx)
                .stat_tx()
                .bits(tx)
        });
    }

    fn set_address(&self, address: u8) {
        self.usb
            .daddr
            .write(|w| w.add().bits(address).ef().enabled());
    }

    fn open_endpoint(&self, endpoint: EndpointType) {
        let endpoint_index = Into::<u8>::into(endpoint);
        self.usb.epr[endpoint_index as usize].modify(|r, w| {
            let (endpoint_type, endpoint_address) = match endpoint {
                EndpointType::Control => (0b01, r.ea().bits()),
                EndpointType::Device(_) => (0b11, endpoint_index),
            };

            w.ep_type()
                .bits(endpoint_type)
                .ea()
                .bits(endpoint_address)
                .stat_rx()
                .bits(status_bits(r.stat_rx().bits(), EndpointStatus::Valid))
                .stat_tx()
                .bits(status_bits(r.stat_tx().bits(), EndpointStatus::Nak))
                .ctr_rx()
                .set_bit()
                .ctr_tx()
                .set_bit()
        });
    }

    fn close_endpoint(&self, endpoint: EndpointType) {
        self.usb.epr[Into::<u8>::into(endpoint) as usize].modify(|r, w| {
            w.stat_tx()
                .bits(status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn is_interrupt_active(&self, interrupt: UsbInterrupt) -> bool {
        let interrupt_flags = self.usb.istr.read();
        match interrupt {
            UsbInterrupt::Reset => interrupt_flags.reset().bit_is_set(),
            UsbInterrupt::Error => interrupt_flags.err().bit_is_set(),
            UsbInterrupt::CorrectTransfer => interrupt_flags.ctr().bit_is_set(),
            UsbInterrupt::SuspendSoFEsoF => {
                interrupt_flags.susp().bit_is_set()
                    || interrupt_flags.sof().bit_is_set()
                    || interrupt_flags.esof().bit_is_set()
            }
        }
    }

    fn mark_interrupt_as_handled(&self, interrupt: UsbInterrupt) {
        match interrupt {
            UsbInterrupt::Reset => self.usb.istr.write(|w| unsafe { w.bits(0xFBFF) }),
            UsbInterrupt::Error => self.usb.istr.write(|w| unsafe { w.bits(0xDFFF) }),
            UsbInterrupt::CorrectTransfer => {
                // `ctr` is read-only attribute.
            }
            UsbInterrupt::SuspendSoFEsoF => self.usb.istr.write(|w| unsafe { w.bits(0xF4FF) }),
        };
    }

    fn mark_transaction_as_handled(&self, endpoint: EndpointType, direction: EndpointDirection) {
        // These bits are set by the hardware when an OUT/IN transaction is successfully completed
        // on this endpoint; the software can only clear this bit.
        // A transaction ended with a NAK or STALL handshake does not set this bit, since no data is
        // actually transferred, as in the case of protocol errors or data toggle mismatches.
        // This bit is read/write but only 0 (false) can be written, writing 1 (true) has no effect.
        let (rx_bit, tx_bit) = match direction {
            EndpointDirection::Receive => (false, true),
            EndpointDirection::Transmit => (true, false),
        };

        let stat_bits = 0b00;
        self.usb.epr[Into::<u8>::into(endpoint) as usize].modify(|_, w| {
            w.ctr_rx()
                .bit(rx_bit)
                .ctr_tx()
                .bit(tx_bit)
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(stat_bits)
                .stat_rx()
                .bits(stat_bits)
        });
    }
}

fn start_clock(rcc: &RCC, crs: &CRS) {
    // Enable HSI48.
    rcc.cr2.modify(|_, w| w.hsi48on().set_bit());
    while rcc.cr2.read().hsi48rdy().bit_is_clear() {}

    // Enable clock recovery system from USB SOF frames.
    rcc.apb1enr.modify(|_, w| w.crsen().set_bit());

    // Before configuration, reset CRS registers to their default values.
    rcc.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Configure Frequency Error Measurement.

    // Enable Automatic trimming.
    crs.cr.modify(|_, w| w.autotrimen().set_bit());
    // Enable Frequency error counter.
    crs.cr.modify(|_, w| w.cen().set_bit());
}

fn stop_clock(rcc: &RCC, crs: &CRS) {
    // Disable Frequency error counter.
    crs.cr.modify(|_, w| w.cen().clear_bit());

    // Reset CRS registers to their default values.
    rcc.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Disable clock recovery system from USB SOF frames.
    rcc.apb1enr.modify(|_, w| w.crsen().clear_bit());

    // Disable HSI48.
    rcc.cr2.modify(|_, w| w.hsi48on().clear_bit());
    while rcc.cr2.read().hsi48rdy().bit_is_set() {}
}
