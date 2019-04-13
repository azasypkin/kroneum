use crate::{beeper, buttons, flash, rtc, usb};

use cortex_m::peripheral::SCB;
use stm32f0::stm32f0x2::Peripherals;

use kroneum_api::{
    beeper::{Melody, PWMBeeper, PWMBeeperHardware},
    buttons::{ButtonPressType, Buttons, ButtonsHardware},
    flash::{Flash, FlashHardware},
    rtc::{RTCHardware, RTC},
    system::{SystemMode, SystemState},
    systick::{SysTick, SysTickHardware},
    time::Time,
    usb::{command_packet::CommandPacket, USBHardware, USB},
};

pub struct System<S: SysTickHardware> {
    p: Peripherals,
    systick: SysTick<S>,
    scb: SCB,
    state: SystemState,
}

impl<S: SysTickHardware> System<S> {
    pub fn new(p: Peripherals, systick: SysTick<S>, scb: SCB) -> Self {
        System {
            p,
            state: SystemState::default(),
            systick,
            scb,
        }
    }

    pub fn setup(&mut self) {
        // Remap PA9-10 to PA11-12 for USB.
        self.p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        self.p
            .SYSCFG
            .cfgr1
            .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());

        // -----------Buttons----------------

        // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2.
        self.p
            .SYSCFG
            .exticr1
            .modify(|_, w| w.exti0().pa0().exti2().pa2());

        // Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge.
        self.p
            .EXTI
            .rtsr
            .modify(|_, w| w.tr0().set_bit().tr2().set_bit());

        // Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
        // EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
        self.p
            .EXTI
            .imr
            .modify(|_, w| w.mr0().set_bit().mr2().set_bit());

        // ---------GPIO------------------

        // Enable clock for GPIO Port A, B and F.
        self.p
            .RCC
            .ahbenr
            .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

        // Switch PA0 (button), PA2 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function
        // mode and PA1, PA3-6 to AIN to reduce power consumption.
        self.p.GPIOA.moder.modify(|_, w| {
            w.moder0()
                .alternate()
                .moder1()
                .analog()
                .moder2()
                .alternate()
                .moder3()
                .analog()
                .moder4()
                .analog()
                .moder5()
                .analog()
                .moder6()
                .analog()
                .moder7()
                .alternate()
                .moder11()
                .alternate()
                .moder12()
                .alternate()
        });

        // Enable AIN for GPIO B and F to reduce power consumption.
        self.p
            .GPIOB
            .moder
            .modify(|_, w| w.moder1().analog().moder8().analog());
        self.p
            .GPIOF
            .moder
            .modify(|_, w| w.moder0().analog().moder1().analog());

        self.p
            .RCC
            .ahbenr
            .modify(|_, w| w.iopben().clear_bit().iopfen().clear_bit());

        // Enable pull-down for PA0 and PA2.
        self.p
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

        // Set "high" output speed for PA7, PA11 and PA12.
        self.p.GPIOA.ospeedr.modify(|_, w| {
            w.ospeedr7()
                .very_high_speed()
                .ospeedr11()
                .very_high_speed()
                .ospeedr12()
                .very_high_speed()
        });

        // Set alternative function #2 for PA0 (WKUP1), PA2 (WKUP4) and PA7 (TIM1_CH1N).
        self.p
            .GPIOA
            .afrl
            .modify(|_, w| w.afrl0().af2().afrl2().af2().afrl7().af2());

        // Set alternative function #2 (USB) for PA11 and PA12.
        self.p
            .GPIOA
            .afrh
            .modify(|_, w| w.afrh11().af2().afrh12().af2());

        self.buttons().setup();

