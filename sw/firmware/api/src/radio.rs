pub mod commands;
mod constants;
pub mod errors;
pub mod pipes;
pub mod registers;

pub use self::constants::MAX_PAYLOAD_SIZE;

use array::Array;
use bare_metal::CriticalSection;
use radio::{
    commands::Command,
    constants::{MAX_REGISTER_VALUE_SIZE, PAYLOAD_SIZE, PIPE_COUNT},
    pipes::{
        Pipe0Register, Pipe1Register, Pipe2Register, Pipe3Register, Pipe4Register, Pipe5Register,
    },
    registers::{
        config::{ConfigRegister, ConfigRegisterCRCO, ConfigRegisterPrim},
        en_aa::AutoAcknowledgmentRegister,
        en_rx_addr::RxPipesStatusesRegister,
        fifo_status::FIFOStatusRegister,
        rf_ch::RFCHRegister,
        rf_setup::{RFSetupRegister, RFSetupRegisterRFDR, RFSetupRegisterRFPower},
        rx_addr::RxPipeAddressRegister,
        rx_pw::RxPipePayloadWidthRegister,
        setup_aw::SetupAWRegister,
        setup_retr::SetupRetrRegister,
        status::StatusRegister,
        tx_addr::TxAddrRegister,
        Register,
    },
};
use systick::{SysTick, SysTickHardware};

/// Describes the Radio hardware management interface.
pub trait RadioHardware {
    /// Initializes hardware if needed.
    fn setup(&mut self, cs: &CriticalSection);

    /// Transmits the specified buffer via the radio interface (SPI).
    fn transfer(&mut self, payload: Array<u8>) -> Result<Array<u8>, ()>;

    /// Activates RX or TX mode.
    fn enable_chip(&mut self) -> Result<(), ()>;

    /// Deactivates RX or TX mode.
    fn disable_chip(&mut self) -> Result<(), ()>;

    /// Releases hardware if needed.
    fn teardown(&mut self, cs: &CriticalSection);
}

pub struct Radio<'a, T: RadioHardware, S: SysTickHardware> {
    hw: &'a mut T,
    systick: &'a mut SysTick<S>,
    config: ConfigRegister,
}

impl<'a, T: RadioHardware, S: SysTickHardware> Radio<'a, T, S> {
    pub fn new(hw: &'a mut T, systick: &'a mut SysTick<S>) -> Self {
        Radio {
            hw,
            systick,
            config: ConfigRegister::default(),
        }
    }

    pub fn receive(&mut self, cs: &CriticalSection) -> Result<Array<u8>, ()> {
        self.hw.setup(cs);

        self.configure(ConfigRegisterPrim::Receiver)
            .and_then(|_| {
                self.hw.enable_chip().unwrap();

                self.systick.delay(1000);

                let can_receive = self.read_register::<StatusRegister>()?.rx_p_no().is_some();
                if can_receive {
                    self.command(Command::ReadRxPayload, [0xFF; PAYLOAD_SIZE])
                        .map(|(_, payload)| payload)
                } else {
                    Ok(Array::<u8>::new())
                }
            })
            .and_then(|payload| {
                self.power_down()?;

                self.hw.teardown(cs);
                Ok(payload)
            })
            .or_else(|_| {
                self.hw.teardown(cs);
                Err(())
            })
    }

    pub fn transmit(&mut self, cs: &CriticalSection, data: Array<u8>) -> Result<(), ()> {
        // We cannot send payload that's different from what's configured.
        if data.len() != PAYLOAD_SIZE {
            return Err(());
        }

        self.hw.setup(cs);

        let operation_result = self
            .configure(ConfigRegisterPrim::Transmitter)
            .map(|status| status.tx_full())
            .and_then(|can_send| if !can_send { self.flush_tx() } else { Ok(()) })
            .and_then(|_| self.command(Command::WriteTxPayload, data))
            .and_then(|(mut status, _)| {
                // Make a 1ms CE pulse so that radio enters StandBy 1 mode right after successful transmission.
                self.hw.enable_chip().unwrap();
                self.systick.delay(1);
                self.hw.disable_chip().unwrap();

                self.write_register(status.clear_max_rt().clear_tx_ds())
            })
            .and_then(|_| self.power_down().map(|_| ()));

        self.hw.teardown(cs);

        operation_result
    }

    pub fn status(&mut self, cs: &CriticalSection) -> Result<Array<u8>, ()> {
        self.hw.setup(cs);

        let operation_result = self
            .configure(ConfigRegisterPrim::Transmitter)
            .and_then(|_| {
                let mut status = Array::<u8>::new();
                self.read_register::<TxAddrRegister>()?
                    .raw()
                    .iter()
                    .chain(
                        self.read_register::<RxPipeAddressRegister<Pipe0Register>>()?
                            .raw()
                            .iter(),
                    )
                    .chain(
                        [
                            self.read_register::<ConfigRegister>()?.raw(),
                            self.read_register::<AutoAcknowledgmentRegister>()?.raw(),
                            self.read_register::<RxPipesStatusesRegister>()?.raw(),
                            self.read_register::<SetupAWRegister>()?.raw(),
                            self.read_register::<SetupRetrRegister>()?.raw(),
                            self.read_register::<RFCHRegister>()?.raw(),
                            self.read_register::<RFSetupRegister>()?.raw(),
                            self.read_register::<StatusRegister>()?.raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe0Register>>()?
                                .raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe1Register>>()?
                                .raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe2Register>>()?
                                .raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe3Register>>()?
                                .raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe4Register>>()?
                                .raw(),
                            self.read_register::<RxPipePayloadWidthRegister<Pipe5Register>>()?
                                .raw(),
                            self.read_register::<FIFOStatusRegister>()?.raw(),
                        ]
                        .iter()
                        .flatten(),
                    )
                    .for_each(|r| status.push(*r));

                Ok(status)
            });

        self.hw.teardown(cs);

        operation_result
    }

