use stm32f0::stm32f0x2::Peripherals;

use kroneum_api::usb::{
    EndpointDirection, EndpointStatus, EndpointType, Transaction, TransactionFlags, USBHardware,
    UsbInterrupt, UsbState,
};

pub type USB<'a> = kroneum_api::usb::USB<'a, USBHardwareImpl<'a>>;

pub struct USBHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> USBHardwareImpl<'a> {
    fn open_control_endpoints(&self) {
        self.p.USB.ep0r.write(|w| unsafe {
            w.ep_type()
                .bits(0b01)
                .ctr_rx()
                .set_bit()
                .ctr_tx()
                .set_bit()
                .stat_tx()
                .bits(self.status_bits(0, EndpointStatus::Nak))
                .stat_rx()
                .bits(self.status_bits(0, EndpointStatus::Valid))
        });
    }

    fn open_device_endpoints(&self) {
        self.p.USB.ep1r.modify(|r, w| unsafe {
            w.ep_type()
                .bits(0b11)
                .ea()
                .bits(0x1)
                .ctr_rx()
                .set_bit()
                .ctr_tx()
                .set_bit()
                .stat_tx()
                .bits(self.status_bits(r.stat_tx().bits(), EndpointStatus::Nak))
                .stat_rx()
                .bits(self.status_bits(r.stat_rx().bits(), EndpointStatus::Valid))
        });
    }

    fn close_control_endpoints(&self) {
        self.p.USB.ep0r.modify(|r, w| unsafe {
            w.stat_tx()
                .bits(self.status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn close_device_endpoints(&self) {
        self.p.USB.ep1r.modify(|r, w| unsafe {
            w.stat_tx()
                .bits(self.status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn set_rx_endpoint_status(&self, endpoint: EndpointType, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            EndpointType::Control => {
                self.p.USB.ep0r.modify(|r, w| unsafe {
                    w.stat_rx()
                        .bits(self.status_bits(r.stat_rx().bits(), status))
                        .ctr_tx()
                        .set_bit()
                        .ctr_rx()
                        .set_bit()
                        .dtog_tx()
                        .clear_bit()
                        .dtog_rx()
                        .clear_bit()
                        .stat_tx()
                        .bits(0b00)
                });
            }
            EndpointType::Device => self.p.USB.ep1r.modify(|r, w| unsafe {
                w.stat_rx()
                    .bits(self.status_bits(r.stat_rx().bits(), status))
                    .ctr_tx()
                    .set_bit()
                    .ctr_rx()
                    .set_bit()
                    .dtog_tx()
                    .clear_bit()
                    .dtog_rx()
                    .clear_bit()
                    .stat_tx()
                    .bits(0b00)
            }),
        }
    }

    fn set_tx_endpoint_status(&self, endpoint: EndpointType, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            EndpointType::Control => {
                self.p.USB.ep0r.modify(|r, w| unsafe {
                    w.stat_tx()
                        .bits(self.status_bits(r.stat_tx().bits(), status))
                        .ctr_tx()
                        .set_bit()
                        .ctr_rx()
                        .set_bit()
                        .dtog_tx()
                        .clear_bit()
                        .dtog_rx()
                        .clear_bit()
                        .stat_rx()
                        .bits(0b00)
                });
            }
            EndpointType::Device => self.p.USB.ep1r.modify(|r, w| unsafe {
                w.stat_tx()
                    .bits(self.status_bits(r.stat_tx().bits(), status))
                    .ctr_tx()
                    .set_bit()
                    .ctr_rx()
                    .set_bit()
                    .dtog_tx()
                    .clear_bit()
                    .dtog_rx()
                    .clear_bit()
                    .stat_rx()
                    .bits(0b00)
            }),
        }
    }

    fn status_bits(&self, current_bits: u8, status: EndpointStatus) -> u8 {
        current_bits ^ status as u8
    }
}

impl<'a> USBHardware for USBHardwareImpl<'a> {
    /// Setups USB hardware, but doesn't activate it.
    fn setup(&self) {
        start_clock(self.p);

        self.p.RCC.apb1enr.modify(|_, w| w.usben().set_bit());

        // Reset the peripheral.
        self.p
            .USB
            .cntr
            .modify(|_, w| w.pdwn().clear_bit().fres().set_bit());
        self.p.USB.cntr.modify(|_, w| w.fres().clear_bit());

        // Reset any pending interrupts.
        self.p.USB.istr.reset();

        // Set interrupt mask.
        self.p
            .USB
            .cntr
            .modify(|_, w| w.ctrm().set_bit().errm().set_bit().resetm().set_bit());

        self.p.USB.bcdr.modify(|_, w| w.dppu().set_bit());
    }

    /// Tears down USB hardware.
    fn teardown(&self) {
        // Tell the host that we're gone by disabling pull-up on DP.
        self.p.USB.bcdr.modify(|_, w| w.dppu().clear_bit());

        // USB clock off.
        self.p.RCC.apb1enr.modify(|_, w| w.usben().clear_bit());

        stop_clock(self.p);
    }

    fn enable(&self) {
        self.p.USB.daddr.write(|w| w.ef().set_bit());
    }

    fn transaction(&self) -> Transaction {
        let istr_reg = self.p.USB.istr.read();

        let endpoint = match istr_reg.ep_id().bits() {
            0 => EndpointType::Control,
            1 => EndpointType::Device,
            _ => panic!("Unknown endpoint"),
        };

        // This bit is written by the hardware according to the direction of the successful transaction,
        // which generated the interrupt request. If DIR bit=0, CTR_TX bit is set in the USB_EPnR register
        // related to the interrupting endpoint. The interrupting transaction is of IN type (data
        // transmitted by the USB peripheral to the host PC).
        //
        // If DIR bit=1, CTR_RX bit or both CTR_TX/CTR_RX are set in the USB_EPnR register related to
        // the interrupting endpoint. The interrupting transaction is of OUT type (data received by the
        // USB peripheral from the host PC) or two pending transactions are waiting to be processed.
        let direction = if istr_reg.dir().bit_is_set() {
            EndpointDirection::Receive
        } else {
            EndpointDirection::Transmit
        };

        let flags = match endpoint {
            EndpointType::Control => {
                let ep_reg = self.p.USB.ep0r.read();
                TransactionFlags {
                    setup: ep_reg.setup().bit_is_set(),
                    rx: ep_reg.ctr_rx().bit_is_set(),
                    tx: ep_reg.ctr_tx().bit_is_set(),
                }
            }
            EndpointType::Device => {
                let ep_reg = self.p.USB.ep1r.read();
                TransactionFlags {
                    setup: ep_reg.setup().bit_is_set(),
                    rx: ep_reg.ctr_rx().bit_is_set(),
                    tx: ep_reg.ctr_tx().bit_is_set(),
                }
            }
        };

        Transaction {
            endpoint,
            direction,
            flags,
        }
    }

    fn set_endpoint_status(
        &self,
        endpoint: EndpointType,
        direction: EndpointDirection,
        status: EndpointStatus,
    ) {
        match direction {
            EndpointDirection::Receive => self.set_rx_endpoint_status(endpoint, status),
            EndpointDirection::Transmit => self.set_tx_endpoint_status(endpoint, status),
        }
    }

    fn set_address(&self, address: u8) {
        self.p
            .USB
            .daddr
            .write(|w| unsafe { w.add().bits(address).ef().set_bit() });
    }

    fn open_endpoint(&self, endpoint: EndpointType) {
        match endpoint {
            EndpointType::Control => self.open_control_endpoints(),
            EndpointType::Device => self.open_device_endpoints(),
        }
    }

    fn close_endpoint(&self, endpoint: EndpointType) {
        match endpoint {
            EndpointType::Control => self.close_control_endpoints(),
            EndpointType::Device => self.close_device_endpoints(),
        }
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
        let stat_bits = 0b00;

        // These bits are set by the hardware when an OUT/IN transaction is successfully completed
        // on this endpoint; the software can only clear this bit.
        // A transaction ended with a NAK or STALL handshake does not set this bit, since no data is
        // actually transferred, as in the case of protocol errors or data toggle mismatches.
        // This bit is read/write but only 0 (false) can be written, writing 1 (true) has no effect.
        let (rx_bit, tx_bit) = match direction {
            EndpointDirection::Receive => (false, true),
            EndpointDirection::Transmit => (true, false),
        };

        match endpoint {
            EndpointType::Control => {
                self.p.USB.ep0r.modify(|_, w| unsafe {
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
            EndpointType::Device => {
                self.p.USB.ep1r.modify(|_, w| unsafe {
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

pub fn create<'a>(p: &'a Peripherals, state: &'a mut UsbState) -> USB<'a> {
    USB::create(USBHardwareImpl { p }, state)
}
