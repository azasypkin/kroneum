use crate::config;
use crate::systick::SysTick;
use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::Peripherals;

const EIGHTH_NOTE: u32 = 150;
const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;

static SCALES: &'static [u32] = &[523, 554, 587, 622, 659, 698, 740, 784, 831, 880, 932, 988];

pub struct Beeper<'a> {
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
}

impl<'a> Beeper<'a> {
    fn new(core_peripherals: &'a mut CorePeripherals, peripherals: &'a Peripherals) -> Beeper<'a> {
        Beeper {
            core_peripherals,
            peripherals,
        }
    }

    pub fn configure(peripherals: &Peripherals) {
        Self::configure_pwm(peripherals);
        Self::configure_timer(peripherals);
    }

    pub fn acquire<'b, F>(
        core_peripherals: &'b mut CorePeripherals,
        peripherals: &'b Peripherals,
        f: F,
    ) -> ()
    where
        F: FnOnce(Beeper),
    {
        f(Beeper::new(core_peripherals, peripherals));
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
            self.play_note(SCALES[7], QUARTER_NOTE);
            self.toggle_pwm(false);

            if i < n {
                SysTick::delay_ms(&mut self.core_peripherals.SYST, 100);
            }
        }
    }

    fn play_note(&mut self, note: u32, delay: u32) {
        self.peripherals
            .TIM1
            .arr
            .write(|w| unsafe { w.bits((config::CLOCK_SPEED / note) - 1) });

        SysTick::delay_ms(&mut self.core_peripherals.SYST, delay);
    }

    fn configure_pwm(peripherals: &Peripherals) {
        // Enable clock for GPIO Port A.
        peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

        // Switch PA7 to alternate function mode.
        peripherals
            .GPIOA
            .moder
            .modify(|_, w| unsafe { w.moder7().bits(0b10) });

        // No pull-up, pull-down.
        peripherals
            .GPIOA
            .pupdr
            .modify(|_, w| unsafe { w.pupdr7().bits(0b0) });

        // Set "high" output speed.
        peripherals
            .GPIOA
            .ospeedr
            .modify(|_, w| unsafe { w.ospeedr7().bits(0b11) });

        // Set alternative function #2.
        peripherals
            .GPIOA
            .afrl
            .modify(|_, w| unsafe { w.afrl7().bits(0b0010) });
    }

    fn configure_timer(peripherals: &Peripherals) {
        // Compute the value to be set in ARR register to generate signal frequency at 17.57 Khz.
        let timer_period: u32 = (config::CLOCK_SPEED / 17_570) - 1;

        // Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N.
        let channel_one_pulse: u32 = (5 * (timer_period - 1)) / 10;

        // Enable TIM1 clock.
        peripherals.RCC.apb2enr.modify(|_, w| w.tim1en().set_bit());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to
        // f(CK_PSC) / (PSC[15:0] + 1).
        peripherals.TIM1.psc.modify(|_, w| unsafe { w.bits(0b0) });

        peripherals.TIM1.cr1.modify(|_, w| {
            // Set direction: counter used as up-counter.
            w.dir().clear_bit();
            // Set clock division to t(DTS) = t(CK_INT).
            unsafe {
                w.ckd().bits(0b00);
            }
            w
        });

        // Set value to auto-reload register.
        peripherals
            .TIM1
            .arr
            .write(|w| unsafe { w.bits(timer_period) });

        // Set repetition counter.
        peripherals.TIM1.rcr.write(|w| unsafe { w.bits(0b0) });

        // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
        // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
        //inactive.
        unsafe {
            peripherals
                .TIM1
                .ccmr1_output
                .modify(|_, w| w.oc1m().bits(0b111));
        }

        // Configure capture/compare enable register.
        peripherals.TIM1.ccer.modify(|_, w| {
            // Enable Capture/Compare 1 output.
            w.cc1e().set_bit();
            // Enable Capture/Compare 1 complementary output.
            w.cc1ne().set_bit();
            // Set low polarity for Capture/Compare 1 output.
            w.cc1p().set_bit();
            // Set high polarity for Capture/Compare complementary 1 output.
            w.cc1np().clear_bit();
            w
        });

        // CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).
        peripherals
            .TIM1
            .ccr1
            .write(|w| unsafe { w.bits(channel_one_pulse) });

        // Configure control register 2.
        peripherals.TIM1.cr2.modify(|_, w| {
            // Set output Idle state 1 (OC1 output).
            w.ois1().set_bit();
            // Set output Idle state 1 (OC1N output).
            w.ois1n().clear_bit();
            w
        });

        // Enable counter.
        peripherals.TIM1.cr1.modify(|_, w| w.cen().set_bit());
    }

    fn toggle_pwm(&self, enable: bool) {
        self.peripherals.TIM1.bdtr.modify(|_, w| {
            if enable {
                w.moe().set_bit()
            } else {
                w.moe().clear_bit()
            }
        });
    }
}
