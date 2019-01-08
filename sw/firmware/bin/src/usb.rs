pub mod pma;

use core::ptr::read_volatile;
use cortex_m::{asm, Peripherals as CorePeripherals};
use stm32f0x2::{Interrupt, Peripherals};

use pma::PacketMemoryArea;

pub const LANG_ID_DESCRIPTOR: [u8; 4] = [
    0x04, 0x03, //
    0x09, 0x04, // English - US
];

pub const MANUFACTURER_STR: [u8; 38] = [
    0x26, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //
    0x4d, 0x00, // M
    0x61, 0x00, // a
    0x6e, 0x00, // n
    0x75, 0x00, // u
    0x66, 0x00, // f
    0x61, 0x00, // a
    0x63, 0x00, // c
    0x74, 0x00, // t
    0x75, 0x00, // u
    0x72, 0x00, // r
    0x65, 0x00, // e
    0x72, 0x00, // r
];

pub const PRODUCT_STR: [u8; 28] = [
    0x1c, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //
    0x50, 0x00, // P
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x64, 0x00, // d
    0x75, 0x00, // u
    0x63, 0x00, // c
    0x74, 0x00, // t
];

pub const SERIAL_NUMBER_STR: [u8; 14] = [
    0x0e, 0x03, //
    0x31, 0x00, // 1
    0x32, 0x00, // 2
    0x33, 0x00, // 3
    0x41, 0x00, // A
    0x42, 0x00, // B
    0x43, 0x00, // C
];

pub const CONF_STR: [u8; 40] = [
    0x28, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //
    0x43, 0x00, // C
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x66, 0x00, // f
    0x69, 0x00, // i
    0x67, 0x00, // g
    0x75, 0x00, // u
    0x72, 0x00, // r
    0x61, 0x00, // a
    0x74, 0x00, // t
    0x69, 0x00, // i
    0x6f, 0x00, // o
    0x6e, 0x00, // n
];

pub const INTERFACE_STR: [u8; 32] = [
    0x20, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //
    0x49, 0x00, // I
    0x6e, 0x00, // n
    0x74, 0x00, // t
    0x65, 0x00, // e
    0x72, 0x00, // r
    0x66, 0x00, // f
    0x61, 0x00, // a
    0x63, 0x00, // c
    0x65, 0x00, // e
];

pub const DEV_DESC: [u8; 18] = [
    0x12, // bLength
    0x01, // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0x00, // bDeviceClass (Use class information in the Interface Descriptors)
    0x00, // bDeviceSubClass
    0x00, // bDeviceProtocol
    0x40, // bMaxPacketSize0 64
    0xFF, 0xFF, // idVendor 0xFFFF
    0xFF, 0xFF, // idProduct 0xFFFF
    0x01, 0x00, // bcdDevice 0.01
    0x01, // iManufacturer (String Index)
    0x02, // iProduct (String Index)
    0x03, // iSerialNumber (String Index)
    0x01, // bNumConfigurations 1
];

pub const CONF_DESC: [u8; 41] = [
    0x09, // bLength
    0x02, // bDescriptorType (Configuration)
    0x29, 0x00, // wTotalLength
    0x01, // bNumInterfaces
    0x01, // bConfigurationValue
    0x04, // iConfiguration (String Index)
    0x80, // bmAttributes
    0xFA, // bMaxPower 500mA
    0x09, // bLength
    0x04, // bDescriptorType (Interface)
    0x00, // bInterfaceNumber 0
    0x00, // bAlternateSetting
    0x02, // bNumEndpoints 2
    0x03, // bInterfaceClass
    0x00, // bInterfaceSubClass 1=BOOT, 0=no boot
    0x00, // bInterfaceProtocol 0=none, 1=keyboard, 2=mouse
    0x00, // iInterface (String Index)
    // HID descriptor
    0x09, // bLength
    0x21, // bDescriptorType (HID)
    0x11, 0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x20, 0x00, // wDescriptorLength[0] 32
    // IN endpoint descriptor
    0x07, // bLength
    0x05, // bDescriptorType (Endpoint)
    0x81, // bEndpointAddress (IN/D2H)
    0x03, // bmAttributes (Interrupt)
    0x40, 0x00, // wMaxPacketSize 64
    0x20, // bInterval 1 (unit depends on device speed)
    // OUT endpoint descriptor
    0x07, // bLength
    0x05, // bDescriptorType (Endpoint)
    0x01, // bEndpointAddress (OUT/H2D)
    0x03, // bmAttributes (Interrupt)
    0x40, 0x00, // wMaxPacketSize 64
    0x20, // bInterval 1 (unit depends on device speed)
];

// The HID descriptor (this is a copy of the descriptor embedded in the above configuration descriptor.
pub const HID_DESC: [u8; 9] = [
    0x09, // bLength: CUSTOM_HID Descriptor size
    0x21, // bDescriptorType (HID)
    0x11, 0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x20, 0x00, // wDescriptorLength[0] 32
];

