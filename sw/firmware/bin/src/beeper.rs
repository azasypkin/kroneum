use crate::system::SystemHardwareImpl;
use kroneum_api::{beeper::PWMBeeperHardware, config};

impl PWMBeeperHardware for SystemHardwareImpl {
    fn enable_pwm(&self) {
        // Enable TIM1 clock.
        self.rcc.regs.apb2enr.modify(|_, w| w.tim1en().enabled());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        self.tim1.psc.reset();

        // Set direction: counter used as up-counter and clock division to t(DTS) = t(CK_INT).
        self.tim1.cr1.reset();

        // Compute the value to be set in ARR (auto-reload) register to generate signal frequency at 17.57 Khz.
        let timer_period = (config::CLOCK_SPEED / 17_570) as u16 - 1;
        self.tim1.arr.write(|w| w.arr().bits(timer_period));

        // Set repetition counter.
        self.tim1.rcr.reset();

        // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
        // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
        //inactive.
        self.tim1.ccmr2_output().modify(|_, w| w.oc3m().bits(0b111));

        // Configure capture/compare enable register.
        self.tim1.ccer.modify(|_, w| {
            // Enable Capture/Compare 1 output.
            w.cc3e()
                .set_bit()
                // Enable Capture/Compare 1 complementary output.
                .cc3ne()
                .set_bit()
                // Set low polarity for Capture/Compare 1 output.
                .cc3p()
                .set_bit()
                // Set high polarity for Capture/Compare complementary 1 output.
                .cc3np()
                .clear_bit()
        });

        // Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N. CCR1 is the
        // value to be loaded in the actual capture/compare 1 register (preload value).
        let channel_one_pulse = (5 * (timer_period - 1)) / 10;
        self.tim1.ccr3.write(|w| w.ccr().bits(channel_one_pulse));

        // Configure control register 2.
        self.tim1.cr2.modify(|_, w| {
            // Set output Idle state 1 (OC1 output and OC1N output).
            w.ois3().set_bit().ois3n().clear_bit()
        });

        // Enable counter.
        self.tim1.cr1.modify(|_, w| w.cen().enabled());

        self.tim1.bdtr.modify(|_, w| w.moe().enabled());
    }

    fn disable_pwm(&self) {
        self.tim1.bdtr.modify(|_, w| w.moe().disabled_idle());

        // Disable counter.
        self.tim1.cr1.modify(|_, w| w.cen().disabled());

        // Disable TIM1 clock.
        self.rcc.regs.apb2enr.modify(|_, w| w.tim1en().disabled());
    }

    fn pulse(&self, note_frequency: u32) {
        let frequency = if note_frequency == 0 || note_frequency > config::CLOCK_SPEED {
            0
        } else {
            (config::CLOCK_SPEED / note_frequency) as u16 - 1
        };

        self.tim1.arr.write(|w| w.arr().bits(frequency));
    }
}
