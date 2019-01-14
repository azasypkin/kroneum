use crate::{
    beeper::Beeper,
    rtc::{Time, RTC},
    usb::{UsbState, USB},
    Peripherals,
};

#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Idle,
    Setup(u8),
    Alarm(Time),
    Config,
}

#[derive(Copy, Clone)]
pub struct SystemState {
    pub mode: SystemMode,
    usb_state: UsbState,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            usb_state: UsbState::default(),
        }
    }
}

pub struct System<'a> {
    p: &'a mut Peripherals,
    state: &'a mut SystemState,
}

impl<'a> System<'a> {
    pub fn new(p: &'a mut Peripherals, state: &'a mut SystemState) -> Self {
        System { p, state }
    }

    pub fn acquire<'b, F>(p: &'b mut Peripherals, state: &'b mut SystemState, f: F) -> ()
    where
        F: FnOnce(System),
    {
        f(System::new(p, state));
    }

    pub fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.toggle_standby_mode(true);

                USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
                    usb.teardown()
                });
            }
            SystemMode::Config => {
                Beeper::acquire(&mut self.p, |mut beeper| {
                    beeper.setup();
                    beeper.play_reset();
                    beeper.teardown();
                });

                self.toggle_standby_mode(false);

                USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
                    usb.setup()
                });
            }
            SystemMode::Setup(0) => {
                Beeper::acquire(&mut self.p, |mut beeper| {
                    beeper.setup();
                    beeper.play_setup();
                    beeper.teardown();
                });
            }
            SystemMode::Setup(c) if c > &0 => {
                Beeper::acquire(&mut self.p, |mut beeper| {
                    beeper.setup();
                    beeper.beep();
                    beeper.teardown();
                });
            }
            SystemMode::Alarm(time) => {
                Beeper::acquire(&mut self.p, |mut beeper| {
                    beeper.setup();
                    beeper.beep_n(time.minutes);
                    beeper.play_setup();
                    beeper.teardown();
                });

                RTC::acquire(&mut self.p, |mut rtc| {
                    rtc.setup();
                    rtc.configure_time(&Time::default());
                    rtc.configure_alarm(&time);
                });
            }
            _ => {}
        }

        self.state.mode = mode;
    }

    pub fn on_rtc_alarm(&mut self) {
        Beeper::acquire(&mut self.p, |mut beeper| {
            beeper.setup();
            beeper.play_melody();
            beeper.teardown();
        });

        RTC::acquire(&mut self.p, |mut rtc| rtc.teardown());

        self.set_mode(SystemMode::Idle);
    }

    pub fn on_usb_packet(&mut self) {
        USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
            usb.interrupt()
        });
    }

    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        self.p.device.PWR.cr.modify(|_, w| w.pdds().bit(on));

        self.p.device.PWR.cr.modify(|_, w| w.cwuf().set_bit());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.p.core.SCB.set_sleepdeep();
        } else {
            self.p.core.SCB.clear_sleepdeep();
        }
    }
}
