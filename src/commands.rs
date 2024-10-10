pub enum Command {
    ReadRegister(u8),
    WriteRegister(u8),
    ReadRxPayload,
    WriteTxPayload,
    FlushTx,
    FlushRx,
    ReuseTxPayload,
    Activate,
    ReadRxPayloadWidth,
    WriteAckPayload(u8),
    WriteTxPayloadNoAck,
    Nop,
}

impl Command {
    pub fn word(self) -> u8 {
        match self {
            Self::ReadRegister(address) => 0 | address,
            Self::WriteRegister(address) => 0b0010_0000 | address,
            Self::ReadRxPayload => 0b0110_0001,
            Self::WriteTxPayload => 0b1010_0000,
            Self::FlushTx => 0b1110_0001,
            Self::FlushRx => 0b1110_0010,
            Self::ReuseTxPayload => 0b1110_0011,
            Self::Activate => 0b0101_0000,
            Self::ReadRxPayloadWidth => 0b0110_0000,
            Self::WriteAckPayload(pipe) => 0b1010_1000 | pipe,
            Self::WriteTxPayloadNoAck => 0b1011_0000,
            Self::Nop => 0b1111_1111,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_register() {
        let command_word = Command::ReadRegister(3).word();
        assert_eq!(command_word, 0b0000_0011);
    }

    #[test]
    fn test_write_register() {
        let command_word = Command::WriteRegister(4).word();
        assert_eq!(command_word, 0b0010_0100);
    }

    #[test]
    fn test_flush_tx() {
        assert_eq!(Command::FlushTx.word(), 0b1110_0001)
    }

    #[test]
    fn test_write_ack_payload() {
        let command_word = Command::WriteAckPayload(4).word();
        assert_eq!(command_word, 0b1010_1100);
    }
}
