use config::{DEVICE_PID, DEVICE_VID};

/*
 * These are the USB device strings in the format required for a USB string descriptor.
 * To change these to suit your device you need only change the unicode string in the
 * last line of each definition to suit your device. Then count up the bytes required for
 * the complete descriptor and go back and insert that byte count in the array declaration
 * in the configuration class.
 */

// Maximum size of the USB packet (64 bytes).
pub const MAX_PACKET_SIZE: usize = 0x40;

pub const LANG_ID_DESCRIPTOR: [u8; 4] = [
    0x04, 0x03, // 4 (length)
    0x09, 0x04, // English - US
];

// To get unicode code in JS web console use e.g. `0x${'K'.charCodeAt(0).toString(16)}`
pub const MANUFACTURER_STR: [u8; 24] = [
    0x18, 0x03, // 24 (length)
    b'K', 0x00, b'r', 0x00, b'o', 0x00, b'n', 0x00, b'e', 0x00, b'u', 0x00, b'm', 0x00, b' ', 0x00,
    b'C', 0x00, b'o', 0x00, b'.', 0x00,
];

pub const PRODUCT_STR: [u8; 16] = [
    0x10, 0x03, // 16 (length)
    b'K', 0x00, b'r', 0x00, b'o', 0x00, b'n', 0x00, b'e', 0x00, b'u', 0x00, b'm', 0x00,
];

pub const SERIAL_NUMBER_STR: [u8; 12] = [
    0x0c, 0x03, // 12 (length)
    b'1', 0x00, b'.', 0x00, b'0', 0x00, b'.', 0x00, b'0', 0x00,
];

pub const CONF_STR: [u8; 26] = [
    0x1a, 0x03, // 26 (length)
    b'K', 0x00, b'r', 0x00, b'o', 0x00, b'n', 0x00, b'e', 0x00, b'u', 0x00, b'm', 0x00, b' ', 0x00,
    b'c', 0x00, b'o', 0x00, b'n', 0x00, b'f', 0x00,
];

pub const INTERFACE_STR: [u8; 22] = [
    0x16, 0x03, // 22 (length)
    b'K', 0x00, b'r', 0x00, b'o', 0x00, b'n', 0x00, b'e', 0x00, b'u', 0x00, b'm', 0x00, b' ', 0x00,
    b'i', 0x00, b'f', 0x00,
];

pub const DEV_DESC: [u8; 18] = [
    0x12, // bLength
    0x01, // bDescriptorType (Device)
    0x00,
    0x02,                      // bcdUSB 2.00
    0x00,                      // bDeviceClass (Use class information in the Interface Descriptors)
    0x00,                      // bDeviceSubClass
    0x00,                      // bDeviceProtocol
    MAX_PACKET_SIZE as u8,     // bMaxPacketSize0 64
    (DEVICE_VID & 0xff) as u8, // idVendor 0xFFFF (split u16 into two u8)
    ((DEVICE_VID & 0xff00) >> 8) as u8,
    (DEVICE_PID & 0xff) as u8, // idProduct 0xFFFF (split u16 into two u8)
    ((DEVICE_PID & 0xff00) >> 8) as u8,
    0x01,
    0x00, // bcdDevice 0.01
    0x01, // iManufacturer (String Index)
    0x02, // iProduct (String Index)
    0x03, // iSerialNumber (String Index)
    0x01, // bNumConfigurations 1
];

pub const CONF_DESC: [u8; 66] = [
    0x09, // bLength
    0x02, // bDescriptorType (Configuration)
    0x42,
    0x00, // wTotalLength
    0x02, // bNumInterfaces
    0x01, // bConfigurationValue
    0x04, // iConfiguration (String Index)
    0x80, // bmAttributes
    0xFA, // bMaxPower 500mA
    // System Interface
    0x09, // bLength
    0x04, // bDescriptorType (Interface)
    0x00, // bInterfaceNumber 0
    0x00, // bAlternateSetting
    0x02, // bNumEndpoints 2
    0x03, // bInterfaceClass
    0x00, // bInterfaceSubClass 1=BOOT, 0=no boot
    0x00, // bInterfaceProtocol 0=none, 1=keyboard, 2=mouse
    0x00, // iInterface (String Index)
    // System HID descriptor
    0x09, // bLength
    0x21, // bDescriptorType (HID)
    0x11,
    0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x20,
    0x00, // wDescriptorLength[0] 32
    // System IN endpoint descriptor
    0x07, // bLength
    0x05, // bDescriptorType (Endpoint)
    0x81, // bEndpointAddress (IN/D2H)
    0x03, // bmAttributes (Interrupt)
    MAX_PACKET_SIZE as u8,
    0x00, // wMaxPacketSize 64
    0x20, // bInterval 1 (unit depends on device speed)
    // System OUT endpoint descriptor
    0x07, // bLength
    0x05, // bDescriptorType (Endpoint)
    0x01, // bEndpointAddress (OUT/H2D)
    0x03, // bmAttributes (Interrupt)
    MAX_PACKET_SIZE as u8,
    0x00, // wMaxPacketSize 64
    0x20, // bInterval 1 (unit depends on device speed),
    // Keyboard Interface
    0x09, // bLength
    0x04, // bDescriptorType (Interface)
    0x01, // bInterfaceNumber 1
    0x00, // bAlternateSetting
    0x01, // bNumEndpoints 1
    0x03, // bInterfaceClass
    0x01, // bInterfaceSubClass 1=BOOT, 0=no boot
    0x01, // bInterfaceProtocol 0=none, 1=keyboard, 2=mouse
    0x00, // iInterface (String Index)
    // Keyboard HID descriptor
    0x09, // bLength
    0x21, // bDescriptorType (HID)
    0x11,
    0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x3F,
    0x00, // wDescriptorLength[0] 63
    // Keyboard IN endpoint descriptor
    0x07, // bLength
    0x05, // bDescriptorType (Endpoint)
    0x82, // bEndpointAddress (IN/D2H)
    0x03, // bmAttributes (Interrupt)
    MAX_PACKET_SIZE as u8,
    0x00, // wMaxPacketSize 64
    0x0A, // bInterval 10 (unit depends on device speed, expressed in milliseconds),
];

