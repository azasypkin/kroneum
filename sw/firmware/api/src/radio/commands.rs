#[allow(dead_code)]
pub enum Command {
    /// Read command and status registers. 5 LSB = 5 bit Register Map Address.
    ReadRegister(u8),

    /// Write command and status registers. 5 LSB = 5 bit Register Map Address. Executable in power
    /// down or standby modes only.
    WriteRegister(u8),

    /// Read RX-payload: 1 – 32 bytes. Command always starts at byte 0. Payload is deleted from FIFO
    /// after it is read. Used in RX mode.
    ReadRxPayload,

    /// Read RX payload width for the top R_RX_PAYLOAD in the RX FIFO.Note: Flush RX FIFO if the
    /// read value is larger than 32 bytes.
    ReadRxPayloadWidth,

    /// Write TX-payload: 1 – 32 bytes. Command always starts at byte 0 used in TX payload.
    WriteTxPayload,

    /// Flush TX FIFO, used in TX mode.
    FlushTx,

    /// Flush RX FIFO, used in RX mode. Should not be executed during transmission of acknowledge,
    /// that is, acknowledge package will not be completed.
    FlushRx,
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::ReadRegister(address) => address | 0b000_00000,
            Command::WriteRegister(address) => address | 0b001_00000,
            Command::ReadRxPayload => 0b0110_0001,
            Command::WriteTxPayload => 0b1010_0000,
            Command::ReadRxPayloadWidth => 0b0110_0000,
            Command::FlushTx => 0b1110_0001,
            Command::FlushRx => 0b1110_0010,
        }
    }
}
