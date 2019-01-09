pub const LANG_ID_DESCRIPTOR: [u8; 4] = [
    0x04, 0x03, //
    0x09, 0x04, // English - US
];

// To get unicode code in JS web console use e.g. `0x${'K'.charCodeAt(0).toString(16)}`
pub const MANUFACTURER_STR: [u8; 24] = [
    0x26, 0x03, //
    0x4b, 0x00, // K
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x65, 0x00, // e
    0x75, 0x00, // u
    0x6d, 0x00, // m
    0x20, 0x00, //
    0x43, 0x00, // C
    0x6f, 0x00, // o
    0x2e, 0x00, // .
];

pub const PRODUCT_STR: [u8; 16] = [
    0x1c, 0x03, //
    0x4b, 0x00, // K
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x65, 0x00, // e
    0x75, 0x00, // u
    0x6d, 0x00, // m
];

pub const SERIAL_NUMBER_STR: [u8; 12] = [
    0x0e, 0x03, //
    0x31, 0x00, // 1
    0x2e, 0x00, // .
    0x30, 0x00, // 0
    0x2e, 0x00, // .
    0x30, 0x00, // 0
];

pub const CONF_STR: [u8; 26] = [
    0x28, 0x03, //
    0x4b, 0x00, // K
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x65, 0x00, // e
    0x75, 0x00, // u
    0x6d, 0x00, // m
    0x20, 0x00, //
    0x63, 0x00, // c
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x66, 0x00, // f
];

pub const INTERFACE_STR: [u8; 22] = [
    0x20, 0x03, //
    0x4b, 0x00, // K
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x6e, 0x00, // n
    0x65, 0x00, // e
    0x75, 0x00, // u
    0x6d, 0x00, // m
    0x20, 0x00, //
    0x69, 0x00, // i
    0x66, 0x00, // f
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