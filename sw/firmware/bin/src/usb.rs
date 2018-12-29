mod pma;

use core::cell::RefCell;
use core::mem::transmute;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use stm32f0x2::Interrupt;
use stm32f0x2::Peripherals;

use pma::PacketMemoryArea;

pub enum UsbRequestDirection {
    HostToDevice,
    DeviceToHost,
}

pub enum UsbRequestKind {
    Standard,
    Class,
    Vendor,
    Reserved,
}

pub enum UsbRequestRecipient {
    Device,
    Interface,
    Endpoint,
    Other,
    Reserved,
}

pub struct UsbRequestType {
    dir: UsbRequestDirection,
    kind: UsbRequestKind,
    recipient: UsbRequestRecipient,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum UsbRequest {
    GetStatus = 0x00,
    ClearFeature = 0x01,
    Two = 0x2,
    SetFeature = 0x03,
    SetAddress = 0x05,
    GetDescriptor = 0x06,
    SetDescriptor = 0x07,
    GetConfiguration = 0x08,
    SetConfiguration = 0x09,
    GetInterface = 0x0A,
    SetInterface = 0x11,
    SynchFrame = 0x12,
}

struct UsbRequestHeader {
    request: UsbRequest,
    dir: UsbRequestDirection,
    kind: UsbRequestKind,
    recipient: UsbRequestRecipient,
    value: u16,
    index: u16,
    data_length: u16,
}

impl From<(u16, u16, u16, u16)> for UsbRequestHeader {
    #[inline]
    fn from((request_header, value, index, data_length): (u16, u16, u16, u16)) -> Self {
        let request_type = (request_header & 0xff) as u8;

        UsbRequestHeader {
            request: unsafe { transmute(((request_header & 0xff00) >> 8) as u8) },
            // Bit 7
            dir: match (request_type & 0b1000_0000) >> 7 {
                0 => UsbRequestDirection::HostToDevice,
                1 => UsbRequestDirection::DeviceToHost,
                _ => panic!("Unreachable"),
            },
            // Bits 6:5
            kind: match (request_type & 0b0110_0000) >> 5 {
                0b000 => UsbRequestKind::Standard,
                0b001 => UsbRequestKind::Class,
                0b010 => UsbRequestKind::Vendor,
                0b011 => UsbRequestKind::Reserved,
                _ => panic!("Unreachable"),
            },
            // Bits 4:0
            recipient: match request_type & 0b0001_1111 {
                0b0000_0000 => UsbRequestRecipient::Device,
                0b0000_0001 => UsbRequestRecipient::Interface,
                0b0000_0010 => UsbRequestRecipient::Endpoint,
                0b0000_0011 => UsbRequestRecipient::Other,
                0b0000_0100...0b0001_1111 => UsbRequestRecipient::Reserved,
                _ => panic!("Unreachable"),
            },
            value,
            index,
            data_length,
        }
    }
}

enum Endpoint<'a> {
    Endpoint0(&'a stm32f0x2::usb::EP0R),
    Endpoint1(&'a stm32f0x2::usb::EP1R),
}

#[derive(Copy, Clone)]
enum EndpointType {
    Bulk = 0b0,
    Control = 0b01,
    Iso = 0b10,
    Interrupt = 0b11,
}

#[derive(Copy, Clone)]
enum EndpointStatus {
    Disabled = 0b0,
    Stall = 0b01,
    Nak = 0b10,
    Valid = 0b11,
}

#[derive(Copy, Clone)]
enum DeviceState {
    // USB isn't started.
    None,
    // Device is starting, or has disconnected.
    Default,
    // We've received an address from the host.
    Addressed,
    // Enumeration is complete, we can talk to the host.
    Configured,
    // Device is suspended.
    Suspended,
    // Synthetic state for the woken up device,
    WokenUp,
}

// The possible states for the control endpoint.
#[derive(Copy, Clone)]
enum ControlEndpointState {
    Idle,
    Setup,
    DataIn,
    DataOut,
    StatusIn,
    StatusOut,
    Stall,
}

struct UsbState {
    device_state: DeviceState,
    suspended_device_state: Option<DeviceState>,
    control_endpoint_state: ControlEndpointState,
    address: u8,
}

/*
 * These are the USB device strings in the format required for a USB string descriptor.
 * To change these to suit your device you need only change the unicode string in the
 * last line of each definition to suit your device. Then count up the bytes required for
 * the complete descriptor and go back and insert that byte count in the array declaration
 * in the configuration class.
 */

pub struct USB<'a> {
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
    pma: PacketMemoryArea,
}

static STATE: Mutex<RefCell<(DeviceState, ControlEndpointState, Option<DeviceState>)>> = Mutex::new(
    RefCell::new((DeviceState::None, ControlEndpointState::Idle, None)),
);

fn set_device_state(state: DeviceState) {
    free(|cs| {
        let (current_device_state, ces, device_state_before_suspend) = *STATE.borrow(cs).borrow();

        *STATE.borrow(cs).borrow_mut() = match (state, device_state_before_suspend) {
            (DeviceState::Suspended, _) => (state, ces, Some(current_device_state)),
            (DeviceState::WokenUp, Some(previous_device_state)) => {
                (previous_device_state, ces, None)
            }
            (DeviceState::WokenUp, None) => (current_device_state, ces, None),
            _ => (state, ces, device_state_before_suspend),
        };
    });
}

fn set_control_endpoint_state(state: ControlEndpointState) {
    free(|cs| {
        let (current_device_state, _, device_state_before_suspend) = *STATE.borrow(cs).borrow();

        *STATE.borrow(cs).borrow_mut() = (current_device_state, state, device_state_before_suspend);
    });
}

fn get_control_endpoint_state() -> ControlEndpointState {
    free(|cs| (*STATE.borrow(cs).borrow()).1)
}

const CONTROL_OUT_PMA_ADDRESS: usize = 0x18;
const CONTROL_IN_PMA_ADDRESS: usize = 0x58;
const DEVICE_IN_PMA_ADDRESS: usize = 0x98;
const DEVICE_OUT_PMA_ADDRESS: usize = 0xD8;

impl<'a> USB<'a> {
    fn new(core_peripherals: &'a mut CorePeripherals, peripherals: &'a Peripherals) -> USB<'a> {
        USB {
            core_peripherals,
            peripherals,
            pma: PacketMemoryArea {},
        }
    }