pub const REPORT_DESC: [u8; 32] = [
    0x05, 0x01, // USAGE_PAGE (Generic Desktop)
    0x09, 0x00, // USAGE (Undefined)
    0xa1, 0x01, // COLLECTION (Application)
    0x15, 0x00, //   LOGICAL_MINIMUM (0)
    0x26, 0xff, 0x00, //   LOGICAL_MAXIMUM (255)
    // IN report
    0x85, 0x01, //   REPORT_ID (1)
    0x75, 0x08, //   REPORT_SIZE (8)
    0x95, 0x3f, // REPORT_COUNT (this is the byte length)
    0x09, 0x00, //   USAGE (Undefined)
    0x81, 0x82, //   INPUT (Data,Var,Abs,Vol)
    // OUT report
    0x85, 0x02, //   REPORT_ID (2)
    0x75, 0x08, //   REPORT_SIZE (8)
    0x95, 0x3f, // REPORT_COUNT (this is the byte length)
    0x09, 0x00, //   USAGE (Undefined)
    0x91, 0x82, //   OUTPUT (Data,Var,Abs,Vol)
    0xc0, // END_COLLECTION
];

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

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum UsbRequest {
    GetStatus = 0x00,
    ClearFeature = 0x01,
    Two = 0x02,
    SetFeature = 0x03,
    SetAddress = 0x05,
    GetDescriptor = 0x06,
    SetDescriptor = 0x07,
    GetConfiguration = 0x08,
    SetConfiguration = 0x09,
    GetInterface = 0x0A,
    SetInterface = 0x0B,
    SynchFrame = 0x0C,
}

struct UsbRequestHeader {
    request: UsbRequest,
    dir: UsbRequestDirection,
    kind: UsbRequestKind,
    recipient: UsbRequestRecipient,
    value: u16,
    index: u16,
    length: u16,
}

impl From<(u16, u16, u16, u16)> for UsbRequestHeader {
    #[inline]
    fn from((request_header, value, index, data_length): (u16, u16, u16, u16)) -> Self {
        let request_type = (request_header & 0x00ff) as u8;
        let request = ((request_header & 0xff00) >> 8) as u8;

        UsbRequestHeader {
            request: match request {
                0x00 => UsbRequest::GetStatus,
                0x01 => UsbRequest::ClearFeature,
                0x02 => UsbRequest::Two,
                0x03 => UsbRequest::SetFeature,
                0x05 => UsbRequest::SetAddress,
                0x06 => UsbRequest::GetDescriptor,
                0x07 => UsbRequest::SetDescriptor,
                0x08 => UsbRequest::GetConfiguration,
                0x09 => UsbRequest::SetConfiguration,
                0x0A => UsbRequest::GetInterface,
                0x0B => UsbRequest::SetInterface,
                0x0C => UsbRequest::SynchFrame,
                _ => panic!("Unreachable"),
            },
            // Bit 7
            dir: match request_type & 0x80 {
                0x00 => UsbRequestDirection::HostToDevice,
                0x80 => UsbRequestDirection::DeviceToHost,
                _ => panic!("Unreachable"),
            },
            // Bits 6:5
            kind: match request_type & 0x60 {
                0x00 => UsbRequestKind::Standard,
                0x20 => UsbRequestKind::Class,
                0x40 => UsbRequestKind::Vendor,
                0x60 => UsbRequestKind::Reserved,
                _ => panic!("Unreachable"),
            },
            // Bits 4:0
            recipient: match request_type & 0x1f {
                0x00 => UsbRequestRecipient::Device,
                0x01 => UsbRequestRecipient::Interface,
                0x02 => UsbRequestRecipient::Endpoint,
                0x03 => UsbRequestRecipient::Other,
                0x04...0x1f => UsbRequestRecipient::Reserved,
                _ => panic!("Unreachable"),
            },
            value,
            index,
            length: data_length,
        }
    }
}

