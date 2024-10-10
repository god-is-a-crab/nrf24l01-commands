#![no_std]

pub mod commands;
pub mod registers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rf_ch() {
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
    fn test_tx_addr() {
        // Check default
        let reg = registers::TxAddr::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = reg.with_tx_addr(0xA2891FFF6A);
        assert_eq!(reg.into_bits(), 0xA2891FFF6A);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::TxAddr>::bytes();
        assert_eq!(read_reg, [0 | 0x10, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(
            write_reg,
            [0b0010_0000 | 0x10, 0x6A, 0xFF, 0x1F, 0x89, 0xA2]
        );
    }

    #[test]
    fn test_rx_addr_p0() {
        // Check default
        let reg = registers::RxAddrP0::new();
        assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
        // Check fields
        let reg = reg.with_rx_addr_p0(0x17E73A6C58);
        assert_eq!(reg.into_bits(), 0x17E73A6C58);
        // Check read command
        let read_reg = commands::ReadRegister::<registers::RxAddrP0>::bytes();
        assert_eq!(read_reg, [0 | 0x0A, 0]);
        // Check write command
        let write_reg = commands::WriteRegister(reg).bytes();
        assert_eq!(
            write_reg,
            [0b0010_0000 | 0x0A, 0x58, 0x6C, 0x3A, 0xE7, 0x17]
        );
    }
}