// The HID descriptor (this is a copy of the descriptor embedded in the above configuration descriptor.
pub const SYSTEM_HID_DESC: [u8; 9] = [
    0x09, // bLength: CUSTOM_HID Descriptor size
    0x21, // bDescriptorType (HID)
    0x11, 0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x20, 0x00, // wDescriptorLength[0] 32
];

// The HID descriptor (this is a copy of the descriptor embedded in the above configuration descriptor.
pub const KEYBOARD_HID_DESC: [u8; 9] = [
    0x09, // bLength: CUSTOM_HID Descriptor size
    0x21, // bDescriptorType (HID)
    0x11, 0x01, // bcdHID 1.11
    0x00, // bCountryCode
    0x01, // bNumDescriptors
    0x22, // bDescriptorType[0] (HID)
    0x3F, 0x00, // wDescriptorLength[0] 63
];

pub const SYSTEM_REPORT_DESC: [u8; 32] = [
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

pub const KEYBOARD_REPORT_DESC: [u8; 63] = [
    0x05, 0x01, // USAGE_PAGE (Generic Desktop)
    0x09, 0x06, // USAGE (Keyboard)
    0xa1, 0x01, // COLLECTION (Application)
    0x05, 0x07, //   Usage Page (Key Codes)
    0x19, 0xE0, //   Usage Minimum (224)
    0x29, 0xE7, //   Usage Maximum (231)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x08, //   Report Count (8)
    0x81, 0x02, //   Input (Data, Variable, Absolute), Modifier byte
    0x95, 0x01, //   Report Count (1)
    0x75, 0x08, //   Report Size (8)
    0x81, 0x01, //   Input (Constant), Reserved byte
    0x95, 0x05, //   Report Count (5)
    0x75, 0x01, //   Report Size (1)
    0x05, 0x08, //   Usage Page (Page# for LEDs)
    0x19, 0x01, //   Usage Minimum (1)
    0x29, 0x05, //   Usage Maximum (5)
    0x91, 0x02, //   Output (Data, Variable, Absolute), LED report
    0x95, 0x01, //   Report Count (1)
    0x75, 0x03, //   Report Size (3)
    0x91, 0x01, //   Output (Constant), LED report padding
    0x95, 0x06, //   Report Count (6)
    0x75, 0x08, //   Report Size (8)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x65, //   Logical Maximum (101)
    0x05, 0x07, //   Usage Page (Key Codes)
    0x19, 0x00, //   Usage Minimum (0)
    0x29, 0x65, //   Usage Maximum (101)
    0x81, 0x00, //   Input (Data, Array), Key arrays (6 bytes)
    0xc0, // END_COLLECTION
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_descriptors_well_formed() {
        let strings: [&[u8]; 5] = [
            &MANUFACTURER_STR,
            &PRODUCT_STR,
            &SERIAL_NUMBER_STR,
            &CONF_STR,
            &INTERFACE_STR,
        ];

        for string in strings.iter() {
            assert_eq!(string[0], string.len() as u8);
            assert_eq!(string[1], 0x3);
            // Chars should be NULL terminated.
            assert_eq!(
                // Skip first two control bytes, the rest is the string content itself.
                string[2..]
                    .iter()
                    .enumerate()
                    .any(|(index, val)| { index % 2 != 0 && *val != 0 }),
                false
            )
        }
    }

    #[test]
    fn descriptors_with_correct_length() {
        let descriptors: [&[u8]; 4] = [
            &LANG_ID_DESCRIPTOR,
            &DEV_DESC,
            &SYSTEM_HID_DESC,
            &KEYBOARD_HID_DESC,
        ];

        for descriptor in descriptors.iter() {
            assert_eq!(descriptor[0], descriptor.len() as u8);
        }

        // Config descriptor length.
        assert_eq!(CONF_DESC[2], CONF_DESC.len() as u8);

        // HID reports lengths.
        assert_eq!(
            SYSTEM_HID_DESC[SYSTEM_HID_DESC.len() - 2],
            SYSTEM_REPORT_DESC.len() as u8
        );
        assert_eq!(
            KEYBOARD_HID_DESC[KEYBOARD_HID_DESC.len() - 2],
            KEYBOARD_REPORT_DESC.len() as u8
        );
    }
}
