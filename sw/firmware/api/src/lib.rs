#![doc = "API for the Kroneum Firmware"]
#![deny(warnings)]
#![no_std]

pub mod beeper;
pub mod buttons;
pub mod config;
pub mod flash;
pub mod rtc;
pub mod systick;
pub mod time;
pub mod usb;

#[cfg(test)]
mod tests {
    pub(crate) struct MockCalls<T: Copy> {
        logs: [Option<T>; 15],
        pointer: usize,
    }

    impl<T: Copy> MockCalls<T> {
        pub fn log_call(&mut self, call: T) {
            self.logs[self.pointer] = Some(call);
            self.pointer += 1;
        }

        pub fn logs(&self) -> &[Option<T>] {
            &self.logs[..self.pointer]
        }
    }

    impl<T: Copy> Default for MockCalls<T> {
        fn default() -> Self {
            MockCalls {
                logs: [None; 15],
                pointer: 0,
            }
        }
    }
}
