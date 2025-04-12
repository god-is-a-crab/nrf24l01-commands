//! <div class="warning">
//! <strong>Requires Rust Nightly</strong>
//! </div>
//!
#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![doc = include_str!("../README.md")]

pub mod commands;
pub mod fields;
pub mod registers;

#[cfg(test)]
mod tests {
    use super::{commands, fields, registers, registers::AddressRegister};

    #[test]
    fn test_reg_config() {
        // Check default
        let reg = registers::Config::new();
        assert_eq!(reg.into_bits(), 0b0000_1000);
        // Check fields
        let reg = reg
            .with_crco(fields::Crco::TwoByte)
            .with_en_crc(false)
            .with_mask_max_rt(true)
            .with_mask_tx_ds(true)
            .with_mask_rx_dr(true);
        assert_eq!(reg.into_bits(), 0b0111_0100);
        // Check read command
        let read_reg = commands::RRegister::<registers::Config>::bytes();
        assert_eq!(read_reg, [0 | 0x00, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
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
        let read_reg = commands::RRegister::<registers::RfCh>::bytes();
        assert_eq!(read_reg, [0 | 0x05, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x05, 48]);
    }

    #[test]
    fn test_reg_status() {
        // Check default
        let reg = registers::Status::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let mut reg = registers::Status::from_bits(0b0010_0110);
        assert!(!reg.tx_full());
        assert_eq!(reg.rx_p_no(), fields::RxPipeNo::Pipe3);
        assert!(!reg.max_rt());
        assert!(reg.tx_ds());
        assert!(!reg.rx_dr());
        // Set field
        reg.set_max_rt(true);
        assert_eq!(reg.into_bits(), 0b0011_0110);
        // Check read command
        let read_reg = commands::RRegister::<registers::Status>::bytes();
        assert_eq!(read_reg, [0 | 0x07, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x07, 0b0011_0110]);
    }

    #[test]
    fn test_reg_cd() {
        // Check default
        let reg = registers::Rpd::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = registers::Rpd::from_bits(1);
        assert_eq!(reg.into_bits(), 1);
        // Check read command
        let read_reg = commands::RRegister::<registers::Rpd>::bytes();
        assert_eq!(read_reg, [0 | 0x09, 0]);
    }

    #[test]
    fn test_reg_rx_addr_p0() {
        // Check default
        let reg = registers::RxAddrP0::<3>::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = registers::RxAddrP0::<4>::new().with_rx_addr_p0(0x17E73A6C58);
        assert_eq!(reg.into_bits(), 0x17E73A6C58);
        // Check read command
        let read_reg = commands::RRegister::<registers::RxAddrP0<5>>::bytes();
        assert_eq!(read_reg, [0 | 0x0A, 0, 0, 0, 0, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x0A, 0x58, 0x6C, 0x3A, 0xE7]);
    }

    #[test]
    fn test_reg_tx_addr() {
        // Check default
        let reg = registers::TxAddr::<5>::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = registers::TxAddr::<5>::new().with_tx_addr(0xA2891FFF6A);
        assert_eq!(reg.into_bits(), 0xA2891FFF6A);
        // Check read command
        let read_reg = commands::RRegister::<registers::TxAddr<4>>::bytes();
        assert_eq!(read_reg, [0 | 0x10, 0, 0, 0, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(
            write_reg,
            [0b0010_0000 | 0x10, 0x6A, 0xFF, 0x1F, 0x89, 0xA2]
        );
    }

    #[test]
    fn test_reg_rx_pw_p0() {
        // Check default
        let reg = registers::RxPwP0::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = reg.with_rx_pw_p0(32);
        assert_eq!(reg.into_bits(), 32);
        // Check read command
        let read_reg = commands::RRegister::<registers::RxPwP0>::bytes();
        assert_eq!(read_reg, [0 | 0x11, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x11, 32]);
    }

    #[test]
    fn test_reg_fifo_status() {
        // Check default
        let reg = registers::FifoStatus::new();
        assert_eq!(reg.into_bits(), 0);
        // Check fields
        let reg = registers::FifoStatus::from_bits(0b0100_0001);
        assert!(reg.rx_empty());
        assert!(!reg.rx_full());
        assert!(!reg.tx_empty());
        assert!(!reg.tx_full());
        assert!(reg.tx_reuse());
        // Check read command
        let read_reg = commands::RRegister::<registers::FifoStatus>::bytes();
        assert_eq!(read_reg, [0 | 0x17, 0]);
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
        let read_reg = commands::RRegister::<registers::Feature>::bytes();
        assert_eq!(read_reg, [0 | 0x1D, 0]);
        // Check write command
        let write_reg = commands::WRegister(reg).bytes();
        assert_eq!(write_reg, [0b0010_0000 | 0x1D, 0b0000_0011]);
    }

    #[test]
    fn test_read_rx_payload() {
        let bytes = commands::RRxPayload::<32>::bytes();
        let mut expected_bytes = [0; 33];
        expected_bytes[0] = 0b0110_0001;
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_cmd_write_tx_payload_no_ack() {
        let bytes = commands::WTxPayloadNoack([b'H', b'e', b'l', b'l', b'o']).bytes();
        assert_eq!(bytes, [0b1011_0000, b'H', b'e', b'l', b'l', b'o']);
    }
}
