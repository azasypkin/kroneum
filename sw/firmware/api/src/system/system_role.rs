use beeper::melody::Melody;
use time::Time;

#[derive(Debug, Copy, Clone)]
pub enum SystemRole {
    Controller,
    Timer(TimerMode),
}

#[derive(Debug, Copy, Clone)]
pub enum TimerMode {
    Idle,
    Setup(u32),
    Alarm(Time, Melody),
}