        self.set_mode(SystemMode::Idle);
    }

    pub fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.toggle_standby_mode(true);

                self.usb().teardown();
                self.rtc().teardown();

                // If we are exiting `Config` or `Alarm` mode let's play special signal.
                if let SystemMode::Setup(_) = self.state.mode {
                    self.beeper().play(Melody::Reset);
                } else if let SystemMode::Alarm(_, _) = self.state.mode {
                    self.beeper().play(Melody::Reset);
                }
            }
            SystemMode::Config => {
                self.beeper().play(Melody::Reset);

                self.toggle_standby_mode(false);

                self.usb().setup();
            }
            SystemMode::Setup(0) => self.beeper().play(Melody::Setup),
            SystemMode::Setup(c) if *c > 0 => self.beeper().beep(),
            SystemMode::Alarm(time, _) => {
                self.beeper().play(Melody::Setup);

                let rtc = self.rtc();
                rtc.setup();
                rtc.set_time(Time::default());
                rtc.set_alarm(*time);
            }
            _ => {}
        }

        self.state.mode = mode;
    }

    pub fn on_rtc_alarm(&mut self) {
        if let SystemMode::Alarm(_, melody) = self.state.mode {
            self.beeper().play(melody);

            self.rtc().teardown();

            // Snooze alarm for 10 seconds.
            self.set_mode(SystemMode::Alarm(Time::from_seconds(10), Melody::Beep));
        }
    }

    pub fn on_usb_packet(&mut self) {
        self.usb().interrupt();

        if let Some(command_packet) = self.state.usb_state.command {
            if let CommandPacket::Beep(num) = command_packet {
                self.beeper().beep_n(num);
            } else if let CommandPacket::AlarmSet(time) = command_packet {
                self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            } else if let CommandPacket::AlarmGet = command_packet {
                let alarm = self.rtc().alarm();
                self.usb()
                    .send(&[alarm.hours, alarm.minutes, alarm.seconds, 0, 0, 0]);
            } else if let CommandPacket::Reset = command_packet {
                self.reset();
            } else if let CommandPacket::FlashRead(slot) = command_packet {
                let value = self.flash().read(slot).unwrap_or_else(|| 0);
                self.usb().send(&[value, 0, 0, 0, 0, 0]);
            } else if let CommandPacket::FlashWrite(slot, value) = command_packet {
                let status = if self.flash().write(slot, value).is_ok() {
                    1
                } else {
                    0
                };
                self.usb().send(&[status, 0, 0, 0, 0, 0]);
            } else if let CommandPacket::FlashEraseAll = command_packet {
                self.flash().erase_all();
            }
        }

        self.state.usb_state.command = None;
    }

    pub fn on_button_press(&mut self) {
        if !buttons::has_pending_interrupt(&self.p) {
            return;
        }

        let (button_i, button_x) = self.buttons().interrupt();

        match (self.state.mode, button_i, button_x) {
            (mode, ButtonPressType::Long, ButtonPressType::Long) => {
                let (button_i, button_x) = self.buttons().interrupt();

                match (mode, button_i, button_x) {
                    (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long)
                    | (SystemMode::Alarm(_, _), ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Idle)
                    }
                    (_, ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Config)
                    }
                    (SystemMode::Setup(counter), _, _) => {
                        self.set_mode(SystemMode::Alarm(Time::from_hours(counter), Melody::Alarm))
                    }
                    _ => {}
                }
            }
            (SystemMode::Idle, ButtonPressType::Long, _)
            | (SystemMode::Idle, _, ButtonPressType::Long)
            | (SystemMode::Alarm(_, _), ButtonPressType::Long, _)
            | (SystemMode::Alarm(_, _), _, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Setup(0))
            }
            (SystemMode::Setup(counter), ButtonPressType::Long, _)
            | (SystemMode::Setup(counter), _, ButtonPressType::Long) => {
                let time = match button_i {
                    ButtonPressType::Long => Time::from_seconds(counter as u32),
                    _ => Time::from_minutes(counter as u32),
                };

                self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            }
            (SystemMode::Setup(counter), ButtonPressType::Short, _) => {
                self.set_mode(SystemMode::Setup(counter + 1))
            }
            (SystemMode::Setup(counter), _, ButtonPressType::Short) => {
                self.set_mode(SystemMode::Setup(counter + 10))
            }
            _ => {}
        }

        buttons::clear_pending_interrupt(&self.p);
    }

    /// Creates an instance of `Beeper` controller.
    fn beeper<'a>(
        &'a mut self,
    ) -> PWMBeeper<'a, impl PWMBeeperHardware + 'a, impl SysTickHardware> {
        beeper::create(&self.p, &mut self.systick)
    }

    /// Creates an instance of `Buttons` controller.
    fn buttons<'a>(&'a mut self) -> Buttons<impl ButtonsHardware + 'a> {
        buttons::create(&self.p, &mut self.systick)
    }

    /// Creates an instance of `RTC` controller.
    fn rtc<'a>(&'a self) -> RTC<impl RTCHardware + 'a> {
        rtc::create(&self.p)
    }

    /// Creates an instance of `USB` controller.
    fn usb<'a>(&'a mut self) -> USB<impl USBHardware + 'a> {
        usb::create(&self.p, &mut self.state.usb_state)
    }

    /// Creates an instance of `Flash` controller.
    fn flash<'a>(&'a mut self) -> Flash<impl FlashHardware + 'a> {
        flash::create(&self.p)
    }

    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        self.p.PWR.cr.modify(|_, w| w.pdds().bit(on));

        self.p.PWR.cr.modify(|_, w| w.cwuf().set_bit());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.scb.set_sleepdeep();
        } else {
            self.scb.clear_sleepdeep();
        }
    }

    /// Performs a software reset.
    fn reset(&mut self) {
        self.scb.system_reset();
    }
}
