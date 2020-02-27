pub mod config;
pub mod en_aa;
pub mod en_rx_addr;
pub mod fifo_status;
pub mod rf_ch;
pub mod rf_setup;
pub mod rx_addr;
pub mod rx_pw;
pub mod setup_aw;
pub mod setup_retr;
pub mod status;
pub mod tx_addr;

pub trait Register {
    type TRaw: AsRef<[u8]> + AsMut<[u8]> + Default;
    fn address() -> u8;
    fn raw(&self) -> Self::TRaw;
    fn from_raw(buffer: Self::TRaw) -> Self;
}
