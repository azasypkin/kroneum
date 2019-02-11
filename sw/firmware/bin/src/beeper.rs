use crate::{config, systick::SysTick, Peripherals};

const EIGHTH_NOTE: u32 = 150;
const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;

const SCALES: [u32; 12] = [523, 554, 587, 622, 659, 698, 740, 784, 831, 880, 932, 988];

pub struct Beeper<'a> {
    p: &'a mut Peripherals,
    phantom: core::marker::PhantomData<()>,
}

impl<'a> Beeper<'a> {
    pub fn acquire<'b, F>(p: &'a mut Peripherals, f: F) -> ()
    where
        F: FnOnce(&mut Beeper),
    {
        let mut beeper = Beeper {
            p,
            phantom: core::marker::PhantomData,
        };

        beeper.setup();
        f(&mut beeper);
        beeper.teardown();
    }

    pub fn play_melody(&mut self) {
        self.toggle_pwm(true);

        self.play_note(SCALES[7], QUARTER_NOTE); // G
        self.play_note(SCALES[7], QUARTER_NOTE); // G
        self.play_note(SCALES[8], QUARTER_NOTE); // A
        self.play_note(SCALES[10], QUARTER_NOTE); // B
        self.play_note(SCALES[10], QUARTER_NOTE); // B
        self.play_note(SCALES[8], QUARTER_NOTE); // A
        self.play_note(SCALES[7], QUARTER_NOTE); // G
        self.play_note(SCALES[5], QUARTER_NOTE); // F
        self.play_note(SCALES[3], QUARTER_NOTE); // D#
        self.play_note(SCALES[3], QUARTER_NOTE); // E
        self.play_note(SCALES[5], QUARTER_NOTE); // F
        self.play_note(SCALES[7], QUARTER_NOTE); // G
        self.play_note(SCALES[7], QUARTER_DOT_NOTE); // G.
        self.play_note(SCALES[5], EIGHTH_NOTE); // F
        self.play_note(SCALES[5], QUARTER_DOT_NOTE); // F.

        self.toggle_pwm(false);
    }

    pub fn play_setup(&mut self) {
        self.toggle_pwm(true);

        self.play_note(SCALES[3], QUARTER_NOTE); // D#
        self.play_note(SCALES[3], QUARTER_NOTE); // E

        self.toggle_pwm(false);
    }

    pub fn play_reset(&mut self) {
        self.toggle_pwm(true);

        self.play_note(SCALES[5], QUARTER_NOTE); // F
        self.play_note(SCALES[5], EIGHTH_NOTE); // F
        self.play_note(SCALES[7], QUARTER_DOT_NOTE); // G.
        self.play_note(SCALES[5], QUARTER_NOTE); // F
        self.play_note(SCALES[5], EIGHTH_NOTE); // F
        self.play_note(SCALES[7], QUARTER_DOT_NOTE); // G.

        self.toggle_pwm(false);
    }

    pub fn beep(&mut self) {
        self.beep_n(1);
    }

    pub fn beep_n(&mut self, n: u8) {
        for i in 1..n + 1 {
            self.toggle_pwm(true);
            self.play_note(SCALES[7], EIGHTH_NOTE);
            self.toggle_pwm(false);

            if i < n {
                SysTick::delay_ms(&mut self.p.core.SYST, 100);
            }
        }
    }

    fn setup(&self) {
        // Enable TIM1 clock.
        self.p
            .device
            .RCC
            .apb2enr
            .modify(|_, w| w.tim1en().set_bit());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        self.p.device.TIM1.psc.modify(|_, w| unsafe { w.bits(0b0) });

        // Set direction: counter used as up-counter and clock division to t(DTS) = t(CK_INT).
        self.p
            .device
            .TIM1
            .cr1
            .modify(|_, w| unsafe { w.dir().clear_bit().ckd().bits(0b00) });

        // Compute the value to be set in ARR (auto-reload) register to generate signal frequency at 17.57 Khz.
        let timer_period: u32 = (config::CLOCK_SPEED / 17_570) - 1;
        self.p
            .device
            .TIM1
            .arr
            .write(|w| unsafe { w.bits(timer_period) });

        // Set repetition counter.
        self.p.device.TIM1.rcr.write(|w| unsafe { w.bits(0b0) });

        // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
        // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
        //inactive.
        self.p
            .device
            .TIM1
            .ccmr1_output
            .modify(|_, w| unsafe { w.oc1m().bits(0b111) });

        // Configure capture/compare enable register.
        self.p.device.TIM1.ccer.modify(|_, w| {
            // Enable Capture/Compare 1 output.
            w.cc1e()
                .set_bit()
                // Enable Capture/Compare 1 complementary output.
                .cc1ne()
                .set_bit()
                // Set low polarity for Capture/Compare 1 output.
                .cc1p()
                .set_bit()
                // Set high polarity for Capture/Compare complementary 1 output.
                .cc1np()
                .clear_bit()
        });

        // Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N. CCR1 is the
        // value to be loaded in the actual capture/compare 1 register (preload value).
        let channel_one_pulse: u32 = (5 * (timer_period - 1)) / 10;
        self.p
            .device
            .TIM1
            .ccr1
            .write(|w| unsafe { w.bits(channel_one_pulse) });

        // Configure control register 2.
        self.p.device.TIM1.cr2.modify(|_, w| {
            // Set output Idle state 1 (OC1 output and OC1N output).
            w.ois1().set_bit().ois1n().clear_bit()
        });

        // Enable counter.
        self.p.device.TIM1.cr1.modify(|_, w| w.cen().set_bit());
    }

    fn teardown(&self) {
        // Disable counter.
        self.p.device.TIM1.cr1.modify(|_, w| w.cen().clear_bit());

        // Disable TIM1 clock.
        self.p
            .device
            .RCC
            .apb2enr
            .modify(|_, w| w.tim1en().clear_bit());
    }

    fn play_note(&mut self, note: u32, delay: u32) {
        self.p
            .device
            .TIM1
            .arr
            .write(|w| unsafe { w.bits((config::CLOCK_SPEED / note) - 1) });

        SysTick::delay_ms(&mut self.p.core.SYST, delay);
    }

    fn toggle_pwm(&self, enable: bool) {
        self.p.device.TIM1.bdtr.modify(|_, w| {
            if enable {
                w.moe().set_bit()
            } else {
                w.moe().clear_bit()
            }
        });
    }
}
