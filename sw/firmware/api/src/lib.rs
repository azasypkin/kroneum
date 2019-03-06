#![doc = "API for the Kroneum Firmware"]
#![deny(warnings)]
#![no_std]

pub mod config;

pub fn delay_us(us: u32) -> u32 {
    us * 2
}

#[cfg(test)]
mod tests {
    use delay_us;

    #[test]
    fn test() {
        assert_eq!(delay_us(5), 10);
    }
}