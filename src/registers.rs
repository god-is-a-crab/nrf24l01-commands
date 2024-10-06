use bitfield_struct::bitfield;

#[repr(u8)]
pub enum Address {
    Config = 0x00,
    EnAa = 0x01,
    EnRxaddr = 0x02,
    SetupAw = 0x03,
    SetupRetr = 0x04,
    RfCh = 0x05,
    RfSetup = 0x06,
    Status = 0x07,
    ObserveTx = 0x08,
    Cd = 0x09,
    RxAddrP0 = 0x0A,
    RxAddrP1 = 0x0B,
    RxAddrP2 = 0x0C,
    RxAddrP3 = 0x0D,
    RxAddrP4 = 0x0E,
    RxAddrP5 = 0x0F,
    TxAddr = 0x10,
    RxPwP0 = 0x11,
    RxPwP1 = 0x12,
    RxPwP2 = 0x13,
    RxPwP3 = 0x14,
    RxPwP4 = 0x15,
    RxPwP5 = 0x16,
    FifoStatus = 0x17,
    Dynpd = 0x1C,
    Feature = 0x1D,
}

#[bitfield(u8)]
pub struct Config {
    #[bits(1)]
    pub prim_rx: bool,
    #[bits(1)]
    pub pwr_up: bool,
    #[bits(1)]
    pub crco: bool,
    #[bits(1, default = true)]
    pub en_crc: bool,
    #[bits(1)]
    pub mask_max_rt: bool,
    #[bits(1)]
    pub mask_tx_ds: bool,
    #[bits(1)]
    pub mask_rx_dr: bool,
    #[bits(1)]
    __: bool,
}

#[bitfield(u8)]
pub struct EnAa {
    #[bits(1, default = true)]
    pub en_aa_p0: bool,
    #[bits(1, default = true)]
    pub en_aa_p1: bool,
    #[bits(1, default = true)]
    pub en_aa_p2: bool,
    #[bits(1, default = true)]
    pub en_aa_p3: bool,
    #[bits(1, default = true)]
    pub en_aa_p4: bool,
    #[bits(1, default = true)]
    pub en_aa_p5: bool,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct EnRxaddr {
    #[bits(1, default = true)]
    pub erx_p0: bool,
    #[bits(1, default = true)]
    pub erx_p1: bool,
    #[bits(1)]
    pub erx_p2: bool,
    #[bits(1)]
    pub erx_p3: bool,
    #[bits(1)]
    pub erx_p4: bool,
    #[bits(1)]
    pub erx_p5: bool,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct SetupAw {
    #[bits(2, default = 3)]
    pub aw: u8,
    #[bits(6)]
    __: u8,
}

#[bitfield(u8)]
pub struct SetupRetr {
    #[bits(4, default = 3)]
    pub arc: u8,
    #[bits(4)]
    pub ard: u8,
}

#[bitfield(u8)]
pub struct RfCh {
    #[bits(7, default = 2)]
    pub rf_ch: u8,
    #[bits(1)]
    __: bool,
}

#[bitfield(u8)]
pub struct RfSetup {
    #[bits(1, default = true)]
    pub lna_hcurr: bool,
    #[bits(2, default = 3)]
    pub rf_pwr: u8,
    #[bits(1, default = true)]
    pub rf_dr: bool,
    #[bits(1)]
    pub pll_lock: bool,
    #[bits(3)]
    __: u8,
}

#[bitfield(u8)]
pub struct Status {
    #[bits(1, access = RO)]
    pub tx_full: bool,
    #[bits(3, default = 7, access = RO)]
    pub rx_p_no: u8,
    #[bits(1)]
    pub max_rt: bool,
    #[bits(1)]
    pub tx_ds: bool,
    #[bits(1)]
    pub rx_dr: bool,
    #[bits(1)]
    __: bool,
}

#[bitfield(u8)]
pub struct ObserveTx {
    #[bits(4, access = RO)]
    pub arc_cnt: u8,
    #[bits(4, access = RO)]
    pub plos_cnt: u8,
}

#[bitfield(u64)]
pub struct RxAddrP0 {
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub rx_addr_p0: u64,
    #[bits(24)]
    __: u32,
}

impl RxAddrP0 {
    pub fn as_payload(self) -> [u8; 5] {
        unsafe { *(self.0.to_le_bytes()[..5].as_ptr() as *const [u8; 5]) }
    }
}

#[bitfield(u64)]
pub struct RxAddrP1 {
    #[bits(40, default = 0xC2C2C2C2C2)]
    pub rx_addr_p1: u64,
    #[bits(24)]
    __: u32,
}

impl RxAddrP1 {
    pub fn as_payload(self) -> [u8; 5] {
        unsafe { *(self.0.to_le_bytes()[..5].as_ptr() as *const [u8; 5]) }
    }
}

#[bitfield(u64)]
pub struct TxAddr {
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub tx_addr: u64,
    #[bits(24)]
    __: u32,
}

#[bitfield(u8)]
pub struct RxPwP0 {
    #[bits(6)]
    pub rx_pw_p0: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct RxPwP1 {
    #[bits(6)]
    pub rx_pw_p1: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct RxPwP2 {
    #[bits(6)]
    pub rx_pw_p2: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct RxPwP3 {
    #[bits(6)]
    pub rx_pw_p3: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct RxPwP4 {
    #[bits(6)]
    pub rx_pw_p4: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct RxPwP5 {
    #[bits(6)]
    pub rx_pw_p5: u8,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct FifoStatus {
    #[bits(1, access = RO)]
    pub rx_empty: bool,
    #[bits(1, access = RO)]
    pub rx_full: bool,
    #[bits(2)]
    __: u8,
    #[bits(1, access = RO)]
    pub tx_empty: bool,
    #[bits(1, access = RO)]
    pub tx_full: bool,
    #[bits(1, access = RO)]
    pub tx_reuse: bool,
    #[bits(1)]
    __: bool,
}

#[bitfield(u8)]
pub struct Dynpd {
    #[bits(1)]
    pub dpl_p0: bool,
    #[bits(1)]
    pub dpl_p1: bool,
    #[bits(1)]
    pub dpl_p2: bool,
    #[bits(1)]
    pub dpl_p3: bool,
    #[bits(1)]
    pub dpl_p4: bool,
    #[bits(1)]
    pub dpl_p5: bool,
    #[bits(2)]
    __: u8,
}

#[bitfield(u8)]
pub struct Feature {
    #[bits(1)]
    pub en_dyn_ack: bool,
    #[bits(1)]
    pub en_ack_pay: bool,
    #[bits(1)]
    pub en_dpl: bool,
    #[bits(5)]
    __: u8,
}

impl TxAddr {
    pub fn as_payload(self) -> [u8; 5] {
        unsafe { *(self.0.to_le_bytes()[..5].as_ptr() as *const [u8; 5]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::default().with_pwr_up(true);
        assert_eq!(config.0, 0b0000_1010);
        assert!(!config.prim_rx());
        assert!(config.pwr_up());
        assert!(!config.crco());
        assert!(config.en_crc());
    }

    #[test]
    fn test_tx_addr() {
        let tx_addr = TxAddr::default().with_tx_addr(0xA2891FFF6A);
        assert_eq!(tx_addr.as_payload(), [0x6A, 0xFF, 0x1F, 0x89, 0xA2])
    }
}