    pub fn acquire<'b, F>(
        core_peripherals: &'b mut CorePeripherals,
        peripherals: &'b Peripherals,
        f: F,
    ) -> ()
    where
        F: FnOnce(USB),
    {
        f(USB::new(core_peripherals, peripherals));
    }

    pub fn configure(peripherals: &Peripherals, core_peripherals: &mut CorePeripherals) {
        // Enable HSI48 and wait until it's ready.
        peripherals.RCC.cr2.modify(|_, w| w.hsi48on().set_bit());
        while peripherals.RCC.cr2.read().hsi48rdy().bit_is_clear() {}

        // Disable the PLL and wait until it's turned off.
        peripherals.RCC.cr.modify(|_, w| w.pllon().clear_bit());
        while peripherals.RCC.cr.read().pllrdy().bit_is_set() {}

        // Select HSI48 as the USB clock source.
        peripherals.RCC.cfgr3.modify(|_, w| w.usbsw().clear_bit());

        // Make AHB and APB clocks not divided on anything.
        peripherals.RCC.cfgr.modify(|_, w| unsafe {
            w.hpre().bits(0b0);
            w.ppre().bits(0b0);
            w
        });

        // Select HSI48 (0b11) as system clock.
        peripherals
            .RCC
            .cfgr
            .modify(|_, w| unsafe { w.sw().bits(0b11) });
        while peripherals.RCC.cfgr.read().sws().bits() != 0b11 {}

        // Enable clock recovery system from USB SOF frames.
        peripherals.RCC.apb1enr.modify(|_, w| w.crsen().set_bit());

        // Before configuration, reset CRS registers to their default values.
        peripherals.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
        peripherals
            .RCC
            .apb1rstr
            .modify(|_, w| w.crsrst().clear_bit());

        // Configure Synchronization input.
        peripherals.CRS.cfgr.modify(|_, w| unsafe {
            // Clear SYNCDIV[2:0], SYNCSRC[1:0] & SYNCSPOL bits.
            w.syncdiv().bits(0b0);
            w.syncpol().clear_bit();

            // USB SOF selected as SYNC signal source (default).
            w.syncsrc().bits(0b10);

            // Reset Frequency Error Measurement.
            w.reload().bits(0b0);
            w.felim().bits(0b0);

            w
        });

        peripherals.CRS.cfgr.modify(|_, w| unsafe {
            // Configure Frequency Error Measurement.
            // (f TARGET / f SYNC ) - 1 The reset value of the RELOAD field corresponds to a target
            // frequency of 48 MHz and a synchronization signal frequency of 1 kHz (SOF signal from USB).
            // (48MHz/1kHz) - 1.
            w.reload().bits(47999);

            // f TARGET / f SYNC ) * STEP[%] / 100% / 2. The reset value of the FELIM field corresponds to
            // (f TARGET / f SYNC ) = 48000 and to a typical trimming step size of 0.14%. The result should
            // be always rounded up to the nearest integer value in order to obtain the best trimming
            // response. 48000 * (0.14 / 100 / 2) = 33.6 ~= 34
            w.felim().bits(34);

            w
        });

        // Adjust HSI48 oscillator smooth trimming.
        peripherals.CRS.cr.write(|w| {
            // The default value is 32, which corresponds to the middle of the trimming interval. The
            // trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value
            // corresponds to a higher output frequency.
            unsafe { w.trim().bits(32) };

            // Enable Automatic trimming.
            w.autotrimen().set_bit();

            // Enable Frequency error counter.
            w.cen().set_bit();

            w
        });

        peripherals.RCC.apb1enr.write(|w| w.usbrst().set_bit());
        core_peripherals.NVIC.enable(Interrupt::USB);

        // reset the peripheral and any pending interrupts
        peripherals.USB.cntr.write(|w| w.fres().set_bit());
        peripherals.USB.cntr.write(|w| unsafe { w.bits(0b0) });

        peripherals.USB.istr.write(|w| unsafe { w.bits(0b0) });

        peripherals.USB.cntr.write(|w| {
            w.ctrm()
                .set_bit()
                .wkupm()
                .set_bit()
                .suspm()
                .set_bit()
                .errm()
                .set_bit()
                .esofm()
                .set_bit()
                .resetm()
                .set_bit()
                .pmaovrm()
                .set_bit()
        });

        peripherals.USB.bcdr.write(|w| w.dppu().set_bit());

        set_device_state(DeviceState::Default);
    }

    pub fn interrupt(&self) {
        let istr = self.peripherals.USB.istr.read();

        if istr.reset().bit_is_set() {
            self.reset();
        }

        if istr.susp().bit_is_set() {
            self.suspend();
        }

        if istr.wkup().bit_is_set() {
            self.wake_up();
        }

        self.peripherals.USB.istr.write(|w| {
            w.pmaovr()
                .clear_bit()
                .err()
                .clear_bit()
                .sof()
                .clear_bit()
                .esof()
                .clear_bit()
        });

        // Correct endpoint transfer
        if istr.ctr().bit_is_set() {
            self.correct_transfer();
        }
    }

    pub fn reset(&self) {
        self.peripherals.USB.istr.write(|w| w.reset().clear_bit());

        self.reset_buffer_table();

        self.set_address(0);
        self.open_control_endpoints();
    }

    fn correct_transfer(&self) {
        // USB_ISTR_CTR is read only and will be automatically cleared by
        // hardware when we've processed all endpoint results.
        while self.peripherals.USB.istr.read().ctr().bit_is_set() {
            let istr = self.peripherals.USB.istr.read();
            let endpoint = istr.ep_id().bits();
            let is_out = istr.dir().bit_is_set();

            match endpoint {
                0 => {
                    if is_out {
                        self.handle_control_out_transfer();
                    } else {
                        self.handle_control_in_transfer();
                    }
                }
                1 => {
                    let ep1 = self.peripherals.USB.ep1r.read();
                    if is_out && ep1.ctr_rx().bit_is_set() {
                        self.handle_device_out_transfer();
                    } else if !is_out && ep1.ctr_tx().bit_is_set() {
                        self.handle_device_out_transfer();
                    }
                }
                _ => panic!("Unknown endpoint"),
            }
        }
    }

    fn handle_control_out_transfer(&self) {
        let ep0 = self.peripherals.USB.ep0r.read();
        if ep0.setup().bit_is_set() {
            self.handle_control_setup_out_transfer();
        } else if ep0.ctr_rx().bit_is_set() {
            self.handle_control_data_out_transfer();
        }
    }

    fn handle_control_setup_out_transfer(&self) {
        let header = UsbRequestHeader::from((
            self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS),
            self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS + 2),
            self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS + 4),
            self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS + 6),
        ));

        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.write(|w| w.ctr_rx().clear_bit());

        set_control_endpoint_state(ControlEndpointState::Setup);

        match header.recipient {
            UsbRequestRecipient::Device => self.handle_device_request(header),
            UsbRequestRecipient::Interface => self.handle_interface_request(header),
            UsbRequestRecipient::Endpoint => self.handle_endpoint_request(header),
            _ => self.set_rx_endpoint_status(&Endpoint::Endpoint0(endpoint), EndpointStatus::Stall),
        }

        /* self.pma.set_u16(6, (0x8000 | 1 << 10) as u16);*/
    }

    fn handle_control_data_out_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.write(|w| w.ctr_rx().clear_bit());

        // Here we can check the amount of data and do smth with it....

        self.pma.set_u16(
            6,
            (0x8000 | (1 << 10)) as u16, /* 32 byte size, 1 block */
        );

        self.set_rx_endpoint_status(&Endpoint::Endpoint0(endpoint), EndpointStatus::Valid);
    }

    fn handle_control_in_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.write(|w| w.ctr_tx().clear_bit());

        if let ControlEndpointState::DataIn = get_control_endpoint_state() {
            /*
            if(_controlEndpointState==ControlEndpointStateType::DATA_IN) {

            if(_inEndpointData[0].remaining) {

                // continue sending the next in the multi-packet transfer

                continueSendData(0);

                // prepare for premature end of transfer

                USBR_BDT[0].rx.setRxCount(0);
                setRxEndpointStatus(&USBR->EP0R,USB_EP_RX_VALID);
            }
            else {

                // if we're sending a multiple of max packet size then a zero length is required

                if((_inEndpointData[0].total % CONTROL_MAX_PACKET_SIZE)==0 &&
                    _inEndpointData[0].total>CONTROL_MAX_PACKET_SIZE &&
                    _inEndpointData[0].total<_setupDataLength) {

                    // send zero length packet

                    sendData(0,CONTROL_IN_PMA_ADDRESS,CONTROL_MAX_PACKET_SIZE,nullptr,0);

                    // prepare for premature end of transfer

                    USBR_BDT[0].rx.setRxCount(0);
                    setRxEndpointStatus(&USBR->EP0R,USB_EP_RX_VALID);
                }
                else {
                    _controlEndpointState=ControlEndpointStateType::STATUS_OUT;
                    USBR_BDT[0].rx.setRxCount(0);
                    setRxEndpointStatus(&USBR->EP0R,USB_EP_RX_VALID);
                }
            }
        }
            */
        }

        /* if(_address>0 && _inEndpointData[0].remaining==0) {
            USBR->DADDR=_address | USB_DADDR_EF;
            _address=0;
        }*/
    }

    fn handle_device_request(&self, request_header: UsbRequestHeader) {}

    fn handle_interface_request(&self, request_header: UsbRequestHeader) {}

    fn handle_endpoint_request(&self, request_header: UsbRequestHeader) {}

    fn handle_device_out_transfer(&self) {}

    fn handle_device_in_transfer(&self) {}

    fn suspend(&self) {
        self.peripherals.USB.istr.write(|w| w.susp().clear_bit());

        // suspend and low power mode
        self.peripherals
            .USB
            .cntr
            .write(|w| w.fsusp().set_bit().lpmode().set_bit());

        set_device_state(DeviceState::Suspended);
    }

    fn wake_up(&self) {
        // Come out of low power mode.
        self.peripherals.USB.cntr.write(|w| w.lpmode().clear_bit());
        self.set_interrupt_mask();

        // clear interrupt flag
        self.peripherals.USB.istr.write(|w| w.wkup().clear_bit());

        set_device_state(DeviceState::WokenUp);
    }

    fn set_interrupt_mask(&self) {
        self.peripherals.USB.cntr.write(|w| {
            w.ctrm()
                .set_bit()
                .wkupm()
                .set_bit()
                .suspm()
                .set_bit()
                .errm()
                .set_bit()
                .esofm()
                .set_bit()
                .resetm()
                .set_bit()
                .pmaovrm()
                .set_bit()
        });
    }

    fn reset_buffer_table(&self) {
        self.pma.clear();

        // Configure 0 (control) endpoint
        self.pma.set_u16(0, 0x58 /* tx address */);
        self.pma.set_u16(2, 0x0);
        self.pma.set_u16(4, 0x18 /* rx address */);
        self.pma.set_u16(
            6,
            (0x8000 | (1 << 10)) as u16, /* 32 byte size, 1 block */
        );

        // Configure 1 (app) endpoint
        self.pma.set_u16(8, 0x98);
        self.pma.set_u16(10, 0x0);
        self.pma.set_u16(12, 0xD8);
        self.pma.set_u16(14, (0x8000 | (1 << 10)) as u16);
    }

    fn set_address(&self, address: u8) {
        if address == 0 {
            self.peripherals.USB.daddr.write(|w| w.ef().set_bit());
        } else {
            self.peripherals
                .USB
                .daddr
                .write(|w| unsafe { w.add().bits(address) });
        }
    }

    fn open_control_endpoints(&self) {
        let endpoint = Endpoint::Endpoint0(&self.peripherals.USB.ep0r);

        self.open_tx_endpoint(
            &endpoint,
            0b0,
            EndpointType::Control,
            // NAK the TX endpoint (nothing to go yet)
            EndpointStatus::Nak,
        );

        self.open_rx_endpoint(
            &endpoint,
            0b0,
            EndpointType::Control,
            // Initiate reception of the first packet.
            EndpointStatus::Valid,
        );
    }

    fn open_device_endpoints(&self) {
        let endpoint = Endpoint::Endpoint1(&self.peripherals.USB.ep1r);

        self.open_tx_endpoint(
            &endpoint,
            0b1,
            EndpointType::Interrupt,
            // NAK the TX endpoint (nothing to go yet)
            EndpointStatus::Nak,
        );

        self.open_rx_endpoint(
            &endpoint,
            0b1,
            EndpointType::Interrupt,
            // Initiate reception of the first packet.
            EndpointStatus::Valid,
        );
    }

    fn open_rx_endpoint(
        &self,
        endpoint: &Endpoint,
        address: u8,
        endpoint_type: EndpointType,
        status: EndpointStatus,
    ) {
        // Set up the endpoint type and address.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| {
                    unsafe {
                        w.ep_type().bits(endpoint_type as u8).ea().bits(address);
                    }

                    // if DTOG_RX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_rx().bit_is_set() {
                        w.dtog_rx().set_bit()
                    } else {
                        w
                    }
                })
            }
            Endpoint::Endpoint1(e) => {
                e.modify(|r, w| {
                    unsafe {
                        w.ep_type().bits(endpoint_type as u8).ea().bits(address);
                    }

                    // if DTOG_RX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_rx().bit_is_set() {
                        w.dtog_rx().set_bit()
                    } else {
                        w
                    }
                })
            }
        }

        self.set_rx_endpoint_status(endpoint, status);
    }

    fn open_tx_endpoint(
        &self,
        endpoint: &Endpoint,
        address: u8,
        endpoint_type: EndpointType,
        status: EndpointStatus,
    ) {
        // Set up the endpoint type and address.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| {
                    unsafe {
                        w.ep_type().bits(endpoint_type as u8).ea().bits(address);
                    }

                    // if DTOG_TX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_tx().bit_is_set() {
                        w.dtog_tx().set_bit()
                    } else {
                        w
                    }
                })
            }
            Endpoint::Endpoint1(e) => {
                e.modify(|r, w| {
                    unsafe {
                        w.ep_type().bits(endpoint_type as u8).ea().bits(address);
                    }

                    // if DTOG_TX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_tx().bit_is_set() {
                        w.dtog_tx().set_bit()
                    } else {
                        w
                    }
                })
            }
        }

        self.set_tx_endpoint_status(endpoint, status);
    }

    fn set_rx_endpoint_status(&self, endpoint: &Endpoint, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| unsafe { w.stat_rx().bits(r.stat_rx().bits() ^ status as u8) })
            }
            Endpoint::Endpoint1(e) => {
                e.modify(|r, w| unsafe { w.stat_rx().bits(r.stat_rx().bits() ^ status as u8) })
            }
        }
    }

    fn set_tx_endpoint_status(&self, endpoint: &Endpoint, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| unsafe { w.stat_tx().bits(r.stat_tx().bits() ^ status as u8) })
            }
            Endpoint::Endpoint1(e) => {
                e.modify(|r, w| unsafe { w.stat_tx().bits(r.stat_tx().bits() ^ status as u8) })
            }
        }
    }
}