enum Endpoint<'a> {
    Endpoint0(&'a stm32f0x2::usb::EP0R),
    Endpoint1(&'a stm32f0x2::usb::EP1R),
}

#[derive(Copy, Clone)]
enum EndpointType {
    Control = 0b0,
    Device = 0b1,
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
    Setup(u16),
    DataIn,
    DataOut,
    StatusIn,
    StatusOut,
    Stall,
}

#[derive(Copy, Clone)]
pub struct UsbState {
    device_state: DeviceState,
    suspended_device_state: Option<DeviceState>,
    control_endpoint_state: ControlEndpointState,
    setup_data_length: u16,
    address: u8,
    configuration_index: u8,
    protocol: u8,
    idle_state: u8,
    alt_setting: u8,
}

impl Default for UsbState {
    fn default() -> Self {
        UsbState {
            device_state: DeviceState::None,
            suspended_device_state: None,
            control_endpoint_state: ControlEndpointState::Idle,
            setup_data_length: 0,
            address: 0,
            configuration_index: 0,
            protocol: 0,
            idle_state: 0,
            alt_setting: 0,
        }
    }
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
    pma: &'a PacketMemoryArea,
    state: &'a mut UsbState,
}

impl<'a> USB<'a> {
    pub fn new(
        core_peripherals: &'a mut CorePeripherals,
        peripherals: &'a Peripherals,
        state: &'a mut UsbState,
        pma: &'a PacketMemoryArea,
    ) -> USB<'a> {
        USB {
            core_peripherals,
            peripherals,
            pma,
            state,
        }
    }

    pub fn acquire<'b, F>(
        core_peripherals: &'b mut CorePeripherals,
        peripherals: &'b Peripherals,
        state: &'b mut UsbState,
        pma: &'b PacketMemoryArea,
        f: F,
    ) -> ()
    where
        F: FnOnce(USB),
    {
        f(USB::new(core_peripherals, peripherals, state, pma));
    }

    pub fn start(&mut self) {
        self.state.address = 0;

        self.update_device_state(DeviceState::Default);

        self.peripherals
            .RCC
            .apb1enr
            .modify(|_, w| w.usben().set_bit());
        self.core_peripherals.NVIC.enable(Interrupt::USB);

        // Reset the peripheral.
        self.peripherals
            .USB
            .cntr
            .modify(|_, w| w.pdwn().clear_bit().fres().set_bit());
        self.peripherals
            .USB
            .cntr
            .modify(|_, w| w.fres().clear_bit());

        // Reset any pending interrupts.
        self.peripherals.USB.istr.reset();

        self.set_interrupt_mask();

        self.pma.init();
        /*self.set_tx_addr(CONTROL_IN_PMA_ADDRESS);
        self.set_tx_count(0);
        self.set_rx_addr(CONTROL_OUT_PMA_ADDRESS);
        self.set_rx_count(0);*/

        self.peripherals.USB.bcdr.modify(|_, w| w.dppu().set_bit());
    }

    pub fn stop(&mut self) {
        self.close_device_endpoints();
        self.close_control_endpoints();

        self.core_peripherals.NVIC.disable(Interrupt::USB);

        // Tell the host that we're gone by disabling pull-up on DP.
        self.peripherals
            .USB
            .bcdr
            .modify(|_, w| w.dppu().clear_bit());

        // USB clock off.
        self.peripherals
            .RCC
            .apb1enr
            .modify(|_, w| w.usben().clear_bit());
    }

    pub fn interrupt(&mut self) {
        if self.peripherals.USB.istr.read().reset().bit_is_set() {
            self.reset();
        }

        if self.peripherals.USB.istr.read().err().bit_is_set() {
            self.peripherals
                .USB
                .istr
                .write(|w| unsafe { w.bits(0xDFFF) });
        }

        /*if istr.susp().bit_is_set() {
            self.suspend();
        }

        if istr.wkup().bit_is_set() {
            self.wake_up();
        }*/

        // Clear SUSP, SOF and ESOF
        self.peripherals
            .USB
            .istr
            .write(|w| unsafe { w.bits(0xF4FF) });

        // Correct endpoint transfer
        if self.peripherals.USB.istr.read().ctr().bit_is_set() {
            /*let istr = self.peripherals.USB.istr.read().bits();
            let cntr = self.peripherals.USB.cntr.read().bits();
            let base_address = CONTROL_OUT_PMA_ADDRESS as usize;
            let setup_packet = [
                self.pma.get_u16(base_address),
                self.pma.get_u16(base_address + 2),
                self.pma.get_u16(base_address + 4),
                self.pma.get_u16(base_address + 6),
            ];

            let btable = [
                self.pma.get_u16(0),
                self.pma.get_u16(2),
                self.pma.get_u16(4),
                self.pma.get_u16(6),
            ];

            if setup_packet.iter().any(|&x| x > 0) && btable.iter().any(|&x| x > 0) && istr > 0 && cntr > 0 {
                self.blue_on();
            }

            asm::bkpt();*/
            self.correct_transfer();
        }
    }

    fn reset(&mut self) {
        self.peripherals
            .USB
            .istr
            .write(|w| unsafe { w.bits(0xFBFF) });

        self.update_address(0);
        self.open_control_endpoints();
    }

    #[no_mangle]
    fn read(&self, address: u32) -> u16 {
        return unsafe { read_volatile(address as *mut u16) };
    }

    #[no_mangle]
    fn read_u32(&self, address: u32) -> u32 {
        return unsafe { read_volatile(address as *mut u32) };
    }

    fn correct_transfer(&mut self) {
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
                /* 1 => {
                    let ep1 = self.peripherals.USB.ep1r.read();
                    if is_out && ep1.ctr_rx().bit_is_set() {
                        self.handle_device_out_transfer();
                    } else if !is_out && ep1.ctr_tx().bit_is_set() {
                        self.handle_device_in_transfer();
                    }
                }*/
                _ => panic!("Unknown endpoint"),
            }
        }
    }

    fn handle_control_out_transfer(&mut self) {
        if self.peripherals.USB.ep0r.read().setup().bit_is_set() {
            self.handle_control_setup_out_transfer();
        } else if self.peripherals.USB.ep0r.read().ctr_rx().bit_is_set() {
            self.handle_control_data_out_transfer();
        }
    }

    fn handle_control_setup_out_transfer(&mut self) {
        /*  let req: u16 = self.pma.get_u16(base_address);*/
        /*  let val = self.pma.get_u16(base_address + 2);
        let index = self.pma.get_u16(base_address + 4);
        let len = self.pma.get_u16(base_address + 6);*/

        /*  let comparator: u16 = 0x0001;
        let mask: u16 = 0x00FF;
        let full: u16 = 0x0001;*/
        /* if req != 0x0000 {
            self.blue_on();
        } else {
            self.red_on();
        }*/

        /*let count_mask: u16 = 0xfc00;
        let count_tx: u16 = self.pma.get_u16(2);*/
        //let count_rx: u16 = self.pma.get_u16(6) & 0x03ff;
        /*  let comp: u16 = 0x8400;
        let ref_count: u16 = 0x0000;*/
        /*if count_rx != 0x0000 {
            self.green_on();
        }*/

        /*let req = self.pma.get_u16(CONTROL_IN_PMA_ADDRESS as usize);
        let val = self.pma.get_u16(CONTROL_IN_PMA_ADDRESS as usize + 2);
        let index = self.pma.get_u16(CONTROL_IN_PMA_ADDRESS as usize + 4);
        let len = self.pma.get_u16(CONTROL_IN_PMA_ADDRESS as usize + 6);

        if req != 0 || val != 0 || index != 0 || len != 0 {
            self.blue_on();
        } else {
            self.red_on();
        }0x1f*/

        /*  let tx_add = self.pma.get_u16(0);
        let raw_tx_add = self.read(0x40006000);

        let tx_count = self.pma.get_u16(2);
        let raw_tx_count = self.read(0x40006000 + 4);

        let rx_add = self.pma.get_u16(4);
        let raw_rx_add = self.read(0x40006000 + 8);

        let rx_count = self.pma.get_u16(6);
        let raw_rx_count = self.read(0x40006000 + 12);*/

        /* let base_add_1 = self.pma.get_u16(CONTROL_IN_PMA_ADDRESS as usize);
        let base_add_2 = self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS as usize);*/

        /* let arr = [
            self.pma.get_u16(0),
            self.pma.get_u16(2),
            self.pma.get_u16(4),
            self.pma.get_u16(6),
            self.pma.get_u16(CONTROL_OUT_PMA_ADDRESS as usize),
            self.pma.get_u16((CONTROL_OUT_PMA_ADDRESS + 2) as usize),
            self.pma.get_u16((CONTROL_OUT_PMA_ADDRESS + 4) as usize),
            self.pma.get_u16((CONTROL_OUT_PMA_ADDRESS + 6) as usize),
        ];

        let arr2 = [
            self.pma.get_u16_old(0 / 2),
            self.pma.get_u16_old(2 / 2),
            self.pma.get_u16_old(4 / 2),
            self.pma.get_u16_old(6 / 2),
            self.pma.get_u16_old((CONTROL_OUT_PMA_ADDRESS / 2) as usize),
            self.pma.get_u16_old(((CONTROL_OUT_PMA_ADDRESS + 2) / 2) as usize),
            self.pma.get_u16_old(((CONTROL_OUT_PMA_ADDRESS + 4) / 2) as usize),
            self.pma.get_u16_old(((CONTROL_OUT_PMA_ADDRESS + 6) / 2) as usize),
        ];

        self.blue_on();
        asm::bkpt();
        if arr.iter().all(|&x| x > 0) && arr2.iter().all(|&x| x > 0) {
            self.green_on();
        }*/

        /*asm::bkpt();

        if setup_packet.iter().any(|&x| x > 0) && btable.iter().any(|&x| x > 0) {
            self.blue_on();
        }*/

        /* if setup_packet[1] > 0 && setup_packet.iter().any(|&x| x == 0x0680 || x == 0x8060) {
            asm::bkpt();
        }*/

        let btable = [
            self.pma.get_tx_addr(EndpointType::Control),
            self.pma.get_tx_count(EndpointType::Control),
            self.pma.get_rx_addr(EndpointType::Control),
            self.pma.get_rx_count(EndpointType::Control),
        ];

        let setup_packet = [
            self.pma.read(EndpointType::Control, 0),
            self.pma.read(EndpointType::Control, 2),
            self.pma.read(EndpointType::Control, 4),
            self.pma.read(EndpointType::Control, 6),
        ];

        /*   let c_setup_packet = [
            self.read(0x40006000),
            self.read(0x40006002),
            self.read(0x40006004),
            self.read(0x40006006)
        ];*/

        let setup_packet_length = btable[3];

        let header = UsbRequestHeader::from((
            setup_packet[0],
            setup_packet[1],
            setup_packet[2],
            setup_packet[3],
        ));

        /* asm::bkpt();

        if setup_packet.iter().any(|&x| x > 0) && btable.iter().any(|&x| x > 0) && c_setup_packet.iter().any(|&x| x > 0) {
            self.blue_on();
        }*/

        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.modify(|_, w| unsafe {
            w.ctr_rx()
                .clear_bit()
                .ctr_tx()
                .set_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(0b00)
                .stat_rx()
                .bits(0b00)
        });
        self.update_control_endpoint_state(ControlEndpointState::Setup(setup_packet_length));

        match header.recipient {
            UsbRequestRecipient::Device => self.handle_device_request(header),
            UsbRequestRecipient::Interface => self.handle_interface_request(header),
            UsbRequestRecipient::Endpoint => self.handle_endpoint_request(header),
            _ => self.set_rx_endpoint_status(&Endpoint::Endpoint0(endpoint), EndpointStatus::Stall),
        }
    }

    fn handle_control_data_out_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.modify(|_, w| unsafe {
            w.ctr_rx()
                .clear_bit()
                .ctr_tx()
                .set_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(0b00)
                .stat_rx()
                .bits(0b00)
        });

        // Here we can check the amount of data and do smth with it....

        self.pma.set_rx_count(EndpointType::Control, 0);

        self.set_rx_endpoint_status(&Endpoint::Endpoint0(endpoint), EndpointStatus::Valid);
    }

    fn handle_control_in_transfer(&mut self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.modify(|_, w| unsafe {
            w.ctr_tx()
                .clear_bit()
                .ctr_rx()
                .set_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(0b00)
                .stat_rx()
                .bits(0b00)
        });

        if let ControlEndpointState::DataIn = self.state.control_endpoint_state {
            self.update_control_endpoint_state(ControlEndpointState::DataOut);

            // Prepare for premature end of transfer.
            self.pma.set_rx_count(EndpointType::Control, 0);
            self.set_rx_endpoint_status(&Endpoint::Endpoint0(&endpoint), EndpointStatus::Valid);
        }

        if self.state.address > 0 {
            self.peripherals
                .USB
                .daddr
                .write(|w| unsafe { w.add().bits(self.state.address).ef().set_bit() });
            self.state.address = 0;
        }
    }

    fn handle_device_request(&mut self, request_header: UsbRequestHeader) {
        match request_header.request {
            UsbRequest::GetDescriptor => self.handle_get_descriptor(request_header),
            UsbRequest::SetAddress => self.handle_set_address(request_header),
            UsbRequest::SetConfiguration => self.handle_set_configuration(request_header),
            UsbRequest::GetConfiguration => self.handle_get_configuration(request_header),
            UsbRequest::GetStatus => self.handle_get_status(),
            UsbRequest::SetFeature => self.handle_set_feature(request_header),
            UsbRequest::ClearFeature => self.handle_clear_feature(request_header),
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_get_descriptor(&mut self, request_header: UsbRequestHeader) {
        let data_to_send: Option<&[u8]> = match (&request_header.value >> 8) as u16 {
            1 => Some(&DEV_DESC),
            2 => Some(&CONF_DESC),
            3 => self.get_descriptor_string(&request_header),
            _ => None,
        };

        if let Some(data) = data_to_send {
            let data_length = data.len();
            if request_header.length > 0 && data_length > 0 {
                // Send the data to the host.
                let data_to_send_length = if data_length <= request_header.length as usize {
                    data_length
                } else {
                    request_header.length as usize
                };
                self.send_control_data(Some(&data[..data_to_send_length]));
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn get_descriptor_string(&self, request_header: &UsbRequestHeader) -> Option<&'static [u8]> {
        match request_header.value & 0xff {
            0x00 => Some(&LANG_ID_DESCRIPTOR),
            0x01 => Some(&MANUFACTURER_STR),
            0x02 => Some(&PRODUCT_STR),
            0x03 => Some(&SERIAL_NUMBER_STR),
            0x04 => Some(&CONF_STR),
            0x05 => Some(&INTERFACE_STR),
            _ => None,
        }
    }

    fn handle_set_address(&mut self, request_header: UsbRequestHeader) {
        if request_header.index == 0 && request_header.length == 0 {
            if let DeviceState::Configured = self.state.device_state {
                self.control_endpoint_error();
            } else {
                let address = (request_header.value & 0x7F) as u8;
                self.update_address(address);
                self.send_control_zero_length_packet();
                self.update_device_state(if address != 0 {
                    DeviceState::Addressed
                } else {
                    DeviceState::Default
                });
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn handle_set_configuration(&mut self, request_header: UsbRequestHeader) {
        let configuration_index = request_header.value as u8;

        self.state.configuration_index = configuration_index;

        if configuration_index > 1 {
            self.control_endpoint_error();
        } else {
            match self.state.device_state {
                DeviceState::Addressed => {
                    if configuration_index != 0 {
                        self.open_device_endpoints();
                        self.send_control_zero_length_packet();
                        self.update_device_state(DeviceState::Configured);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                DeviceState::Configured => {
                    if configuration_index == 0 {
                        self.close_control_endpoints();
                        self.send_control_zero_length_packet();
                        self.update_device_state(DeviceState::Addressed);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_configuration(&mut self, request_header: UsbRequestHeader) {
        if request_header.length != 1 {
            self.control_endpoint_error();
        } else {
            match self.state.device_state {
                DeviceState::Addressed => {
                    self.state.configuration_index = 0;
                    self.send_control_data(Some([0].as_ref()));
                }
                DeviceState::Configured => {
                    self.send_control_data(Some([self.state.configuration_index].as_ref()));
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_status(&mut self) {
        match self.state.device_state {
            DeviceState::Addressed | DeviceState::Configured => {
                self.send_control_data(Some([3].as_ref()))
            }
            _ => {}
        }
    }

    fn handle_set_feature(&mut self, request_header: UsbRequestHeader) {
        if request_header.value == 1 {
            // ACK
            self.send_control_zero_length_packet();
        }
    }

    fn handle_clear_feature(&mut self, request_header: UsbRequestHeader) {
        match self.state.device_state {
            DeviceState::Addressed | DeviceState::Configured => {
                if request_header.value == 1 {
                    // ACK
                    self.send_control_zero_length_packet();
                }
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_interface_request(&mut self, request_header: UsbRequestHeader) {
        match self.state.device_state {
            DeviceState::Configured if (request_header.index & 0xff) <= 1 => {
                self.handle_setup(request_header);
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_setup(&mut self, request_header: UsbRequestHeader) {
        match request_header.kind {
            UsbRequestKind::Class => self.handle_class_setup(request_header),
            UsbRequestKind::Standard => self.handle_standard_setup(request_header),
            _ => {}
        }
    }

    fn handle_class_setup(&mut self, request_header: UsbRequestHeader) {
        match request_header.request {
            // CUSTOM_HID_REQ_SET_PROTOCOL
            UsbRequest::SetInterface => {
                self.state.protocol = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_PROTOCOL
            UsbRequest::SetFeature => self.send_control_data(Some([self.state.protocol].as_ref())),
            // CUSTOM_HID_REQ_SET_IDLE
            UsbRequest::GetInterface => {
                self.state.idle_state = (request_header.value >> 8) as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_IDLE
            UsbRequest::Two => self.send_control_data(Some([self.state.idle_state].as_ref())),
            // CUSTOM_HID_REQ_SET_REPORT
            UsbRequest::SetConfiguration => {
                self.update_control_endpoint_state(ControlEndpointState::DataOut);
                self.pma
                    .set_rx_count(EndpointType::Control, request_header.length);
                self.set_rx_endpoint_status(
                    &Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
                    EndpointStatus::Valid,
                );
                self.send_control_zero_length_packet();
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_standard_setup(&mut self, request_header: UsbRequestHeader) {
        match request_header.request {
            UsbRequest::GetDescriptor => {
                let data = match request_header.value >> 8 {
                    // USB_DESC_TYPE_HID_REPORT
                    0x22 => Some(if (request_header.length as usize) < REPORT_DESC.len() {
                        &REPORT_DESC[..request_header.length as usize]
                    } else {
                        &REPORT_DESC
                    }),
                    // USB_DESC_TYPE_HID_DESCRIPTOR
                    0x21 => Some(if (request_header.length as usize) < HID_DESC.len() {
                        &HID_DESC[..request_header.length as usize]
                    } else {
                        &HID_DESC
                    }),
                    _ => None,
                };

                self.send_control_data(data);
            }
            UsbRequest::SetInterface => {
                self.state.alt_setting = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            UsbRequest::GetInterface => {
                self.send_control_data(Some([self.state.alt_setting].as_ref()))
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_endpoint_request(&mut self, request_header: UsbRequestHeader) {
        if let UsbRequestKind::Class = request_header.kind {
            self.handle_setup(request_header);
            return;
        }

        let endpoint_address = request_header.index as u8;
        match request_header.request {
            UsbRequest::SetFeature => {
                match self.state.device_state {
                    DeviceState::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceState::Configured => {
                        // USB_FEATURE_EP_HALT
                        if request_header.value == 0 && endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }

                        self.handle_setup(request_header);
                        self.send_control_zero_length_packet();
                    }
                    _ => self.control_endpoint_error(),
                }
            }
            UsbRequest::ClearFeature => {
                match self.state.device_state {
                    DeviceState::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceState::Configured => {
                        // USB_FEATURE_EP_HALT
                        if request_header.value == 0 && endpoint_address & 0x7f != 0 {
                            self.unstall_endpoint(endpoint_address);
                            self.handle_setup(request_header);
                        }
                    }
                    _ => self.control_endpoint_error(),
                }
            }
            UsbRequest::GetStatus => {
                match self.state.device_state {
                    DeviceState::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceState::Configured => {
                        // SHOULD BE:  status=isStalled(ep_addr) ? 1 : 0; sendControlData(&status,2);
                        self.send_control_data(Some([0x0, 0x0].as_ref()));
                    }
                    _ => self.control_endpoint_error(),
                }
            }
            _ => {}
        }
    }

    fn handle_device_out_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep1r;
        endpoint.modify(|_, w| unsafe {
            w.ctr_rx()
                .clear_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(0b00)
                .stat_rx()
                .bits(0b00)
        });

        // Here we can check the amount of data and do smth with it....

        self.pma.set_rx_count(EndpointType::Device, 0);

        self.set_rx_endpoint_status(&Endpoint::Endpoint1(endpoint), EndpointStatus::Valid);
    }

    fn handle_device_in_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        self.peripherals.USB.ep1r.modify(|_, w| unsafe {
            w.ctr_tx()
                .clear_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
                .stat_tx()
                .bits(0b00)
                .stat_rx()
                .bits(0b00)
        });
    }

    fn stall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        if endpoint_index == 0 {
            self.control_endpoint_error();
        } else {
            let endpoint = match endpoint_index {
                0 => Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
                1 => Endpoint::Endpoint1(&self.peripherals.USB.ep1r),
                _ => panic!("AAA"),
            };

            if endpoint_address & 0x80 == 0x80 {
                self.set_tx_endpoint_status(&endpoint, EndpointStatus::Stall);
            } else {
                self.set_rx_endpoint_status(&endpoint, EndpointStatus::Stall);
            }
        }
    }

    fn unstall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        let endpoint = match endpoint_index {
            0 => Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
            1 => Endpoint::Endpoint1(&self.peripherals.USB.ep1r),
            _ => panic!("BBBB"),
        };

        if endpoint_index == 0 || endpoint_address & 0x80 == 0x80 {
            self.set_tx_endpoint_status(&endpoint, EndpointStatus::Stall);
        } else if endpoint_address & 0x80 == 0x0 {
            self.set_rx_endpoint_status(&endpoint, EndpointStatus::Stall);
        }
    }

    fn control_endpoint_error(&self) {
        self.peripherals.USB.ep0r.modify(|r, w| unsafe {
            w.stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Stall))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Stall))
                .ctr_rx()
                .set_bit()
                .ctr_tx()
                .set_bit()
                .dtog_tx()
                .clear_bit()
                .dtog_rx()
                .clear_bit()
        });
    }

    fn send_control_data(&mut self, data: Option<&[u8]>) {
        self.update_control_endpoint_state(ControlEndpointState::DataIn);
        self.send_data(EndpointType::Control, data);
    }

    fn send_control_zero_length_packet(&mut self) {
        self.update_control_endpoint_state(ControlEndpointState::StatusIn);
        self.send_data(EndpointType::Control, None);
    }

    fn send_data(&self, endpoint_type: EndpointType, data: Option<&[u8]>) {
        let length = data
            .and_then(|d| {
                self.pma.write(endpoint_type, &d);
                Some(d)
            })
            .map_or(0, |d| d.len() as u16);

        // Now that the PMA memory is prepared, set the length and tell the peripheral to send it.
        let endpoint = match endpoint_type {
            EndpointType::Control => Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
            EndpointType::Device => Endpoint::Endpoint1(&self.peripherals.USB.ep1r),
        };

        self.pma.set_tx_count(endpoint_type, length);
        self.set_tx_endpoint_status(&endpoint, EndpointStatus::Valid);
    }

    fn suspend(&mut self) {
        self.peripherals
            .USB
            .istr
            .modify(|_, w| w.susp().clear_bit());

        // suspend and low power mode
        self.peripherals
            .USB
            .cntr
            .modify(|_, w| w.fsusp().set_bit().lpmode().set_bit());

        self.update_device_state(DeviceState::Suspended);
    }

    fn wake_up(&mut self) {
        // Come out of low power mode.
        self.peripherals
            .USB
            .cntr
            .modify(|_, w| w.lpmode().clear_bit());
        self.set_interrupt_mask();

        // clear interrupt flag
        self.peripherals
            .USB
            .istr
            .modify(|_, w| w.wkup().clear_bit());

        self.update_device_state(DeviceState::WokenUp);
    }

    fn set_interrupt_mask(&self) {
        self.peripherals.USB.cntr.modify(|_, w| {
            w.ctrm()
                .set_bit()
                /*.wkupm()
                .set_bit()
                .suspm()
                .set_bit()*/
                .errm()
                .set_bit()
                /*.pmaovrm()
                .set_bit()*/
                .resetm()
                .set_bit()
        });
    }

    fn update_device_state(&mut self, device_state: DeviceState) {
        match (self.state.device_state, self.state.suspended_device_state) {
            (DeviceState::Suspended, _) => {
                self.state.device_state = device_state;
                self.state.suspended_device_state = Some(self.state.device_state);
            }
            (DeviceState::WokenUp, Some(previous_device_state)) => {
                self.state.device_state = previous_device_state;
                self.state.suspended_device_state = None;
            }
            (DeviceState::WokenUp, None) => {}
            _ => self.state.device_state = device_state,
        }
    }

    fn update_control_endpoint_state(&mut self, control_endpoint_state: ControlEndpointState) {
        if let ControlEndpointState::Setup(data_length) = control_endpoint_state {
            self.state.setup_data_length = data_length;
        }

        self.state.control_endpoint_state = control_endpoint_state;
    }

    fn update_address(&mut self, address: u8) {
        if address == 0 {
            self.peripherals.USB.daddr.write(|w| w.ef().set_bit());
        }

        self.state.address = address;
    }

    fn open_control_endpoints(&self) {
        self.peripherals.USB.ep0r.write(|w| unsafe {
            w.ep_type()
                .bits(0b01)
                .ctr_rx()
                .set_bit()
                .ctr_tx()
                .set_bit()
                .stat_tx()
                .bits(self.get_status_bits(0, EndpointStatus::Nak))
                .stat_rx()
                .bits(self.get_status_bits(0, EndpointStatus::Valid))
            // If DTOG_RX is 1 then we need to write 1 to toggle it to zero.
            /* .dtog_rx()
            .bit(r.dtog_rx().bit_is_set())
            .dtog_tx()
            .bit(r.dtog_tx().bit_is_set())*/
        });
    }

    fn open_device_endpoints(&self) {
        self.peripherals.USB.ep1r.modify(|r, w| unsafe {
            w.ep_type()
                .bits(EndpointType::Device as u8)
                .ea()
                .bits(0x1)
                .stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Nak))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Valid))
                // If DTOG_RX is 1 then we need to write 1 to toggle it to zero.
                .dtog_rx()
                .bit(r.dtog_rx().bit_is_set())
                .dtog_tx()
                .bit(r.dtog_tx().bit_is_set())
        });
    }

    fn close_control_endpoints(&self) {
        self.peripherals.USB.ep0r.modify(|r, w| unsafe {
            let mut bits = w
                .stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Disabled));

            // If DTOG_RX is 1 then we need to write 1 to toggle it to zero.
            if r.dtog_rx().bit_is_set() {
                bits = bits.dtog_rx().set_bit();
            }

            if r.dtog_tx().bit_is_set() {
                bits.dtog_tx().set_bit()
            } else {
                bits
            }
        });
    }

    fn close_device_endpoints(&self) {
        self.peripherals.USB.ep1r.modify(|r, w| unsafe {
            let mut bits = w
                .stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Disabled));

            // If DTOG_RX is 1 then we need to write 1 to toggle it to zero.
            if r.dtog_rx().bit_is_set() {
                bits = bits.dtog_rx().set_bit();
            }

            if r.dtog_tx().bit_is_set() {
                bits.dtog_tx().set_bit()
            } else {
                bits
            }
        });
    }

    fn get_status_bits(&self, current_bits: u8, status: EndpointStatus) -> u8 {
        return current_bits ^ status as u8;
    }

    fn set_rx_endpoint_status(&self, endpoint: &Endpoint, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| unsafe {
                    w.stat_rx()
                        .bits(self.get_status_bits(r.stat_rx().bits(), status))
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
            Endpoint::Endpoint1(e) => e.modify(|r, w| unsafe {
                w.stat_rx()
                    .bits(self.get_status_bits(r.stat_rx().bits(), status))
                    .dtog_tx()
                    .clear_bit()
                    .dtog_rx()
                    .clear_bit()
                    .stat_tx()
                    .bits(0b00)
            }),
        }
    }

    fn set_tx_endpoint_status(&self, endpoint: &Endpoint, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| unsafe {
                    w.stat_tx()
                        .bits(self.get_status_bits(r.stat_tx().bits(), status))
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
            Endpoint::Endpoint1(e) => e.modify(|r, w| unsafe {
                w.stat_tx()
                    .bits(self.get_status_bits(r.stat_tx().bits(), status))
                    .dtog_tx()
                    .clear_bit()
                    .dtog_rx()
                    .clear_bit()
                    .stat_rx()
                    .bits(0b00)
            }),
        }
    }

    /* fn blue_off(&self) {
        self.peripherals.GPIOA.bsrr.write(|w| w.br2().set_bit());
    }*/

    fn blue_on(&self) {
        self.peripherals.GPIOA.bsrr.write(|w| w.bs2().set_bit());
    }

    pub fn red_on(&self) {
        self.peripherals.GPIOA.bsrr.write(|w| w.bs4().set_bit());
    }

    pub fn green_on(&self) {
        self.peripherals.GPIOA.bsrr.write(|w| w.bs3().set_bit());
    }

    /* fn green_off(&self) {
        self.peripherals.GPIOA.bsrr.write(|w| w.br3().set_bit());
    }*/
}
