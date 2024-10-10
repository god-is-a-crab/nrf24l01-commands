//! Register definitions for the nRF24L01.
use bitfield_struct::bitfield;

pub trait Register {
    const ADDRESS: u8;
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

impl Register for Config {
    const ADDRESS: u8 = 0x00;
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

impl Register for EnAa {
    const ADDRESS: u8 = 0x01;
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

impl Register for EnRxaddr {
    const ADDRESS: u8 = 0x02;
}

#[bitfield(u8)]
pub struct SetupAw {
    #[bits(2, default = 3)]
    pub aw: u8,
    #[bits(6)]
    __: u8,
}

impl Register for SetupAw {
    const ADDRESS: u8 = 0x03;
}

#[bitfield(u8)]
pub struct SetupRetr {
    #[bits(4, default = 3)]
    pub arc: u8,
    #[bits(4)]
    pub ard: u8,
}

impl Register for SetupRetr {
    const ADDRESS: u8 = 0x04;
}

#[bitfield(u8)]
pub struct RfCh {
    #[bits(7, default = 2)]
    pub rf_ch: u8,
    #[bits(1)]
    __: bool,
}

impl Register for RfCh {
    const ADDRESS: u8 = 0x05;
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

impl Register for RfSetup {
    const ADDRESS: u8 = 0x06;
}

#[bitfield(u8)]
pub struct Status {
    #[bits(1, access = RO)]
    pub tx_full: bool,
    #[bits(3, access = RO)]
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

impl Register for Status {
    const ADDRESS: u8 = 0x07;
}

#[bitfield(u8)]
pub struct ObserveTx {
    #[bits(4, access = RO)]
    pub arc_cnt: u8,
    #[bits(4, access = RO)]
    pub plos_cnt: u8,
}

impl Register for ObserveTx {
    const ADDRESS: u8 = 0x08;
}

#[bitfield(u8)]
pub struct Cd {
    #[bits(1, access = RO)]
    pub cd: bool,
    #[bits(7)]
    __: u8,
}

impl Register for Cd {
    const ADDRESS: u8 = 0x09;
}

#[bitfield(u64)]
pub struct RxAddrP0 {
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub rx_addr_p0: u64,
    #[bits(24)]
    __: u32,
}

impl Register for RxAddrP0 {
    const ADDRESS: u8 = 0x0A;
}

impl RxAddrP0 {
    pub const fn into_bytes(self) -> [u8; 5] {
        let le_bytes: [u8; 8] = self.0.to_le_bytes();
        [
            le_bytes[0],
            le_bytes[1],
            le_bytes[2],
            le_bytes[3],
            le_bytes[4],
        ]
    }
}

#[bitfield(u64)]
pub struct RxAddrP1 {
    #[bits(40, default = 0xC2C2C2C2C2)]
    pub rx_addr_p1: u64,
    #[bits(24)]
    __: u32,
}

impl Register for RxAddrP1 {
    const ADDRESS: u8 = 0x0B;
}

impl RxAddrP1 {
    pub const fn into_bytes(self) -> [u8; 5] {
        let le_bytes: [u8; 8] = self.0.to_le_bytes();
        [
            le_bytes[0],
            le_bytes[1],
            le_bytes[2],
            le_bytes[3],
            le_bytes[4],
        ]
    }
}

#[bitfield(u8)]
pub struct RxAddrP2 {
    #[bits(8, default = 0xC3)]
    pub rx_addr_p2: u8,
}

impl Register for RxAddrP2 {
    const ADDRESS: u8 = 0x0C;
}

#[bitfield(u8)]
pub struct RxAddrP3 {
    #[bits(8, default = 0xC4)]
    pub rx_addr_p3: u8,
}

impl Register for RxAddrP3 {
    const ADDRESS: u8 = 0x0D;
}

#[bitfield(u8)]
pub struct RxAddrP4 {
    #[bits(8, default = 0xC5)]
    pub rx_addr_p4: u8,
}

impl Register for RxAddrP4 {
    const ADDRESS: u8 = 0x0E;
}

#[bitfield(u8)]
pub struct RxAddrP5 {
    #[bits(8, default = 0xC6)]
    pub rx_addr_p5: u8,
}

impl Register for RxAddrP5 {
    const ADDRESS: u8 = 0x0F;
}

#[bitfield(u64)]
pub struct TxAddr {
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub tx_addr: u64,
    #[bits(24)]
    __: u32,
}

impl Register for TxAddr {
    const ADDRESS: u8 = 0x10;
}

impl TxAddr {
    pub const fn into_bytes(self) -> [u8; 5] {
        let le_bytes: [u8; 8] = self.0.to_le_bytes();
        [
            le_bytes[0],
            le_bytes[1],
            le_bytes[2],
            le_bytes[3],
            le_bytes[4],
        ]
    }
}

#[bitfield(u8)]
pub struct RxPwP0 {
    #[bits(6)]
    pub rx_pw_p0: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP0 {
    const ADDRESS: u8 = 0x11;
}

#[bitfield(u8)]
pub struct RxPwP1 {
    #[bits(6)]
    pub rx_pw_p1: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP1 {
    const ADDRESS: u8 = 0x12;
}

#[bitfield(u8)]
pub struct RxPwP2 {
    #[bits(6)]
    pub rx_pw_p2: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP2 {
    const ADDRESS: u8 = 0x13;
}

#[bitfield(u8)]
pub struct RxPwP3 {
    #[bits(6)]
    pub rx_pw_p3: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP3 {
    const ADDRESS: u8 = 0x14;
}

#[bitfield(u8)]
pub struct RxPwP4 {
    #[bits(6)]
    pub rx_pw_p4: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP4 {
    const ADDRESS: u8 = 0x15;
}

#[bitfield(u8)]
pub struct RxPwP5 {
    #[bits(6)]
    pub rx_pw_p5: u8,
    #[bits(2)]
    __: u8,
}

impl Register for RxPwP5 {
    const ADDRESS: u8 = 0x16;
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

impl Register for FifoStatus {
    const ADDRESS: u8 = 0x17;
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

impl Register for Dynpd {
    const ADDRESS: u8 = 0x1C;
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

impl Register for Feature {
    const ADDRESS: u8 = 0x1D;
}
