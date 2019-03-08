pub mod command_packet;
pub mod descriptors;
pub mod pma;
pub mod setup_packet;

#[derive(Copy, Clone)]
pub enum EndpointType {
    Control = 0b0,
    Device = 0b1,
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
    // Synthetic status for the woken up device,
    WokenUp,
}

// The possible statuses for the control endpoint.
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
}

impl Default for UsbState {
    fn default() -> Self {
        UsbState {
            device_status: DeviceStatus::None,
            suspended_device_status: None,
            control_endpoint_status: ControlEndpointStatus::Idle,
            setup_data_length: 0,
            address: 0,
            configuration_index: 0,
            protocol: 0,
            idle_state: 0,
            alt_setting: 0,
        }
    }
}
