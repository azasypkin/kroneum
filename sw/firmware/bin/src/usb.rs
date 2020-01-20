use crate::hal::stm32::Peripherals;

use crate::system::SystemHardwareImpl;
use kroneum_api::usb::{
    EndpointDirection, EndpointStatus, EndpointType, Transaction, TransactionFlags, USBHardware,
    UsbInterrupt,
};

const BTABLE_ADDRESS: usize = 0x4000_6000;

fn status_bits(current_bits: u8, status: EndpointStatus) -> u8 {
    current_bits ^ status as u8
}

impl USBHardware for SystemHardwareImpl {
    /// Setups USB hardware, but doesn't activate it.
    fn setup(&self) {
        // Set alternative function #2 (USB) for PA11 and PA12.
        self.p
            .GPIOA
            .afrh
            .modify(|_, w| w.afrh11().af2().afrh12().af2());

        start_clock(&self.p);

        self.p.RCC.apb1enr.modify(|_, w| w.usben().enabled());

        // Reset the peripheral.
        self.p
            .USB
            .cntr
            .modify(|_, w| w.pdwn().disabled().fres().reset());
        self.p.USB.cntr.modify(|_, w| w.fres().no_reset());

        // Reset any pending interrupts.
        self.p.USB.istr.reset();

        // Set interrupt mask.
        self.p
            .USB
            .cntr
            .modify(|_, w| w.ctrm().enabled().errm().enabled().resetm().enabled());

        self.p.USB.bcdr.modify(|_, w| w.dppu().enabled());
    }

    /// Tears down USB hardware.
    fn teardown(&self) {
        // Disable all interrupts and force the USB reset.
        self.p.USB.cntr.write(|w| w.fres().reset());

        // Clear the interrupt status register.
        self.p.USB.istr.reset();

        // Switch-off the USB device.
        self.p.USB.cntr.write(|w| w.pdwn().enabled().fres().reset());

        // Tell the host that we're gone by disabling pull-up on DP.
        self.p.USB.bcdr.modify(|_, w| w.dppu().disabled());

        // USB clock off.
        self.p.RCC.apb1enr.modify(|_, w| w.usben().disabled());

        stop_clock(&self.p);

        // Set alternative function #0 (USB) for PA11 and PA12.
        self.p
            .GPIOA
            .afrh
            .modify(|_, w| w.afrh11().af0().afrh12().af0());
    }

    fn enable(&self) {
        self.p.USB.daddr.write(|w| w.ef().enabled());
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
        let istr_reg = self.p.USB.istr.read();
        let endpoint_index = istr_reg.ep_id().bits();
        let ep_reg = self.p.USB.epr[endpoint_index as usize].read();
        Transaction {
            endpoint: match endpoint_index {
                0 => EndpointType::Control,
                1 => EndpointType::Device,
                _ => panic!("Unknown endpoint"),
            },
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
        let endpoint_index = match endpoint {
            EndpointType::Control => 0,
            EndpointType::Device => 1,
        };

        self.p.USB.epr[endpoint_index].modify(|r, w| {
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
        self.p
            .USB
            .daddr
            .write(|w| w.add().bits(address).ef().enabled());
    }

    fn open_endpoint(&self, endpoint: EndpointType) {
        let endpoint_index = match endpoint {
            EndpointType::Control => 0,
            EndpointType::Device => 1,
        };

        self.p.USB.epr[endpoint_index].modify(|r, w| {
            let (endpoint_type, endpoint_address) = match endpoint {
                EndpointType::Control => (0b01, r.ea().bits()),
                EndpointType::Device => (0b11, 0x1),
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
        let endpoint_index = match endpoint {
            EndpointType::Control => 0,
            EndpointType::Device => 1,
        };

        self.p.USB.epr[endpoint_index].modify(|r, w| {
            w.stat_tx()
                .bits(status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn is_interrupt_active(&self, interrupt: UsbInterrupt) -> bool {
        let interrupt_flags = self.p.USB.istr.read();
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
            UsbInterrupt::Reset => self.p.USB.istr.write(|w| unsafe { w.bits(0xFBFF) }),
            UsbInterrupt::Error => self.p.USB.istr.write(|w| unsafe { w.bits(0xDFFF) }),
            UsbInterrupt::CorrectTransfer => {
                // `ctr` is read-only attribute.
            }
            UsbInterrupt::SuspendSoFEsoF => self.p.USB.istr.write(|w| unsafe { w.bits(0xF4FF) }),
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

        let endpoint_index = match endpoint {
            EndpointType::Control => 0,
            EndpointType::Device => 1,
        };

        let stat_bits = 0b00;
        self.p.USB.epr[endpoint_index].modify(|_, w| {
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

fn start_clock(p: &Peripherals) {
    // Enable HSI48.
    p.RCC.cr2.modify(|_, w| w.hsi48on().set_bit());
    while p.RCC.cr2.read().hsi48rdy().bit_is_clear() {}

    // Enable clock recovery system from USB SOF frames.
    p.RCC.apb1enr.modify(|_, w| w.crsen().set_bit());

    // Before configuration, reset CRS registers to their default values.
    p.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    p.RCC.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Configure Frequency Error Measurement.

    // Enable Automatic trimming.
    p.CRS.cr.modify(|_, w| w.autotrimen().set_bit());
    // Enable Frequency error counter.
    p.CRS.cr.modify(|_, w| w.cen().set_bit());
}

fn stop_clock(p: &Peripherals) {
    // Disable Frequency error counter.
    p.CRS.cr.modify(|_, w| w.cen().clear_bit());

    // Reset CRS registers to their default values.
    p.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    p.RCC.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Disable clock recovery system from USB SOF frames.
    p.RCC.apb1enr.modify(|_, w| w.crsen().clear_bit());

    // Disable HSI48.
    p.RCC.cr2.modify(|_, w| w.hsi48on().clear_bit());
    while p.RCC.cr2.read().hsi48rdy().bit_is_set() {}
}
