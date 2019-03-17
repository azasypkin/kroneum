pub mod command_packet;
mod descriptors;
mod pma;
mod setup_packet;

use self::command_packet::CommandPacket;
use self::descriptors::*;
use self::pma::PacketMemoryArea;
use self::setup_packet::{Request, RequestKind, RequestRecipient, SetupPacket};

#[derive(Copy, Clone)]
pub enum EndpointType {
    Control = 0b0,
    Device = 0b1,
}

#[derive(Copy, Clone)]
pub enum EndpointDirection {
    Receive,
    Transmit,
}

#[derive(Copy, Clone)]
pub enum EndpointStatus {
    Disabled = 0b0,
    Stall = 0b01,
    Nak = 0b10,
    Valid = 0b11,
}

#[derive(Copy, Clone)]
pub enum DeviceStatus {
    // Device hasn't been started yet, starting, or has been disconnected.
    Default,
    // We've received an address from the host.
    Addressed,
    // Enumeration is complete, we can talk to the host.
    Configured,
    // Device is suspended.
    Suspended,
    // Synthetic status for the woken up device,
    WokenUp,
}

/// The possible statuses for the control endpoint.
#[derive(Copy, Clone)]
pub enum ControlEndpointStatus {
    Idle,
    Setup(u16),
    DataIn,
    DataOut,
    StatusIn,
    StatusOut,
    Stall,
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
    pub device_status: DeviceStatus,
    pub suspended_device_status: Option<DeviceStatus>,
    pub control_endpoint_status: ControlEndpointStatus,
    pub setup_data_length: u16,
    pub address: u8,
    pub configuration_index: u8,
    pub protocol: u8,
    pub idle_state: u8,
    pub alt_setting: u8,
    pub command: Option<CommandPacket>,
}

