pub const CLOCK_SPEED: u32 = 8_000_000;

#[cfg(test)]
mod tests {
    use super::CLOCK_SPEED;

    #[test]
    fn correct_clock_speed() {
        assert_eq!(CLOCK_SPEED, 8_000_000);
    }
}
