/*
 * These are the USB device strings in the format required for a USB string descriptor.
 * To change these to suit your device you need only change the unicode string in the
 * last line of each definition to suit your device. Then count up the bytes required for
 * the complete descriptor and go back and insert that byte count in the array declaration
 * in the configuration class.
 */

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
        let descriptors: [&[u8]; 3] = [&LANG_ID_DESCRIPTOR, &DEV_DESC, &HID_DESC];

        for descriptor in descriptors.iter() {
            assert_eq!(descriptor[0], descriptor.len() as u8);
        }

        assert_eq!(CONF_DESC[2], CONF_DESC.len() as u8);
    }
}
