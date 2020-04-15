pub mod command_packet;
pub mod commands;
mod descriptors;
pub mod endpoint;
mod packet_queue;
mod pma;
mod setup_packet;
pub mod usb_error;

use self::{
    command_packet::CommandPacket,
    descriptors::*,
    endpoint::{
        ControlEndpointStatus, DeviceEndpoint, EndpointDirection, EndpointStatus, EndpointType,
    },
    packet_queue::PacketQueue,
    pma::PacketMemoryArea,
    setup_packet::{Request, RequestKind, RequestRecipient, SetupPacket},
};
use array::Array;
use core::convert::TryFrom;

pub const SUPPORTED_ENDPOINTS: [EndpointType; 3] = [
    EndpointType::Control,
    EndpointType::Device(DeviceEndpoint::System),
    EndpointType::Device(DeviceEndpoint::Keyboard),
];

#[derive(Copy, Clone)]
enum DeviceStatus {
    // Device hasn't been started yet, starting, or has been disconnected.
    Default,
    // We've received an address from the host.
    Addressed,
    // Enumeration is complete, we can talk to the host.
    Configured,
}

/// Possible interrupt types.
pub enum UsbInterrupt {
    Reset,
    Error,
    CorrectTransfer,
    SuspendSoFEsoF,
}

pub struct TransactionFlags {
    pub setup: bool,
    pub rx: bool,
    pub tx: bool,
}

pub struct Transaction {
    pub endpoint: EndpointType,
    pub direction: EndpointDirection,
    pub flags: TransactionFlags,
}

#[derive(Copy, Clone)]
pub struct UsbState {
    device_status: DeviceStatus,
    control_endpoint_status: ControlEndpointStatus,
    packets: PacketQueue,
    address: u8,
    configuration_index: u8,
    protocol: u8,
    idle_state: u8,
    alt_setting: u8,
    pub command: Option<CommandPacket>,
}

impl Default for UsbState {
    fn default() -> Self {
        UsbState {
            device_status: DeviceStatus::Default,
            control_endpoint_status: ControlEndpointStatus::Idle,
            packets: PacketQueue::new(),
            address: 0,
            configuration_index: 0,
            protocol: 0,
            idle_state: 0,
            alt_setting: 0,
            command: None,
        }
    }
}

// Describes USB hardware management interface.
pub trait USBHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Enables USB device.
    fn enable(&self);

    /// Returns start address of the BTABLE.
    fn btable_address(&self) -> usize;

    /// Used to retrieve transaction that has been completed and caused `Correct Transfer` interrupt.
    fn transaction(&self) -> Transaction;

    /// Sets status for the specified endpoint.
    fn set_endpoint_status(
        &self,
        endpoint: EndpointType,
        direction: EndpointDirection,
        status: EndpointStatus,
    );

    /// Assigns specified address to the USB device.
    fn set_address(&self, address: u8);

    /// Opens specified endpoint.
    fn open_endpoint(&self, endpoint: EndpointType);

    /// Closes specified endpoint.
    fn close_endpoint(&self, endpoint: EndpointType);

    /// Checks whether specified interrupt is active.
    fn is_interrupt_active(&self, interrupt: UsbInterrupt) -> bool;

    /// Tells hardware that specified USB interrupt has been handled.
    fn mark_interrupt_as_handled(&self, interrupt: UsbInterrupt);

    /// Tells USB peripheral that specific transaction has been successfully handled.
    fn mark_transaction_as_handled(&self, endpoint: EndpointType, direction: EndpointDirection);
}

pub struct USB<'a, T: USBHardware> {
    hw: &'a T,
    pma: PacketMemoryArea,
    state: &'a mut UsbState,
}

impl<'a, T: USBHardware> USB<'a, T> {
    pub fn new(hw: &'a T, state: &'a mut UsbState) -> Self {
        let base_address = hw.btable_address();
        USB {
            hw,
            pma: PacketMemoryArea { base_address },
            state,
        }
    }

    /// Prepares and starts USB hardware (setup clocks, SOF etc.).
    pub fn setup(&mut self) {
        self.hw.setup();
        self.pma.init(&SUPPORTED_ENDPOINTS);

        self.state.address = 0;
        self.update_device_status(DeviceStatus::Default);
    }