    fn configure(&mut self, mode: ConfigRegisterPrim) -> Result<StatusRegister, ()> {
        self.read_register::<SetupAWRegister>()
            .and_then(|setup| {
                let is_connected = !setup.aw().is_illegal();
                if is_connected {
                    // Set CRC to 16 bits.
                    self.config
                        .set_en_crc(true)
                        .set_crco(ConfigRegisterCRCO::TwoBytes)
                        .set_prim_rx(mode);

                    self.write_register(&self.config.clone())
                } else {
                    Err(())
                }
            })
            // Power radio up.
            .and_then(|_| self.power_up())
            // Pick Channel/frequency `100`.
            .and_then(|_| self.write_register(RFCHRegister::default().set_rf_ch(100)))
            // Set transmission address (5 bytes).
            .and_then(|_| {
                if let ConfigRegisterPrim::Transmitter = &mode {
                    self.write_register(&TxAddrRegister::from_raw([0x11, 0x11, 0x11, 0x11, 0x11]))
                } else {
                    self.write_register(&RxPipeAddressRegister::<Pipe0Register>::from_raw([
                        0x11, 0x11, 0x11, 0x11, 0x11,
                    ]))
                }
            })
            // Disable auto-retransmit.
            .and_then(|_| self.write_register(SetupRetrRegister::default().set_ard(0).set_arc(0)))
            // Set max power and 250Kbps data rate.
            .and_then(|_| {
                self.write_register(
                    RFSetupRegister::default()
                        .set_rf_pwr(RFSetupRegisterRFPower::Highest)
                        .set_rf_dr(RFSetupRegisterRFDR::DataRate250Kbps),
                )
            })
            // Disable auto ACK for all pipes.
            .and_then(|_| {
                self.write_register(
                    AutoAcknowledgmentRegister::default().set_all(&[false; PIPE_COUNT]),
                )
            })
            // Enable only first pipe.
            .and_then(|_| {
                self.write_register(
                    RxPipesStatusesRegister::default()
                        .set_all(&[true, false, false, false, false, false]),
                )
            })
            // Set Pipe0 payload width to 6.
            .and_then(|_| {
                self.write_register(
                    RxPipePayloadWidthRegister::<Pipe0Register>::default().set(PAYLOAD_SIZE as u8),
                )
            })
    }

    fn power_up(&mut self) -> Result<(), ()> {
        let new_config = self.config.set_pwr_up(true).clone();
        self.write_register(&new_config).map(|_status| {})
    }

    fn power_down(&mut self) -> Result<(), ()> {
        let new_config = self.config.set_pwr_up(false).clone();
        self.write_register(&new_config).map(|_status| {})
    }

    fn _flush_rx(&mut self) -> Result<(), ()> {
        self.command(Command::FlushRx, []).map(|_| {})
    }

    fn flush_tx(&mut self) -> Result<(), ()> {
        self.command(Command::FlushTx, []).map(|_| {})
    }

    fn read_register<R: Register>(&mut self) -> Result<R, ()> {
        self.command(Command::ReadRegister(R::address()), R::TRaw::default())
            .and_then(|(_, result)| {
                if result.len() != core::mem::size_of::<R::TRaw>() {
                    Err(())
                } else {
                    let mut buffer = R::TRaw::default();
                    buffer.as_mut().copy_from_slice(result.as_ref());
                    Ok(R::from_raw(buffer))
                }
            })
    }

    fn write_register<R: Register>(&mut self, register: &R) -> Result<StatusRegister, ()> {
        self.command(Command::WriteRegister(R::address()), register.raw())
            .map(|(status, _)| status)
    }

    fn command<P: AsRef<[u8]>>(
        &mut self,
        command: Command,
        payload: P,
    ) -> Result<(StatusRegister, Array<u8>), ()> {
        if payload.as_ref().len() > MAX_PAYLOAD_SIZE {
            return Err(());
        }

        let mut command_with_payload = Array::<u8>::new();
        command_with_payload.push(command.into());
        payload
            .as_ref()
            .iter()
            .for_each(|value| command_with_payload.push(*value));

        self.hw
            .transfer(command_with_payload)
            .and_then(|mut result| {
                if let Some(status_raw) = result.shift() {
                    Ok((StatusRegister::from_raw([status_raw]), result))
                } else {
                    Err(())
                }
            })
    }
}
