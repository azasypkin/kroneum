pub mod pma;
mod setup_packet;
mod descriptors;

use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::{Interrupt, Peripherals};

use pma::PacketMemoryArea;
use setup_packet::{Request, RequestKind, RequestRecipient, SetupPacket};
use descriptors::*;


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
                1 => {
                    if is_out && self.peripherals.USB.ep1r.read().ctr_rx().bit_is_set() {
                        self.handle_device_out_transfer();
                    } else if !is_out && self.peripherals.USB.ep1r.read().ctr_tx().bit_is_set() {
                        self.handle_device_in_transfer();
                    }
                }
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
        let endpoint_type = EndpointType::Control;

        let setup_packet_length = self.pma.get_rx_count(endpoint_type);
        let setup_packet = SetupPacket::from((
            self.pma.read(endpoint_type, 0),
            self.pma.read(endpoint_type, 2),
            self.pma.read(endpoint_type, 4),
            self.pma.read(endpoint_type, 6),
        ));

        // Clear the 'correct transfer for reception' bit for this endpoint.
        self.peripherals.USB.ep0r.modify(|_, w| unsafe {
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

        match setup_packet.recipient {
            RequestRecipient::Device => self.handle_device_request(setup_packet),
            RequestRecipient::Interface => self.handle_interface_request(setup_packet),
            RequestRecipient::Endpoint => self.handle_endpoint_request(setup_packet),
            _ => self.set_rx_endpoint_status(endpoint_type, EndpointStatus::Stall),
        }
    }

    fn handle_control_data_out_transfer(&self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        self.peripherals.USB.ep0r.modify(|_, w| unsafe {
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
        self.set_rx_endpoint_status(EndpointType::Control, EndpointStatus::Valid);
    }

    fn handle_control_in_transfer(&mut self) {
        // Clear the 'correct transfer for reception' bit for this endpoint.
        self.peripherals.USB.ep0r.modify(|_, w| unsafe {
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
            self.set_rx_endpoint_status(EndpointType::Control, EndpointStatus::Valid);
        }

        if self.state.address > 0 {
            self.peripherals
                .USB
                .daddr
                .write(|w| unsafe { w.add().bits(self.state.address).ef().set_bit() });
            self.state.address = 0;
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

    fn get_descriptor_string(&self, request_header: &SetupPacket) -> Option<&'static [u8]> {
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

    fn handle_set_configuration(&mut self, request_header: SetupPacket) {
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

    fn handle_get_configuration(&mut self, request_header: SetupPacket) {
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

    fn handle_set_feature(&mut self, request_header: SetupPacket) {
        if request_header.value == 1 {
            // ACK
            self.send_control_zero_length_packet();
        }
    }

    fn handle_clear_feature(&mut self, request_header: SetupPacket) {
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

    fn handle_interface_request(&mut self, request_header: SetupPacket) {
        match self.state.device_state {
            DeviceState::Configured if (request_header.index & 0xff) <= 1 => {
                self.handle_setup(request_header);
            }
            _ => self.control_endpoint_error(),
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
            Request::SetFeature => self.send_control_data(Some([self.state.protocol].as_ref())),
            // CUSTOM_HID_REQ_SET_IDLE
            Request::GetInterface => {
                self.state.idle_state = (request_header.value >> 8) as u8;
                self.send_control_zero_length_packet();
            }
            // CUSTOM_HID_REQ_GET_IDLE
            Request::Two => self.send_control_data(Some([self.state.idle_state].as_ref())),
            // CUSTOM_HID_REQ_SET_REPORT
            Request::SetConfiguration => {
                self.update_control_endpoint_state(ControlEndpointState::DataOut);
                self.pma
                    .set_rx_count(EndpointType::Control, request_header.length);
                self.set_rx_endpoint_status(EndpointType::Control, EndpointStatus::Valid);
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
                self.send_control_data(Some([self.state.alt_setting].as_ref()))
            }
            _ => self.control_endpoint_error(),
        }
    }

    fn handle_endpoint_request(&mut self, request_header: SetupPacket) {
        if let RequestKind::Class = request_header.kind {
            self.handle_setup(request_header);
            return;
        }

        let endpoint_address = request_header.index as u8;
        match request_header.request {
            Request::SetFeature => {
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
            Request::ClearFeature => {
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
            Request::GetStatus => {
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
        self.peripherals.USB.ep1r.modify(|_, w| unsafe {
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
        self.set_rx_endpoint_status(EndpointType::Device, EndpointStatus::Valid);
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
                0 => EndpointType::Control,
                1 => EndpointType::Device,
                _ => panic!("Unknown endpoint"),
            };

            if endpoint_address & 0x80 == 0x80 {
                self.set_tx_endpoint_status(endpoint, EndpointStatus::Stall);
            } else {
                self.set_rx_endpoint_status(endpoint, EndpointStatus::Stall);
            }
        }
    }

    fn unstall_endpoint(&self, endpoint_address: u8) {
        let endpoint_index = endpoint_address & 0x7f;
        let endpoint = match endpoint_index {
            0 => EndpointType::Control,
            1 => EndpointType::Device,
            _ => panic!("Unknown endpoint"),
        };

        if endpoint_index == 0 || endpoint_address & 0x80 == 0x80 {
            self.set_tx_endpoint_status(endpoint, EndpointStatus::Stall);
        } else if endpoint_address & 0x80 == 0x0 {
            self.set_rx_endpoint_status(endpoint, EndpointStatus::Stall);
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
        self.pma.set_tx_count(endpoint_type, length);
        self.set_tx_endpoint_status(endpoint_type, EndpointStatus::Valid);
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
        });
    }

    fn open_device_endpoints(&self) {
        self.peripherals.USB.ep1r.modify(|r, w| unsafe {
            w.ep_type()
                .bits(0b11)
                .ea()
                .bits(0x1)
                .stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Nak))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Valid))
        });
    }

    fn close_control_endpoints(&self) {
        self.peripherals.USB.ep0r.modify(|r, w| unsafe {
            w.stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn close_device_endpoints(&self) {
        self.peripherals.USB.ep1r.modify(|r, w| unsafe {
            w.stat_tx()
                .bits(self.get_status_bits(r.stat_tx().bits(), EndpointStatus::Disabled))
                .stat_rx()
                .bits(self.get_status_bits(r.stat_rx().bits(), EndpointStatus::Disabled))
        });
    }

    fn get_status_bits(&self, current_bits: u8, status: EndpointStatus) -> u8 {
        return current_bits ^ status as u8;
    }

    fn set_rx_endpoint_status(&self, endpoint: EndpointType, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            EndpointType::Control => {
                self.peripherals.USB.ep0r.modify(|r, w| unsafe {
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
            EndpointType::Device => self.peripherals.USB.ep1r.modify(|r, w| unsafe {
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
            }),
        }
    }

    fn set_tx_endpoint_status(&self, endpoint: EndpointType, status: EndpointStatus) {
        // If current reg bit is not equal to the desired reg bit then set 1 in the reg to toggle it.
        match endpoint {
            EndpointType::Control => {
                self.peripherals.USB.ep0r.modify(|r, w| unsafe {
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
            EndpointType::Device => self.peripherals.USB.ep1r.modify(|r, w| unsafe {
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