    /// Stops and releases USB hardware (stops clocks, disables interrupts etc.).
    pub fn teardown(&mut self) {
        SUPPORTED_ENDPOINTS
            .iter()
            .for_each(|endpoint| self.hw.close_endpoint(*endpoint));

        self.state.address = 0;
        self.update_device_status(DeviceStatus::Default);
        self.hw.teardown();
    }

    pub fn interrupt(&mut self) {
        if self.hw.is_interrupt_active(UsbInterrupt::Reset) {
            self.reset();
        }

        if self.hw.is_interrupt_active(UsbInterrupt::Error) {
            self.hw.mark_interrupt_as_handled(UsbInterrupt::Error);
        }

        // Clear SUSP, SOF and ESOF
        self.hw
            .mark_interrupt_as_handled(UsbInterrupt::SuspendSoFEsoF);

        // Correct endpoint transfer
        if self.hw.is_interrupt_active(UsbInterrupt::CorrectTransfer) {
            self.correct_transfer();
        }
    }

    /// Sends report via specified Device endpoint.
    pub fn send(&mut self, endpoint: DeviceEndpoint, data: &[u8]) {
        self.send_data(EndpointType::Device(endpoint), &data);
    }

    fn correct_transfer(&mut self) {
        // USB_ISTR_CTR is read only and will be automatically cleared by hardware when we process
        // all endpoint results.
        while self.hw.is_interrupt_active(UsbInterrupt::CorrectTransfer) {
            let transaction = self.hw.transaction();
            match (&transaction.endpoint, &transaction.direction) {
                (EndpointType::Control, EndpointDirection::Receive) => {
                    self.handle_control_out_transfer(&transaction)
                }
                (EndpointType::Control, EndpointDirection::Transmit) => {
                    self.handle_control_in_transfer(&transaction)
                }
                (EndpointType::Device(_), EndpointDirection::Receive) => {
                    self.handle_device_out_transfer(&transaction)
                }
                (EndpointType::Device(_), EndpointDirection::Transmit) => {
                    self.handle_device_in_transfer(&transaction)
                }
            }
        }
    }

    fn handle_control_out_transfer(&mut self, transaction: &Transaction) {
        if transaction.flags.setup {
            self.handle_control_setup_out_transfer(transaction);
        } else if transaction.flags.rx {
            self.handle_control_data_out_transfer(transaction);
        }
    }

    fn handle_control_setup_out_transfer(&mut self, transaction: &Transaction) {
        let setup_packet = SetupPacket::from((
            self.pma.read(transaction.endpoint, 0),
            self.pma.read(transaction.endpoint, 2),
            self.pma.read(transaction.endpoint, 4),
            self.pma.read(transaction.endpoint, 6),
        ));

        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);
        self.update_control_endpoint_status(ControlEndpointStatus::Setup);

