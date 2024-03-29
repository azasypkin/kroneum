#![doc = "API for the Kroneum Firmware"]
#![deny(warnings)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate bare_metal;
extern crate bit_field;
extern crate libm;

pub mod adc;
pub mod array;
pub mod beeper;
pub mod buttons;
pub mod config;
pub mod flash;
pub mod radio;
pub mod rtc;
pub mod system;
pub mod systick;
pub mod time;
pub mod timer;
pub mod usb;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    #[derive(Default)]
    pub(crate) struct Order {
        counter: RefCell<u32>,
    }

    impl Order {
        fn next(&self) -> u32 {
            let current = *self.counter.borrow();
            self.counter.replace(current + 1);
            current
        }
    }

    pub(crate) struct MockCalls<'a, T: Copy> {
        order: Option<&'a Order>,
        logs: [Option<T>; 15],
        ordered_logs: [Option<(T, u32)>; 15],
        pointer: usize,
    }

    impl<'a, T: Copy> MockCalls<'a, T> {
        pub fn with_order(order: &'a Order) -> Self {
            MockCalls {
                order: Some(order),
                ..Default::default()
            }
        }

        pub fn log_call(&mut self, call: T) {
            self.logs[self.pointer] = Some(call);

            if let Some(ref mut order) = self.order {
                self.ordered_logs[self.pointer] = Some((call, order.next()));
            }

            self.pointer += 1;
        }

        pub fn logs(&self) -> &[Option<T>] {
            &self.logs[..self.pointer]
        }

        pub fn ordered_logs(&self) -> &[Option<(T, u32)>] {
            &self.ordered_logs[..self.pointer]
        }
    }

    impl<'a, T: Copy> Default for MockCalls<'a, T> {
        fn default() -> Self {
            MockCalls {
                order: None,
                logs: [None; 15],
                ordered_logs: [None; 15],
                pointer: 0,
            }
        }
    }

    pub(crate) struct MockData<'a, T: Copy, D = ()> {
        pub calls: MockCalls<'a, T>,
        pub data: D,
    }

    impl<'a, T: Copy, D> MockData<'a, T, D> {
        pub fn new(inner_data: D) -> Self {
            MockData {
                data: inner_data,
                calls: MockCalls::default(),
            }
        }

        pub fn without_data() -> MockData<'a, T, ()> {
            MockData {
                data: (),
                calls: MockCalls::default(),
            }
        }

        pub fn with_call_order(order: &'a Order) -> MockData<'a, T, ()> {
            MockData {
                data: (),
                calls: MockCalls::with_order(order),
            }
        }

        pub fn with_data_and_call_order(data: D, order: &'a Order) -> Self {
            MockData {
                data,
                calls: MockCalls::with_order(order),
            }
        }
    }
}
