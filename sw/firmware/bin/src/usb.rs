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
    Setup(u16),
    DataIn,
    DataOut,
    StatusIn,
    StatusOut,
    Stall,
}

#[derive(Copy, Clone)]
struct UsbState {
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

const CONTROL_OUT_PMA_ADDRESS: u16 = 0x18;
const CONTROL_IN_PMA_ADDRESS: u16 = 0x58;
const DEVICE_IN_PMA_ADDRESS: u16 = 0x98;
const DEVICE_OUT_PMA_ADDRESS: u16 = 0xD8;

static USB_STATE: Mutex<RefCell<UsbState>> = Mutex::new(RefCell::new(UsbState {
    device_state: DeviceState::None,
    suspended_device_state: None,
    control_endpoint_state: ControlEndpointState::Idle,
    setup_data_length: 0,
    address: 0,
    configuration_index: 0,
    protocol: 0,
    idle_state: 0,
    alt_setting: 0,
}));

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

    pub fn configure(peripherals: &Peripherals) {
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
    }

    pub fn start(&mut self) {
        self.set_address(0);
        self.set_device_state(DeviceState::Default);

        self.peripherals.RCC.apb1enr.write(|w| w.usbrst().set_bit());

        self.core_peripherals.NVIC.enable(Interrupt::USB);

        // Reset the peripheral.
        self.peripherals.USB.cntr.write(|w| w.fres().set_bit());
        self.peripherals.USB.cntr.write(|w| unsafe { w.bits(0b0) });

        // Reset any pending interrupts.
        self.peripherals.USB.istr.write(|w| unsafe { w.bits(0b0) });

        self.set_interrupt_mask();

        self.peripherals.USB.bcdr.write(|w| w.dppu().set_bit());
    }

    pub fn stop(&mut self) {
        self.close_device_endpoints();
        self.close_control_endpoints();

        self.core_peripherals.NVIC.disable(Interrupt::USB);

        // Tell the host that we're gone by disabling pull-up on DP.
        self.peripherals.USB.bcdr.write(|w| w.dppu().clear_bit());

        // USB clock off.
        self.peripherals
            .RCC
            .apb1enr
            .write(|w| w.usbrst().clear_bit());
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

    fn reset(&self) {
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
        let base_address = CONTROL_OUT_PMA_ADDRESS as usize;
        let header = UsbRequestHeader::from((
            self.pma.get_u16(base_address),
            self.pma.get_u16(base_address + 2),
            self.pma.get_u16(base_address + 4),
            self.pma.get_u16(base_address + 6),
        ));

        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.write(|w| w.ctr_rx().clear_bit());

        self.set_control_endpoint_state(ControlEndpointState::Setup(self.pma.get_u16(6) & 0x3ff));

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
        endpoint.write(|w| w.ctr_rx().clear_bit());

        // Here we can check the amount of data and do smth with it....

        self.pma
            .set_u16(6, 0x8000 | (1 << 10) /* 32 byte size, 1 block */);

        self.set_rx_endpoint_status(&Endpoint::Endpoint0(endpoint), EndpointStatus::Valid);
    }

    fn handle_control_in_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep0r;
        endpoint.write(|w| w.ctr_tx().clear_bit());

        if let ControlEndpointState::DataIn = self.get_control_endpoint_state() {
            self.set_control_endpoint_state(ControlEndpointState::DataOut);

            // Prepare for premature end of transfer.
            self.pma.set_u16(6, 0);
            self.set_rx_endpoint_status(&Endpoint::Endpoint0(&endpoint), EndpointStatus::Valid);
        }

        let address = self.get_address();
        if address > 0 {
            self.peripherals
                .USB
                .daddr
                .write(|w| unsafe { w.add().bits(address) });
            self.set_address(0);
        }
    }