impl Default for UsbState {
    fn default() -> Self {
        UsbState {
            device_status: DeviceStatus::Default,
            suspended_device_status: None,
            control_endpoint_status: ControlEndpointStatus::Idle,
            setup_data_length: 0,
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
    /// Enables USB device.
    fn enable(&mut self);

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
    hw: T,
    pma: PacketMemoryArea,
    state: &'a mut UsbState,
}

impl<'a, T: USBHardware> USB<'a, T> {
    pub fn create(hw: T, state: &'a mut UsbState) -> Self {
        USB {
            hw,
            pma: PacketMemoryArea::default(),
            state,
        }
    }

    pub fn start(&mut self) {
        self.pma.init();

        self.state.address = 0;
        self.update_device_status(DeviceStatus::Default);
    }

    pub fn stop(&mut self) {
        self.hw.close_endpoint(EndpointType::Device);
        self.hw.close_endpoint(EndpointType::Control);

        self.state.address = 0;
        self.update_device_status(DeviceStatus::Default);
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

    /// Sends custom report via Device endpoint.
    pub fn send(&self, data: &[u8]) {
        self.send_data(EndpointType::Device, Some(&data));
    }

    fn correct_transfer(&mut self) {
        // USB_ISTR_CTR is read only and will be automatically cleared by
        // hardware when we've processed all endpoint results.
        while self.hw.is_interrupt_active(UsbInterrupt::CorrectTransfer) {
            let transaction = self.hw.transaction();
            match &transaction.endpoint {
                EndpointType::Control => match &transaction.direction {
                    EndpointDirection::Receive => self.handle_control_out_transfer(&transaction),
                    EndpointDirection::Transmit => self.handle_control_in_transfer(&transaction),
                },
                EndpointType::Device => match (&transaction.direction, &transaction.flags) {
                    (EndpointDirection::Receive, TransactionFlags { rx: true, .. }) => {
                        self.handle_device_out_transfer(&transaction);
                    }
                    (EndpointDirection::Transmit, TransactionFlags { tx: true, .. }) => {
                        self.handle_device_in_transfer(&transaction);
                    }
                    _ => {}
                },
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
        let setup_packet_length = self.pma.rx_count(transaction.endpoint);
        let setup_packet = SetupPacket::from((
            self.pma.read(transaction.endpoint, 0),
            self.pma.read(transaction.endpoint, 2),
            self.pma.read(transaction.endpoint, 4),
            self.pma.read(transaction.endpoint, 6),
        ));

        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);

        self.update_control_endpoint_status(ControlEndpointStatus::Setup(setup_packet_length));

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

        let _command_packet_length = self.pma.rx_count(transaction.endpoint);
        let command_packet = CommandPacket::from((
            self.pma.read(transaction.endpoint, 0),
            self.pma.read(transaction.endpoint, 2),
            self.pma.read(transaction.endpoint, 4),
        ));

        self.state.command = Some(command_packet);

        self.pma.set_rx_count(transaction.endpoint, 0);
        self.hw.set_endpoint_status(
            transaction.endpoint,
            EndpointDirection::Receive,
            EndpointStatus::Valid,
        );
    }

    fn handle_device_in_transfer(&self, transaction: &Transaction) {
        self.hw
            .mark_transaction_as_handled(transaction.endpoint, transaction.direction);
    }

    fn update_address(&mut self, address: u8) {
        if address == 0 {
            self.hw.enable();
        }

        self.state.address = address;
    }

    fn send_data(&self, endpoint_type: EndpointType, data: Option<&[u8]>) {
        let length = data
            .and_then(|d| {
                self.pma.write(endpoint_type, &d);
                Some(d)
            })
            .map_or(0, |d| d.len() as u16);

        // Now that the PMA memory is prepared, set the length and tell the peripheral to send it.
        self.pma.set_tx_count(endpoint_type, length);
        self.hw.set_endpoint_status(
            endpoint_type,
            EndpointDirection::Transmit,
            EndpointStatus::Valid,
        );
    }

    fn send_control_data(&mut self, data: Option<&[u8]>) {
        self.update_control_endpoint_status(ControlEndpointStatus::DataIn);
        self.send_data(EndpointType::Control, data);
    }

    fn send_control_zero_length_packet(&mut self) {
        self.update_control_endpoint_status(ControlEndpointStatus::StatusIn);
        self.send_data(EndpointType::Control, None);
    }

    fn reset(&mut self) {
        self.hw.mark_interrupt_as_handled(UsbInterrupt::Reset);

        self.update_address(0);
        self.hw.open_endpoint(EndpointType::Control);
    }

    fn update_device_status(&mut self, device_status: DeviceStatus) {
        match (self.state.device_status, self.state.suspended_device_status) {
            (DeviceStatus::Suspended, _) => {
                self.state.device_status = device_status;
                self.state.suspended_device_status = Some(self.state.device_status);
            }
            (DeviceStatus::WokenUp, Some(previous_device_status)) => {
                self.state.device_status = previous_device_status;
                self.state.suspended_device_status = None;
            }
            (DeviceStatus::WokenUp, None) => {}
            _ => self.state.device_status = device_status,
        }
    }

    fn update_control_endpoint_status(&mut self, control_endpoint_status: ControlEndpointStatus) {
        if let ControlEndpointStatus::Setup(data_length) = control_endpoint_status {
            self.state.setup_data_length = data_length;
        }

        self.state.control_endpoint_status = control_endpoint_status;
    }

    fn stall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        if endpoint_index == 0 {
            self.control_endpoint_error();
        } else {
            let endpoint = match endpoint_index {
                0 => EndpointType::Control,
                1 => EndpointType::Device,
                _ => return,
            };

            let direction = if endpoint_address & 0x80 == 0x80 {
                EndpointDirection::Transmit
            } else {
                EndpointDirection::Receive
            };

            self.hw
                .set_endpoint_status(endpoint, direction, EndpointStatus::Stall);
        }
    }

    fn unstall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        let endpoint = match endpoint_index {
            0 => EndpointType::Control,
            1 => EndpointType::Device,
            _ => return,
        };

        let direction = if endpoint_index == 0 || endpoint_address & 0x80 == 0x80 {
            EndpointDirection::Transmit
        } else {
            EndpointDirection::Receive
        };

        self.hw
            .set_endpoint_status(endpoint, direction, EndpointStatus::Stall);
    }

    fn handle_endpoint_request(&mut self, request_header: SetupPacket) {
        if let RequestKind::Class = request_header.kind {
            self.handle_setup(request_header);
            return;
        }

        let endpoint_address = request_header.index as u8;
        match request_header.request {
            Request::SetFeature => {
                match self.state.device_status {
                    DeviceStatus::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceStatus::Configured => {
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
            Request::ClearFeature => {
                match self.state.device_status {
                    DeviceStatus::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceStatus::Configured => {
                        // USB_FEATURE_EP_HALT
                        if request_header.value == 0 && endpoint_address & 0x7f != 0 {
                            self.unstall_endpoint(endpoint_address);
                            self.handle_setup(request_header);
                        }
                    }
                    _ => self.control_endpoint_error(),
                }
            }
            Request::GetStatus => {
                match self.state.device_status {
                    DeviceStatus::Addressed => {
                        if endpoint_address & 0x7f != 0 {
                            self.stall_endpoint(endpoint_address);
                        }
                    }
                    DeviceStatus::Configured => {
                        // SHOULD BE:  status=isStalled(ep_addr) ? 1 : 0; sendControlData(&status,2);
                        self.send_control_data(Some([0x0, 0x0].as_ref()));
                    }
                    _ => self.control_endpoint_error(),
                }
            }
            _ => {}
        }
    }

    fn handle_setup(&mut self, request_header: SetupPacket) {
        match request_header.kind {
            RequestKind::Class => self.handle_class_setup(request_header),
            RequestKind::Standard => self.handle_standard_setup(request_header),
            _ => {}
        }
    }

    fn handle_class_setup(&mut self, request_header: SetupPacket) {
        match request_header.request {
            // CUSTOM_HID_REQ_SET_PROTOCOL
            Request::SetInterface => {
                self.state.protocol = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_PROTOCOL
            Request::SetFeature => {
                let protocol = [self.state.protocol];
                self.send_control_data(Some(protocol.as_ref()))
            }
            // CUSTOM_HID_REQ_SET_IDLE
            Request::GetInterface => {
                self.state.idle_state = (request_header.value >> 8) as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_IDLE
            Request::Two => {
                let idle_state = [self.state.idle_state];
                self.send_control_data(Some(idle_state.as_ref()))
            }
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
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_standard_setup(&mut self, request_header: SetupPacket) {
        match request_header.request {
            Request::GetDescriptor => {
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
            Request::SetInterface => {
                self.state.alt_setting = request_header.value as u8;
                self.send_control_zero_length_packet();
            }
            Request::GetInterface => {
                let alt_setting = [self.state.alt_setting];
                self.send_control_data(Some(alt_setting.as_ref()))
            }
            _ => self.control_endpoint_error(),
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
        let data_to_send: Option<&[u8]> = match (&request_header.value >> 8) as u16 {
            1 => Some(&DEV_DESC),
            2 => Some(&CONF_DESC),
            3 => self.descriptor_string(&request_header),
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

    fn descriptor_string(&self, request_header: &SetupPacket) -> Option<&'static [u8]> {
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

        if configuration_index > 1 {
            self.control_endpoint_error();
        } else {
            match self.state.device_status {
                DeviceStatus::Addressed => {
                    if configuration_index != 0 {
                        self.hw.open_endpoint(EndpointType::Device);
                        self.send_control_zero_length_packet();
                        self.update_device_status(DeviceStatus::Configured);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                DeviceStatus::Configured => {
                    if configuration_index == 0 {
                        self.hw.close_endpoint(EndpointType::Device);
                        self.send_control_zero_length_packet();
                        self.update_device_status(DeviceStatus::Addressed);
                    } else {
                        self.send_control_zero_length_packet();
                    }
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_configuration(&mut self, request_header: SetupPacket) {
        if request_header.length != 1 {
            self.control_endpoint_error();
        } else {
            match self.state.device_status {
                DeviceStatus::Addressed => {
                    self.state.configuration_index = 0;
                    self.send_control_data(Some([0].as_ref()));
                }
                DeviceStatus::Configured => {
                    let configuration_index = [self.state.configuration_index];
                    self.send_control_data(Some(configuration_index.as_ref()));
                }
                _ => self.control_endpoint_error(),
            }
        }
    }

    fn handle_get_status(&mut self) {
        match self.state.device_status {
            DeviceStatus::Addressed | DeviceStatus::Configured => {
                self.send_control_data(Some([3].as_ref()))
            }
            _ => {}
        }
    }

    fn handle_set_feature(&mut self, request_header: SetupPacket) {
        if request_header.value == 1 {
            // ACK
            self.send_control_zero_length_packet();
        }
    }

    fn handle_clear_feature(&mut self, request_header: SetupPacket) {
        match self.state.device_status {
            DeviceStatus::Addressed | DeviceStatus::Configured => {
                if request_header.value == 1 {
                    // ACK
                    self.send_control_zero_length_packet();
                }
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_interface_request(&mut self, request_header: SetupPacket) {
        match self.state.device_status {
            DeviceStatus::Configured if (request_header.index & 0xff) <= 1 => {
                self.handle_setup(request_header);
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
