#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod commands;
pub mod registers;

#[cfg(test)]
mod tests {
    use super::{commands, commands::Command, registers};

    #[test]
    fn test_reg_config() {
        // Check default
        let reg = registers::Config::new();
        assert_eq!(reg.into_bits(), 0b0000_1000);
        // Check fields
        let reg = reg
            .with_crco(true)
            .with_en_crc(false)
            .with_mask_max_rt(true)
            .with_mask_tx_ds(true)
            .with_mask_rx_dr(true);
        assert_eq!(reg.into_bits(), 0b0111_0100);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::Config>::bytes();
        assert_eq!(read_reg, [0 | 0x00, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x00, 0b0111_0100]);
    }

    #[test]
    fn test_reg_rf_ch() {
        // Check default
        let reg = registers::RfCh::new();
        assert_eq!(reg.into_bits(), 0b0000_0010);
        // Check fields
        let reg = reg.with_rf_ch(48);
        assert_eq!(reg.into_bits(), 48);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::RfCh>::bytes();
        assert_eq!(read_reg, [0 | 0x05, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x05, 48]);
    }

    #[test]
    fn test_reg_status() {
        // Check default
        let reg = registers::Status::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = registers::Status::from_bits(0b0010_0110);
        assert!(!reg.tx_full());
        assert_eq!(reg.rx_p_no(), 0b011);
        assert!(!reg.max_rt());
        assert!(reg.tx_ds());
        assert!(!reg.rx_dr());
        // Check read command
        let read_reg = commands::ReadRegister::<registers::Status>::bytes();
        assert_eq!(read_reg, [0 | 0x07, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x07, 0b0010_0110]);
    }

    #[test]
    fn test_reg_rx_addr_p0() {
        // Check default
        let reg = registers::RxAddrP0::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = reg.with_rx_addr_p0(0x17E73A6C58);
        assert_eq!(reg.into_bits(), 0x17E73A6C58);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::RxAddrP0>::bytes();
        assert_eq!(read_reg, [0 | 0x0A, 0, 0, 0, 0, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(
            write_reg,
            [0b0010_0000 | 0x0A, 0x58, 0x6C, 0x3A, 0xE7, 0x17]
        );
    }

    #[test]
    fn test_reg_tx_addr() {
        // Check default
        let reg = registers::TxAddr::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = reg.with_tx_addr(0xA2891FFF6A);
        assert_eq!(reg.into_bits(), 0xA2891FFF6A);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::TxAddr>::bytes();
        assert_eq!(read_reg, [0 | 0x10, 0, 0, 0, 0, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(
            write_reg,
            [0b0010_0000 | 0x10, 0x6A, 0xFF, 0x1F, 0x89, 0xA2]
        );
    }

    #[test]
    fn test_reg_feature() {
        // Check default
        let reg = registers::Feature::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = reg
            .with_en_dyn_ack(true)
            .with_en_ack_pay(true)
            .with_en_dpl(false);
        assert_eq!(reg.into_bits(), 0b0000_0011);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::Feature>::bytes();
        assert_eq!(read_reg, [0 | 0x1D, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x1D, 0b0000_0011]);
    }

    #[test]
    fn test_cmd_activate() {
        let command = commands::Activate();
        assert_eq!(command.bytes(), [0b0101_0000, 0x73]);
    }

    #[test]
    fn test_cmd_write_tx_payload_no_ack() {
        let bytes = commands::WriteTxPayloadNoAck::bytes([b'H', b'e', b'l', b'l', b'o']);
        assert_eq!(bytes, [0b1011_0000, b'o', b'l', b'l', b'e', b'H']);
    }

    #[test]
    fn test_cmd_nop() {
        assert_eq!(commands::Nop::WORD, 0xFF);
    }
}