    fn handle_device_request(&self, request_header: UsbRequestHeader) {
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

    fn handle_get_descriptor(&self, request_header: UsbRequestHeader) {
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

    fn handle_set_address(&self, request_header: UsbRequestHeader) {
        if request_header.index == 0 && request_header.length == 0 {
            if let DeviceState::Configured = self.get_device_state() {
                self.control_endpoint_error();
            } else {
                let address = (request_header.value & 0x7F) as u8;
                self.set_address(address);
                self.send_control_zero_length_packet();
                self.set_device_state(if address != 0 {
                    DeviceState::Addressed
                } else {
                    DeviceState::Default
                });
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn handle_set_configuration(&self, request_header: UsbRequestHeader) {
        let configuration_index = request_header.value as u8;

        self.set_configuration_index(configuration_index);

        if configuration_index > 1 {
            self.control_endpoint_error();
        } else {
            match self.get_device_state() {
                DeviceState::Addressed => {
                    if configuration_index != 0 {
                        self.open_device_endpoints();
                        self.send_control_zero_length_packet();
                        self.set_device_state(DeviceState::Configured);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                DeviceState::Configured => {
                    if configuration_index == 0 {
                        self.close_control_endpoints();
                        self.send_control_zero_length_packet();
                        self.set_device_state(DeviceState::Addressed);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_configuration(&self, request_header: UsbRequestHeader) {
        if request_header.length != 1 {
            self.control_endpoint_error();
        } else {
            match self.get_device_state() {
                DeviceState::Addressed => {
                    self.set_configuration_index(0);
                    self.send_control_data(Some([0].as_ref()));
                }
                DeviceState::Configured => {
                    self.send_control_data(Some([self.get_configuration_index()].as_ref()));
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_status(&self) {
        match self.get_device_state() {
            DeviceState::Addressed | DeviceState::Configured => {
                self.send_control_data(Some([3].as_ref()))
            }
            _ => {}
        }
    }

    fn handle_set_feature(&self, request_header: UsbRequestHeader) {
        if request_header.value == 1 {
            // ACK
            self.send_control_zero_length_packet();
        }
    }

    fn handle_clear_feature(&self, request_header: UsbRequestHeader) {
        match self.get_device_state() {
            DeviceState::Addressed | DeviceState::Configured => {
                if request_header.value == 1 {
                    // ACK
                    self.send_control_zero_length_packet();
                }
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_interface_request(&self, request_header: UsbRequestHeader) {
        match self.get_device_state() {
            DeviceState::Configured if (request_header.index & 0xff) <= 1 => {
                self.handle_setup(request_header);
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_setup(&self, request_header: UsbRequestHeader) {
        match request_header.kind {
            UsbRequestKind::Class => self.handle_class_setup(request_header),
            UsbRequestKind::Standard => self.handle_standard_setup(request_header),
            _ => {}
        }
    }

    fn handle_class_setup(&self, request_header: UsbRequestHeader) {
        match request_header.request {
            // CUSTOM_HID_REQ_SET_PROTOCOL
            UsbRequest::SetInterface => {
                self.set_protocol(request_header.value as u8);
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_PROTOCOL
            UsbRequest::SetFeature => self.send_control_data(Some([self.get_protocol()].as_ref())),
            // CUSTOM_HID_REQ_SET_IDLE
            UsbRequest::GetInterface => {
                self.set_idle_state((request_header.value >> 8) as u8);
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_IDLE
            UsbRequest::Two => self.send_control_data(Some([self.get_idle_state()].as_ref())),
            // CUSTOM_HID_REQ_SET_REPORT
            UsbRequest::SetConfiguration => {
                self.set_control_endpoint_state(ControlEndpointState::DataOut);
                self.pma.set_u16(6, request_header.length);
                self.set_rx_endpoint_status(
                    &Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
                    EndpointStatus::Valid,
                );
                self.send_control_zero_length_packet();
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_standard_setup(&self, request_header: UsbRequestHeader) {
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
                self.set_alt_setting(request_header.value as u8);
                self.send_control_zero_length_packet();
            }
            UsbRequest::GetInterface => {
                self.send_control_data(Some([self.get_alt_setting()].as_ref()))
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_endpoint_request(&self, request_header: UsbRequestHeader) {
        if let UsbRequestKind::Class = request_header.kind {
            self.handle_setup(request_header);
            return;
        }

        let endpoint_address = request_header.index as u8;
        let device_state = self.get_device_state();
        match request_header.request {
            UsbRequest::SetFeature => {
                match device_state {
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
                    _ => self.control_endpoint_error()
                }
            }
            UsbRequest::ClearFeature => {
                match device_state {
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
                    _ => self.control_endpoint_error()
                }
            }
            UsbRequest::GetStatus => {
                match device_state {
                    DeviceState::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceState::Configured => {
                        // SHOULD BE:  status=isStalled(ep_addr) ? 1 : 0; sendControlData(&status,2);
                        self.send_control_data(Some([0x0, 0x0].as_ref()));
                    }
                    _ => self.control_endpoint_error()
                }
            }
            _ => {}
        }
    }

    fn handle_device_out_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        let endpoint = &self.peripherals.USB.ep1r;
        endpoint.write(|w| w.ctr_rx().clear_bit());

        // Here we can check the amount of data and do smth with it....

        self.pma
            .set_u16(14, 0x8000 | (1 << 10) /* 32 byte size, 1 block */);

        self.set_rx_endpoint_status(&Endpoint::Endpoint1(endpoint), EndpointStatus::Valid);
    }

    fn handle_device_in_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        self.peripherals.USB.ep1r.write(|w| w.ctr_tx().clear_bit());
    }

    fn stall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        if endpoint_index == 0 {
            self.control_endpoint_error();
        } else {
            let endpoint = match endpoint_index {
                0 => Endpoint::Endpoint0(&self.peripherals.USB.ep0r),
                1 => Endpoint::Endpoint1(&self.peripherals.USB.ep1r),
                _ => panic!("AAA")
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
            _ => panic!("BBBB")
        };

        if endpoint_index == 0 || endpoint_address & 0x80 == 0x80 {
            self.set_tx_endpoint_status(&endpoint, EndpointStatus::Stall);
        } else if endpoint_address & 0x80 == 0x0 {
            self.set_rx_endpoint_status(&endpoint, EndpointStatus::Stall);
        }
    }

    fn control_endpoint_error(&self) {
        let endpoint = Endpoint::Endpoint0(&self.peripherals.USB.ep0r);
        self.set_rx_endpoint_status(&endpoint, EndpointStatus::Stall);
        self.set_tx_endpoint_status(&endpoint, EndpointStatus::Stall);
    }

    fn send_control_data(&self, data: Option<&[u8]>) {
        self.set_control_endpoint_state(ControlEndpointState::DataIn);
        self.send_data(EndpointType::Control, CONTROL_IN_PMA_ADDRESS, data);
    }

    fn send_control_zero_length_packet(&self) {
        self.set_control_endpoint_state(ControlEndpointState::StatusIn);
        self.send_data(EndpointType::Control, CONTROL_IN_PMA_ADDRESS, None);
    }

    fn send_data(&self, endpoint_type: EndpointType, pma_address: u16, data: Option<&[u8]>) {
        let length = data
            .and_then(|d| {
                self.pma.write_buffer_u8(pma_address as usize, &d);
                Some(d)
            })
            .map_or(0, |d| d.len() as u16);

        // Now that the PMA memory is prepared, set the length and tell the peripheral to send it.
        let (tx_count_offset, endpoint) = match endpoint_type {
            EndpointType::Control => (2, Endpoint::Endpoint0(&self.peripherals.USB.ep0r)),
            _ => (10, Endpoint::Endpoint1(&self.peripherals.USB.ep1r)),
        };

        self.pma.set_u16(tx_count_offset, length);
        self.set_tx_endpoint_status(&endpoint, EndpointStatus::Valid);
    }

    fn suspend(&self) {
        self.peripherals.USB.istr.write(|w| w.susp().clear_bit());

        // suspend and low power mode
        self.peripherals
            .USB
            .cntr
            .write(|w| w.fsusp().set_bit().lpmode().set_bit());

        self.set_device_state(DeviceState::Suspended);
    }

    fn wake_up(&self) {
        // Come out of low power mode.
        self.peripherals.USB.cntr.write(|w| w.lpmode().clear_bit());
        self.set_interrupt_mask();

        // clear interrupt flag
        self.peripherals.USB.istr.write(|w| w.wkup().clear_bit());

        self.set_device_state(DeviceState::WokenUp);
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
        self.pma.set_u16(0, CONTROL_IN_PMA_ADDRESS /* tx address */);
        self.pma.set_u16(2, 0x0);
        self.pma
            .set_u16(4, CONTROL_OUT_PMA_ADDRESS /* rx address */);
        self.pma
            .set_u16(6, 0x8000 | (1 << 10) /* 32 byte size, 1 block */);

        // Configure 1 (app) endpoint
        self.pma.set_u16(8, DEVICE_IN_PMA_ADDRESS as u16);
        self.pma.set_u16(10, 0x0);
        self.pma.set_u16(12, DEVICE_OUT_PMA_ADDRESS as u16);
        self.pma.set_u16(14, (0x8000 | (1 << 10)) as u16);
    }

    fn set_device_state(&self, device_state: DeviceState) {
        free(|cs| {
            let mut state = *USB_STATE.borrow(cs).borrow_mut();

            match (state.device_state, state.suspended_device_state) {
                (DeviceState::Suspended, _) => {
                    state.device_state = device_state;
                    state.suspended_device_state = Some(state.device_state);
                }
                (DeviceState::WokenUp, Some(previous_device_state)) => {
                    state.device_state = previous_device_state;
                    state.suspended_device_state = None;
                }
                (DeviceState::WokenUp, None) => {}
                _ => state.device_state = device_state,
            }
        });
    }

    fn get_device_state(&self) -> DeviceState {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).device_state)
    }

    fn set_control_endpoint_state(&self, control_endpoint_state: ControlEndpointState) {
        free(|cs| {
            let mut state = *USB_STATE.borrow(cs).borrow_mut();
            if let ControlEndpointState::Setup(data_length) = control_endpoint_state {
                state.setup_data_length = data_length;
            }

            state.control_endpoint_state = control_endpoint_state;
        });
    }

    fn get_control_endpoint_state(&self) -> ControlEndpointState {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).control_endpoint_state)
    }

    fn get_setup_data_length(&self) -> u16 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).setup_data_length)
    }

    fn get_address(&self) -> u8 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).address)
    }

    fn set_configuration_index(&self, index: u8) {
        free(|cs| {
            (*USB_STATE.borrow(cs).borrow_mut()).configuration_index = index;
        });
    }

    fn get_configuration_index(&self) -> u8 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).configuration_index)
    }

    fn set_protocol(&self, protocol: u8) {
        free(|cs| {
            (*USB_STATE.borrow(cs).borrow_mut()).protocol = protocol;
        });
    }

    fn get_protocol(&self) -> u8 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).protocol)
    }

    fn set_idle_state(&self, idle_state: u8) {
        free(|cs| {
            (*USB_STATE.borrow(cs).borrow_mut()).idle_state = idle_state;
        });
    }

    fn get_idle_state(&self) -> u8 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).idle_state)
    }

    fn set_alt_setting(&self, alt_setting: u8) {
        free(|cs| {
            (*USB_STATE.borrow(cs).borrow_mut()).alt_setting = alt_setting;
        });
    }

    fn get_alt_setting(&self) -> u8 {
        free(|cs| (*USB_STATE.borrow(cs).borrow()).alt_setting)
    }

    fn set_address(&self, address: u8) {
        if address == 0 {
            self.peripherals.USB.daddr.write(|w| w.ef().set_bit());
        }

        free(|cs| {
            (*USB_STATE.borrow(cs).borrow_mut()).address = address;
        });
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

    fn close_control_endpoints(&self) {
        let endpoint = Endpoint::Endpoint0(&self.peripherals.USB.ep0r);

        self.close_tx_endpoint(&endpoint);
        self.close_rx_endpoint(&endpoint);
    }

    fn close_device_endpoints(&self) {
        let endpoint = Endpoint::Endpoint1(&self.peripherals.USB.ep1r);

        self.close_tx_endpoint(&endpoint);
        self.close_rx_endpoint(&endpoint);
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

    fn close_rx_endpoint(&self, endpoint: &Endpoint) {
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| {
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
                    // if DTOG_RX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_rx().bit_is_set() {
                        w.dtog_rx().set_bit()
                    } else {
                        w
                    }
                })
            }
        }

        self.set_rx_endpoint_status(endpoint, EndpointStatus::Disabled);
    }

    fn close_tx_endpoint(&self, endpoint: &Endpoint) {
        match endpoint {
            Endpoint::Endpoint0(e) => {
                e.modify(|r, w| {
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
                    // if DTOG_TX is 1 then we need to write 1 to toggle it to zero
                    if r.dtog_tx().bit_is_set() {
                        w.dtog_tx().set_bit()
                    } else {
                        w
                    }
                })
            }
        }

        self.set_tx_endpoint_status(endpoint, EndpointStatus::Disabled);
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