        match setup_packet.recipient {
            RequestRecipient::Device => self.handle_device_request(setup_packet),
            RequestRecipient::Interface => self.handle_interface_request(setup_packet),
            RequestRecipient::Endpoint => self.handle_endpoint_request(setup_packet),
            _ => self.hw.set_endpoint_status(
                transaction.endpoint,
                transaction.direction,
                EndpointStatus::Stall,
            ),
        }
    }

    fn handle_control_data_out_transfer(&self, transaction: &Transaction) {
        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);

        // Here we can check the amount of data and do smth with it....

        self.pma.set_rx_count(transaction.endpoint, 0);
        self.hw.set_endpoint_status(
            transaction.endpoint,
            transaction.direction,
            EndpointStatus::Valid,
        );
    }

    fn handle_control_in_transfer(&mut self, transaction: &Transaction) {
        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);

        // If we have pending packets, continue to send to them.
        if let Some(packet) = self.state.packets.dequeue(transaction.endpoint) {
            self.send_packet(transaction.endpoint, packet.as_ref());
            return;
        }

        if let ControlEndpointStatus::DataIn = self.state.control_endpoint_status {
            self.update_control_endpoint_status(ControlEndpointStatus::DataOut);

            // Prepare for premature end of transfer.
            self.pma.set_rx_count(transaction.endpoint, 0);
            self.hw.set_endpoint_status(
                transaction.endpoint,
                EndpointDirection::Receive,
                EndpointStatus::Valid,
            );
        }

        if self.state.address > 0 {
            self.hw.set_address(self.state.address);
            self.state.address = 0;
        }
    }

    fn handle_device_out_transfer(&mut self, transaction: &Transaction) {
        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);

        let command_packet_length = self.pma.rx_count(transaction.endpoint) as usize;
        let mut command_byte_array = Array::new();
        for index in (0..command_packet_length).step_by(2) {
            let half_word = self.pma.read(transaction.endpoint, index as u16);
            command_byte_array.push((half_word & 0x00ff) as u8);
            // It's possible to receive odd number of bytes, and second part of `u16` will contain
            // some garbage value we don't want to pick up.
            if command_byte_array.len() < command_packet_length {
                command_byte_array.push(((half_word & 0xff00) >> 8) as u8);
            }
        }

        self.state.command = CommandPacket::try_from(command_byte_array).ok();

        self.pma.set_rx_count(transaction.endpoint, 0);
        self.hw.set_endpoint_status(
            transaction.endpoint,
            EndpointDirection::Receive,
            EndpointStatus::Valid,
        );
    }

    fn handle_device_in_transfer(&mut self, transaction: &Transaction) {
        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);

        // If we have pending packets, continue to send to them.
        if let Some(packet) = self.state.packets.dequeue(transaction.endpoint) {
            self.send_packet(transaction.endpoint, packet.as_ref());
        }
    }

    fn update_address(&mut self, address: u8) {
        if address == 0 {
            self.hw.enable();
        }

        self.state.address = address;
    }

    fn send_data(&mut self, endpoint_type: EndpointType, data: &[u8]) {
        // If data to send is larger than maximum packet size, let's send the first chunk and put
        // the rest of the data into packet queue.
        let packet = if data.len() > MAX_PACKET_SIZE {
            self.state
                .packets
                .enqueue(endpoint_type, &data[MAX_PACKET_SIZE..]);
            &data[..MAX_PACKET_SIZE]
        } else {
            self.state.packets.clear(endpoint_type);
            data
        };

        self.send_packet(endpoint_type, packet);
    }

    fn send_packet<'p, P: IntoIterator<Item = &'p u8>>(
        &mut self,
        endpoint_type: EndpointType,
        packet: P,
    ) {
        self.pma
            .set_tx_count(endpoint_type, self.pma.write(endpoint_type, packet) as u16);

        // Now that the PMA memory is prepared,tell the peripheral to send it.
        self.hw.set_endpoint_status(
            endpoint_type,
            EndpointDirection::Transmit,
            EndpointStatus::Valid,
        );
    }

    fn send_control_data(&mut self, data: &[u8]) {
        self.update_control_endpoint_status(ControlEndpointStatus::DataIn);
        self.send_data(EndpointType::Control, data);
    }

    fn send_control_zero_length_packet(&mut self) {
        self.update_control_endpoint_status(ControlEndpointStatus::StatusIn);
        self.send_data(EndpointType::Control, &[]);
    }

    fn reset(&mut self) {
        self.hw.mark_interrupt_as_handled(UsbInterrupt::Reset);

        self.update_address(0);
        self.hw.open_endpoint(EndpointType::Control);
    }

    fn update_device_status(&mut self, device_status: DeviceStatus) {
        self.state.device_status = device_status
    }

    fn update_control_endpoint_status(&mut self, control_endpoint_status: ControlEndpointStatus) {
        self.state.control_endpoint_status = control_endpoint_status;
    }

    fn stall_endpoint(&self, endpoint_address: u8) {
        match EndpointType::try_from(endpoint_address & 0x7f) {
            Ok(EndpointType::Control) => {
                self.control_endpoint_error();
            }
            Ok(endpoint) => {
                let direction = if endpoint_address & 0x80 == 0x80 {
                    EndpointDirection::Transmit
                } else {
                    EndpointDirection::Receive
                };

                self.hw
                    .set_endpoint_status(endpoint, direction, EndpointStatus::Stall);
            }
            _ => {}
        };
    }

    fn unstall_endpoint(&self, endpoint_address: u8) {
        if let Ok(endpoint) = EndpointType::try_from(endpoint_address & 0x7f) {
            let direction = if endpoint.is_control() || endpoint_address & 0x80 == 0x80 {
                EndpointDirection::Transmit
            } else {
                EndpointDirection::Receive
            };

            self.hw
                .set_endpoint_status(endpoint, direction, EndpointStatus::Stall);
        }
    }

    fn handle_endpoint_request(&mut self, request_header: SetupPacket) {
        let endpoint_address = request_header.index as u8;
        let is_device_endpoint = endpoint_address & 0x7f != 0;
        match (request_header.request, self.state.device_status) {
            (Request::SetFeature, DeviceStatus::Addressed)
            | (Request::ClearFeature, DeviceStatus::Addressed) => {
                if is_device_endpoint {
                    self.stall_endpoint(endpoint_address);
                }
            }
            (Request::SetFeature, DeviceStatus::Configured) => {
                // USB_FEATURE_EP_HALT
                if request_header.value == 0 && is_device_endpoint {
                    self.stall_endpoint(endpoint_address);
                }

                self.send_control_zero_length_packet();
            }
            (Request::ClearFeature, DeviceStatus::Configured) => {
                // USB_FEATURE_EP_HALT
                if request_header.value == 0 && is_device_endpoint {
                    self.unstall_endpoint(endpoint_address);
                }
            }
            (Request::GetStatus, DeviceStatus::Configured)
            | (Request::GetStatus, DeviceStatus::Addressed) => {
                // SHOULD BE:  status=isStalled(ep_addr) ? 1 : 0; sendControlData(&status,2);
                self.send_control_data(&[0x0, 0x0]);
            }
            (Request::SetFeature, _) | (Request::ClearFeature, _) | (Request::GetStatus, _) => {
                self.control_endpoint_error()
            }
            _ => {}
        }
    }

    fn handle_device_request(&mut self, request_header: SetupPacket) {
        match request_header.request {
            Request::GetDescriptor => self.handle_get_descriptor(request_header),
            Request::SetAddress => self.handle_set_address(request_header),
            Request::SetConfiguration => self.handle_set_configuration(request_header),
            Request::GetConfiguration => self.handle_get_configuration(request_header),
            Request::GetStatus => self.handle_get_status(),
            Request::SetFeature => self.handle_set_feature(request_header),
            Request::ClearFeature => self.handle_clear_feature(request_header),
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_get_descriptor(&mut self, request_header: SetupPacket) {
        // See USB 2.0 Specification, Table 9-5. Descriptor Types
        let data_to_send: Option<&[u8]> =
            match (request_header.value >> 8, request_header.value & 0xff) {
                (0x1, _) => Some(&DEV_DESC),
                (0x2, _) => Some(&CONF_DESC),
                (0x3, 0x0) => Some(&LANG_ID_DESCRIPTOR),
                // 0x1 - 0x3 Based on values in Device descriptor.
                (0x3, 0x1) => Some(&MANUFACTURER_STR),
                (0x3, 0x2) => Some(&PRODUCT_STR),
                (0x3, 0x3) => Some(&SERIAL_NUMBER_STR),
                // 0x4 - Based on value in Config descriptor (iConfiguration)
                (0x3, 0x4) => Some(&CONF_STR),
                // 0x5 - Based on value in Interface descriptor (iInterface)
                (0x3, 0x5) => Some(&INTERFACE_STR),
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
                self.send_control_data(&data[..data_to_send_length]);
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn handle_set_address(&mut self, request_header: SetupPacket) {
        if request_header.index == 0 && request_header.length == 0 {
            if let DeviceStatus::Configured = self.state.device_status {
                self.control_endpoint_error();
            } else {
                let address = (request_header.value & 0x7F) as u8;
                self.update_address(address);
                self.send_control_zero_length_packet();
                self.update_device_status(if address != 0 {
                    DeviceStatus::Addressed
                } else {
                    DeviceStatus::Default
                });
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn handle_set_configuration(&mut self, request_header: SetupPacket) {
        let configuration_index = request_header.value as u8;
        self.state.configuration_index = configuration_index;

        if configuration_index > 1 || matches!(self.state.device_status, DeviceStatus::Default) {
            self.control_endpoint_error();
            return;
        }

        let device_status = match self.state.device_status {
            DeviceStatus::Addressed if configuration_index != 0 => {
                self.hw
                    .open_endpoint(EndpointType::Device(DeviceEndpoint::System));
                self.hw
                    .open_endpoint(EndpointType::Device(DeviceEndpoint::Keyboard));

                DeviceStatus::Configured
            }
            DeviceStatus::Configured if configuration_index == 0 => {
                self.hw
                    .close_endpoint(EndpointType::Device(DeviceEndpoint::System));
                self.hw
                    .close_endpoint(EndpointType::Device(DeviceEndpoint::Keyboard));

                DeviceStatus::Addressed
            }
            _ => self.state.device_status,
        };

        self.send_control_zero_length_packet();
        self.update_device_status(device_status);
    }

    fn handle_get_configuration(&mut self, request_header: SetupPacket) {
        if request_header.length != 1 {
            self.control_endpoint_error();
        } else {
            match self.state.device_status {
                DeviceStatus::Addressed => {
                    self.state.configuration_index = 0;
                    self.send_control_data(&[0]);
                }
                DeviceStatus::Configured => {
                    self.send_control_data(&[self.state.configuration_index])
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_status(&mut self) {
        if let DeviceStatus::Addressed | DeviceStatus::Configured = self.state.device_status {
            // Bus powered, supports remote wakeup.
            self.send_control_data(&[0x2, 0x0]);
        }
    }

    fn handle_set_feature(&mut self, request_header: SetupPacket) {
        if request_header.value == 1 {
            // ACK
            self.send_control_zero_length_packet();
        }
    }

    fn handle_clear_feature(&mut self, request_header: SetupPacket) {
        if let DeviceStatus::Addressed | DeviceStatus::Configured = self.state.device_status {
            if request_header.value == 1 {
                // ACK
                self.send_control_zero_length_packet();
            }
        } else {
            self.control_endpoint_error();
        }
    }

    fn handle_interface_request(&mut self, request_header: SetupPacket) {
        match (self.state.device_status, &request_header.kind) {
            (DeviceStatus::Configured, RequestKind::Standard) => {
                self.handle_standard_setup(request_header)
            }
            (DeviceStatus::Configured, RequestKind::Class) => {
                self.handle_class_setup(request_header)
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_standard_setup(&mut self, request_header: SetupPacket) {
        match request_header.request {
            // See HID Spec 7.1: the HID class uses the standard request `Get_Descriptor` as
            // described in the USB Specification.
            Request::GetDescriptor => {
                let ack_data = [];
                // Value is 2 bytes value, we need only high byte:
                let data = match (request_header.value >> 8, request_header.index) {
                    // USB_DESC_TYPE_HID_DESCRIPTOR (HID)
                    (0x21, 0) => get_hid_descriptor(DeviceEndpoint::System),
                    (0x21, 1) => get_hid_descriptor(DeviceEndpoint::Keyboard),
                    // USB_DESC_TYPE_HID_REPORT (Report)
                    (0x22, 0) => get_hid_report_descriptor(DeviceEndpoint::System),
                    (0x22, 1) => get_hid_report_descriptor(DeviceEndpoint::Keyboard),
                    // 0x23 - Physical descriptor, 0x24 - 0x2F Reserved or unknown interface.
                    _ => ack_data.as_ref(),
                };

                let report_length = request_header.length as usize;
                self.send_control_data(if report_length < data.len() {
                    &data[..report_length]
                } else {
                    &data
                });
            }
            Request::GetStatus => self.send_control_data(&[0x0, 0x0]),
            Request::SetInterface => {
                self.state.alt_setting = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            Request::GetInterface => self.send_control_data(&[self.state.alt_setting]),
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_class_setup(&mut self, request_header: SetupPacket) {
        match request_header.request {
            // CUSTOM_HID_REQ_GET_IDLE
            Request::Two => self.send_control_data(&[self.state.idle_state]),
            // CUSTOM_HID_REQ_GET_PROTOCOL
            Request::SetFeature => self.send_control_data(&[self.state.protocol]),
            // CUSTOM_HID_REQ_SET_REPORT
            Request::SetConfiguration => {
                self.update_control_endpoint_status(ControlEndpointStatus::DataOut);
                self.pma
                    .set_rx_count(EndpointType::Control, request_header.length);
                self.hw.set_endpoint_status(
                    EndpointType::Control,
                    EndpointDirection::Receive,
                    EndpointStatus::Valid,
                );
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_SET_IDLE
            Request::GetInterface => {
                self.state.idle_state = (request_header.value >> 8) as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_SET_PROTOCOL
            Request::SetInterface => {
                self.state.protocol = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn control_endpoint_error(&self) {
        self.hw.set_endpoint_status(
            EndpointType::Control,
            EndpointDirection::Receive,
            EndpointStatus::Stall,
        );
        self.hw.set_endpoint_status(
            EndpointType::Control,
            EndpointDirection::Transmit,
            EndpointStatus::Stall,
        );
    }
}
