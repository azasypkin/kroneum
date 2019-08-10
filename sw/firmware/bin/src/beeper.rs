use kroneum_api::{beeper::PWMBeeperHardware, config};
use stm32f0::stm32f0x2::Peripherals;

pub struct BeeperHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> BeeperHardwareImpl<'a> {
    pub fn new(p: &'a Peripherals) -> Self {
        Self { p }
    }
}

impl<'a> PWMBeeperHardware for BeeperHardwareImpl<'a> {
    fn toggle_pwm(&self, enable: bool) {
        if enable {
            self.setup();
        }

        self.p.TIM1.bdtr.modify(|_, w| w.moe().bit(enable));

        if !enable {
            self.teardown();
        }
    }

    fn pulse(&self, note_frequency: u32) {
        self.p
            .TIM1
            .arr
            .write(|w| unsafe { w.bits((config::CLOCK_SPEED / note_frequency) - 1) });
    }
}

impl<'a> BeeperHardwareImpl<'a> {
    fn setup(&self) {
        // Enable TIM1 clock.
        self.p.RCC.apb2enr.modify(|_, w| w.tim1en().set_bit());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        self.p.TIM1.psc.reset();

        // Set direction: counter used as up-counter and clock division to t(DTS) = t(CK_INT).
        self.p.TIM1.cr1.reset();

        // Compute the value to be set in ARR (auto-reload) register to generate signal frequency at 17.57 Khz.
        let timer_period: u32 = (config::CLOCK_SPEED / 17_570) - 1;
        self.p.TIM1.arr.write(|w| unsafe { w.bits(timer_period) });

        // Set repetition counter.
        self.p.TIM1.rcr.reset();

        // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
        // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
        //inactive.
        self.p
            .TIM1
            .ccmr1_output()
            .modify(|_, w| w.oc1m().bits(0b111));

        // Configure capture/compare enable register.
        self.p.TIM1.ccer.modify(|_, w| {
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
            .TIM1
            .ccr1
            .write(|w| unsafe { w.bits(channel_one_pulse) });

        // Configure control register 2.
        self.p.TIM1.cr2.modify(|_, w| {
            // Set output Idle state 1 (OC1 output and OC1N output).
            w.ois1().set_bit().ois1n().clear_bit()
        });

        // Enable counter.
        self.p.TIM1.cr1.modify(|_, w| w.cen().set_bit());
    }

    fn teardown(&self) {
        // Disable counter.
        self.p.TIM1.cr1.modify(|_, w| w.cen().clear_bit());

        // Disable TIM1 clock.
        self.p.RCC.apb2enr.modify(|_, w| w.tim1en().clear_bit());
    }
}
