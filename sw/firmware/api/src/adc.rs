use core::convert::TryFrom;

/// Describes available ADC channels.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADCChannel {
    // PA1 - ADC_IN1
    Channel1,
    // PA3 - ADC_IN3
    Channel3,
    // PA4 - ADC_IN4
    Channel4,
    // PA5 - ADC_IN5
    Channel5,
    // PA6 - ADC_IN6
    Channel6,
    // PA7 - ADC_IN7
    Channel7,
}

impl From<ADCChannel> for u8 {
    fn from(channel: ADCChannel) -> Self {
        match channel {
            ADCChannel::Channel1 => 1,
            ADCChannel::Channel3 => 3,
            ADCChannel::Channel4 => 4,
            ADCChannel::Channel5 => 5,
            ADCChannel::Channel6 => 6,
            ADCChannel::Channel7 => 7,
        }
    }
}

impl TryFrom<u8> for ADCChannel {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ADCChannel::Channel1),
            3 => Ok(ADCChannel::Channel3),
            4 => Ok(ADCChannel::Channel4),
            5 => Ok(ADCChannel::Channel5),
            6 => Ok(ADCChannel::Channel6),
            7 => Ok(ADCChannel::Channel7),
            _ => Err("Not supported channel!"),
        }
    }
}

/// Describes the ADC hardware management interface.
pub trait ADCHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Calibrates ADC.
    fn calibrate(&self);

    /// Reads ADC value using the specified channel.
    fn read(&self, channel: ADCChannel) -> u16;

    /// Releases hardware if needed.
    fn teardown(&self);
}

pub struct ADC<'a, T: ADCHardware> {
    hw: &'a T,
}

impl<'a, T: ADCHardware> ADC<'a, T> {
    pub fn new(hw: &'a T) -> Self {
        ADC { hw }
    }

    pub fn read(&mut self, channel: ADCChannel) -> u16 {
        self.hw.setup();

        self.hw.calibrate();
        let value = self.hw.read(channel);

        self.hw.teardown();
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{MockData, Order};
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Call {
        Setup,
        Teardown,
        Calibrate,
        Read(ADCChannel),
    }

    #[test]
    fn channel_number_serialize() {
        assert_eq!(u8::from(ADCChannel::Channel1), 1);
        assert_eq!(u8::from(ADCChannel::Channel3), 3);
        assert_eq!(u8::from(ADCChannel::Channel4), 4);
        assert_eq!(u8::from(ADCChannel::Channel5), 5);
        assert_eq!(u8::from(ADCChannel::Channel6), 6);
        assert_eq!(u8::from(ADCChannel::Channel7), 7);
    }

    #[test]
    fn channel_number_deserialize() {
        assert_eq!(ADCChannel::try_from(1), Ok(ADCChannel::Channel1));
        assert_eq!(ADCChannel::try_from(3), Ok(ADCChannel::Channel3));
        assert_eq!(ADCChannel::try_from(4), Ok(ADCChannel::Channel4));
        assert_eq!(ADCChannel::try_from(5), Ok(ADCChannel::Channel5));
        assert_eq!(ADCChannel::try_from(6), Ok(ADCChannel::Channel6));
        assert_eq!(ADCChannel::try_from(7), Ok(ADCChannel::Channel7));

        assert_eq!(ADCChannel::try_from(0), Err("Not supported channel!"));
        assert_eq!(ADCChannel::try_from(2), Err("Not supported channel!"));
        assert_eq!(ADCChannel::try_from(8), Err("Not supported channel!"));
    }

    struct ADCHardwareMock<'a> {
        pub data: RefCell<MockData<'a, Call>>,
    }

    impl<'a> ADCHardware for ADCHardwareMock<'a> {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn calibrate(&self) {
            self.data.borrow_mut().calls.log_call(Call::Calibrate);
        }

        fn read(&self, channel: ADCChannel) -> u16 {
            self.data.borrow_mut().calls.log_call(Call::Read(channel));
            555
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }
    }

    #[test]
    fn calibrates_and_reads_adc_value() {
        let order = Order::default();

        let adc_hw_mock = ADCHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut adc = ADC::new(&adc_hw_mock);

        assert_eq!(adc.read(ADCChannel::Channel3), 555);

        assert_eq!(
            [
                Some((Call::Setup, 0)),
                Some((Call::Calibrate, 1)),
                Some((Call::Read(ADCChannel::Channel3), 2)),
                Some((Call::Teardown, 3)),
            ],
            adc.hw.data.borrow().calls.ordered_logs()
        );
    }
}
