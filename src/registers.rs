//! Register definitions for the nRF24L01.
use bitfield_struct::bitfield;

/// Defines an nRF24L01 register with an address.
pub trait Register {
    const ADDRESS: u8;
}

/// Config register.
///
/// Address = 0x00
///
/// At reset: `en_crc` = 1
///
/// ```rust
/// use nrf24l01_commands::registers;
///
/// let reg = registers::Config::new();
/// assert_eq!(reg.into_bits(), 0b0000_1000);
///
/// // Set fields
/// let reg = reg
///     .with_prim_rx(false)
///     .with_pwr_up(true)
///     .with_mask_rx_dr(true);
/// assert_eq!(reg.into_bits(), 0b0100_1010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Config {
    #[bits(1)]
    __: bool,

    /// Mask/unmask interrupt caused by RX_DR.
    ///
    /// 0: unmasked
    ///
    /// 1: masked
    #[bits(1)]
    pub mask_rx_dr: bool,

    /// Mask/unmask interrupt caused by TX_DS.
    ///
    /// 0: unmasked
    ///
    /// 1: masked
    #[bits(1)]
    pub mask_tx_ds: bool,

    /// Mask/unmask interrupt caused by MAX_RT.
    ///
    /// 0: unmasked
    ///
    /// 1: masked
    #[bits(1)]
    pub mask_max_rt: bool,

    /// Enable/disable CRC.
    ///
    /// At reset: 1
    ///
    /// 0: disabled
    ///
    /// 1: enabled
    #[bits(1, default = true)]
    pub en_crc: bool,

    /// CRC encoding scheme.
    ///
    /// 0: 1 byte
    ///
    /// 1: 2 byte
    #[bits(1)]
    pub crco: bool,

    /// Power up/down. 0: power down, 1: power up.
    #[bits(1)]
    pub pwr_up: bool,

    /// Set primary RX/TX.
    ///
    /// 0: primary TX
    ///
    /// 1: primary RX
    #[bits(1)]
    pub prim_rx: bool,
}

impl Register for Config {
    const ADDRESS: u8 = 0x00;
}

/// Enable 'Auto Acknowledgement' on data pipes 0-5.
///
/// Address = 0x01
///
/// At reset, 'Auto Acknowledgement' is enabled for all pipes.
///
/// ```rust
/// use nrf24l01_commands::registers;
///
/// let reg = registers::EnAa::new();
/// assert_eq!(reg.into_bits(), 0b0011_1111);
///
/// // Set fields
/// let reg = reg
///     .with_en_aa_p3(false)
///     .with_en_aa_p5(false);
/// assert_eq!(reg.into_bits(), 0b0001_0111);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnAa {
    #[bits(2)]
    __: u8,
    /// Enable 'Auto Acknowledgement' on data pipe 5.
    #[bits(1, default = true)]
    pub en_aa_p5: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 4.
    #[bits(1, default = true)]
    pub en_aa_p4: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 3.
    #[bits(1, default = true)]
    pub en_aa_p3: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 2.
    #[bits(1, default = true)]
    pub en_aa_p2: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 1.
    #[bits(1, default = true)]
    pub en_aa_p1: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 0.
    #[bits(1, default = true)]
    pub en_aa_p0: bool,
}

impl Register for EnAa {
    const ADDRESS: u8 = 0x01;
}

/// Enable receive on data pipes 0-5.
///
/// Address = 0x02
///
/// At reset, pipe 0 and 1 are enabled.
///
/// ```rust
/// use nrf24l01_commands::registers;
///
/// let reg = registers::EnRxaddr::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Set fields
/// let reg = reg
///     .with_erx_p2(true)
///     .with_erx_p1(false);
/// assert_eq!(reg.into_bits(), 0b0000_0101);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnRxaddr {
    #[bits(2)]
    __: u8,
    /// Enable data pipe 5.
    #[bits(1)]
    pub erx_p5: bool,
    /// Enable data pipe 4.
    #[bits(1)]
    pub erx_p4: bool,
    /// Enable data pipe 3.
    #[bits(1)]
    pub erx_p3: bool,
    /// Enable data pipe 2.
    #[bits(1)]
    pub erx_p2: bool,
    /// Enable data pipe 1.
    #[bits(1, default = true)]
    pub erx_p1: bool,
    /// Enable data pipe 0.
    #[bits(1, default = true)]
    pub erx_p0: bool,
}

impl Register for EnRxaddr {
    const ADDRESS: u8 = 0x02;
}

/// Set up address width. This is common for the TX address and all RX data pipes.
///
/// Address = 0x03
///
/// At reset: aw = 0b11 (5 bit address)
#[bitfield(u8, order = Msb)]
pub struct SetupAw {
    #[bits(6)]
    __: u8,

    /// Address width. 00: Illegal, 01: 3 bytes, 10: 4 bytes, 11: 5 bytes.
    #[bits(2, default = 3)]
    pub aw: u8,
}

impl Register for SetupAw {
    const ADDRESS: u8 = 0x03;
}

/// Set up 'Automatic Retransmission'.
///
/// Address = 0x04
///
/// At reset: `arc` = 0b0011 (up to 3 retransmits)
#[bitfield(u8, order = Msb)]
pub struct SetupRetr {
    /// Auto retransmit delay.
    ///
    /// 0000: Wait 250µS
    ///
    /// 0001: Wait 500µS
    ///
    /// 0010: Wait 750µS
    ///
    /// ……
    ///
    /// 1111: Wait 4000µS
    #[bits(4)]
    pub ard: u8,

    /// Maximum auto retransmits.
    ///
    /// 0000: Auto retransmit disabled
    ///
    /// 0001: Up to 1 retransmit
    ///
    /// 0010: Up to 2 retransmits
    ///
    /// ……
    ///
    /// 1111: Up to 15 retransmits
    #[bits(4, default = 3)]
    pub arc: u8,
}

impl Register for SetupRetr {
    const ADDRESS: u8 = 0x04;
}

/// Set RF channel.
///
/// Address = 0x05
///
/// At reset: `rf_ch` = 2
#[bitfield(u8, order = Msb)]
pub struct RfCh {
    #[bits(1)]
    __: bool,

    /// Set frequency channel 0 - 125.
    #[bits(7, default = 2)]
    pub rf_ch: u8,
}

impl Register for RfCh {
    const ADDRESS: u8 = 0x05;
}

/// RF setup register.
///
/// Address = 0x06
///
/// At reset:
///
/// - `rf_dr` = 1 (2 Mbps)
///
/// - `rf_pwr` = 0b11 (0 dBm),
///
/// - `lna_hcurr` = 1 (higher gain)
#[bitfield(u8, order = Msb)]
pub struct RfSetup {
    #[bits(3)]
    __: u8,

    /// Force PLL lock signal. Only used in test.
    #[bits(1)]
    pub pll_lock: bool,

    /// Air data rate.
    ///
    /// 0: 1 Mbps
    ///
    /// 1: 2 Mbps
    #[bits(1, default = true)]
    pub rf_dr: bool,

    /// RF output power in TX mode.
    ///
    /// 00: -18 dBm
    ///
    /// 01: -12 dBm
    ///
    /// 10: -6 dBm
    ///
    /// 11: 0 dbm
    #[bits(2, default = 3)]
    pub rf_pwr: u8,

    /// Set LNA gain.
    ///
    /// 0: lower gain and current consumption
    ///
    /// 1: higher gain and current consumption
    #[bits(1, default = true)]
    pub lna_hcurr: bool,
}

impl Register for RfSetup {
    const ADDRESS: u8 = 0x06;
}

/// Status register.
///
/// Address = 0x07
#[bitfield(u8, order = Msb)]
pub struct Status {
    #[bits(1)]
    __: bool,

    /// Data Ready RX FIFO interrupt. Asserted when new data arrives
    /// on RX FIFO. Write 1 to clear bit.
    #[bits(1)]
    pub rx_dr: bool,

    /// Data Sent TX FIFO interrupt. Asserted when packet is transmitted on TX.
    /// If AUTO_ACK is activated, this bit is high only when ACK is received.
    /// Write 1 to clear bit.
    #[bits(1)]
    pub tx_ds: bool,

    /// Maximum number of TX retransmits interrupt. If MAX_RT is asserted,
    /// it must be reset to continue communication. Write 1 to clear bit.
    #[bits(1)]
    pub max_rt: bool,

    /// Data pipe number for last received payload in RX FIFO.
    ///
    /// 000-101: Data pipe number
    ///
    /// 110: Not used
    ///
    /// 111: RX FIFO empty
    #[bits(3, access = RO)]
    pub rx_p_no: u8,

    /// TX FIFO full flag. 0: not full, 1: TX FIFO full.
    #[bits(1, access = RO)]
    pub tx_full: bool,
}

impl Register for Status {
    const ADDRESS: u8 = 0x07;
}

/// Transmit observe register.
///
/// Address = 0x08
#[bitfield(u8, order = Msb)]
pub struct ObserveTx {
    /// Count lost packets. This counter is overflow protected to 15,
    /// and continues at max until reset. This counter is reset by writing
    /// to RF_CH.
    #[bits(4, access = RO)]
    pub plos_cnt: u8,

    /// Count retransmitted packets. The counter is reset when transmission
    /// of a new packet starts.
    #[bits(4, access = RO)]
    pub arc_cnt: u8,
}

impl Register for ObserveTx {
    const ADDRESS: u8 = 0x08;
}

/// Carrier detect register.
///
/// Address = 0x09
///
/// ```rust
/// use nrf24l01_commands::registers;
///
/// let reg = registers::Cd::from_bits(1);
/// assert_eq!(reg.cd(), true);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Cd {
    #[bits(7)]
    __: u8,

    /// Carrier detect. The carrier detect is a signal that is set high
    /// when an RF signal is detected inside the receiving frequency channel.
    #[bits(1, access = RO)]
    pub cd: bool,
}

impl Register for Cd {
    const ADDRESS: u8 = 0x09;
}

/// Receive address data pipe 0. TODO: 3-5 byte address.
///
/// Address = 0x0A
///
/// At reset: rx_addr_p0 = 0xE7E7E7E7E7
#[bitfield(u64, order = Msb)]
pub struct RxAddrP0 {
    #[bits(24)]
    __: u32,

    /// Receive address data pipe 0.
    ///
    /// At reset: 0xE7E7E7E7E7
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub rx_addr_p0: u64,
}

impl Register for RxAddrP0 {
    const ADDRESS: u8 = 0x0A;
}

impl RxAddrP0 {
    /// Convert into bytes ordered by LSByte first.
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

/// Receive address data pipe 1.
///
/// Address = 0x0B
///
/// At reset: rx_addr_p1 = 0xC2C2C2C2C2
#[bitfield(u64, order = Msb)]
pub struct RxAddrP1 {
    #[bits(24)]
    __: u32,

    /// Receive address data pipe 1.
    ///
    /// At reset: 0xC2C2C2C2C2
    #[bits(40, default = 0xC2C2C2C2C2)]
    pub rx_addr_p1: u64,
}

impl Register for RxAddrP1 {
    const ADDRESS: u8 = 0x0B;
}

impl RxAddrP1 {
    /// Convert into bytes ordered by LSByte first.
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

/// Receive address data pipe 2.
///
/// Address = 0x0C
///
/// At reset: rx_addr_p2 = 0xC3
#[bitfield(u8)]
pub struct RxAddrP2 {
    /// Receive address data pipe 2.
    ///
    /// At reset: 0xC3
    #[bits(8, default = 0xC3)]
    pub rx_addr_p2: u8,
}

impl Register for RxAddrP2 {
    const ADDRESS: u8 = 0x0C;
}

/// Receive address data pipe 3.
///
/// Address = 0x0D
///
/// At reset: rx_addr_p3 = 0xC4
#[bitfield(u8)]
pub struct RxAddrP3 {
    /// Receive address data pipe 3.
    ///
    /// At reset: 0xC4
    #[bits(8, default = 0xC4)]
    pub rx_addr_p3: u8,
}

impl Register for RxAddrP3 {
    const ADDRESS: u8 = 0x0D;
}

/// Receive address data pipe 4.
///
/// Address = 0x0E
///
/// At reset: rx_addr_p4 = 0xC5
#[bitfield(u8)]
pub struct RxAddrP4 {
    /// Receive address data pipe 4.
    ///
    /// At reset: 0xC5
    #[bits(8, default = 0xC5)]
    pub rx_addr_p4: u8,
}

impl Register for RxAddrP4 {
    const ADDRESS: u8 = 0x0E;
}

/// Receive address data pipe 5.
///
/// Address = 0x0F
///
/// At reset: rx_addr_p5 = 0xC6
#[bitfield(u8)]
pub struct RxAddrP5 {
    /// Receive address data pipe 5.
    ///
    /// At reset: 0xC6
    #[bits(8, default = 0xC6)]
    pub rx_addr_p5: u8,
}

impl Register for RxAddrP5 {
    const ADDRESS: u8 = 0x0F;
}

/// Transmit address. Used for PTX device only.
///
/// Address = 0x10
///
/// At reset: tx_addr = 0xE7E7E7E7E7
#[bitfield(u64, order = Msb)]
pub struct TxAddr {
    #[bits(24)]
    __: u32,

    /// Transmit address.
    ///
    /// At reset: 0xE7E7E7E7E7
    #[bits(40, default = 0xE7E7E7E7E7)]
    pub tx_addr: u64,
}

impl Register for TxAddr {
    const ADDRESS: u8 = 0x10;
}

impl TxAddr {
    /// Convert into bytes ordered by LSByte first.
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

/// RX payload width for data pipe 0.
///
/// Address = 0x11
#[bitfield(u8, order = Msb)]
pub struct RxPwP0 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 0. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p0: u8,
}

impl Register for RxPwP0 {
    const ADDRESS: u8 = 0x11;
}

/// RX payload width for data pipe 1.
///
/// Address = 0x12
#[bitfield(u8, order = Msb)]
pub struct RxPwP1 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 1. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p1: u8,
}

impl Register for RxPwP1 {
    const ADDRESS: u8 = 0x12;
}

/// RX payload width for data pipe 2.
///
/// Address = 0x13
#[bitfield(u8, order = Msb)]
pub struct RxPwP2 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 2. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p2: u8,
}

impl Register for RxPwP2 {
    const ADDRESS: u8 = 0x13;
}

/// RX payload width for data pipe 3.
///
/// Address = 0x14
#[bitfield(u8, order = Msb)]
pub struct RxPwP3 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 3. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p3: u8,
}

impl Register for RxPwP3 {
    const ADDRESS: u8 = 0x14;
}

/// RX payload width for data pipe 4.
///
/// Address = 0x15
#[bitfield(u8, order = Msb)]
pub struct RxPwP4 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 4. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p4: u8,
}

impl Register for RxPwP4 {
    const ADDRESS: u8 = 0x15;
}

/// RX payload width for data pipe 5.
///
/// Address = 0x16
#[bitfield(u8, order = Msb)]
pub struct RxPwP5 {
    #[bits(2)]
    __: u8,

    /// RX payload width for data pipe 5. 1 - 32 bytes.
    #[bits(6)]
    pub rx_pw_p5: u8,
}

impl Register for RxPwP5 {
    const ADDRESS: u8 = 0x16;
}

/// FIFO status register.
///
/// Address = 0x17
#[bitfield(u8, order = Msb)]
pub struct FifoStatus {
    #[bits(1)]
    __: bool,

    /// Reuse last transmitted data packet if set high.
    /// The packet is repeatedly retransmitted as long as CE is high.
    /// TX_REUSE is set by the REUSE_TX_PL command and reset by
    /// W_TX_PAYLOAD or FLUSH_TX.
    #[bits(1, access = RO)]
    pub tx_reuse: bool,

    /// TX FIFO full flag. 0: TX FIFO available locations, 1: TX FIFO full.
    #[bits(1, access = RO)]
    pub tx_full: bool,

    /// TX FIFO empty flag. 0: Data in TX FIFO, 1: TX FIFO empty.
    #[bits(1, access = RO)]
    pub tx_empty: bool,

    #[bits(2)]
    __: u8,

    /// RX FIFO full flag. 0: RX FIFO available locations, 1: RX FIFO full.
    #[bits(1, access = RO)]
    pub rx_full: bool,

    /// RX FIFO empty flag. 0: Data in RX FIFO, 1: RX FIFO empty.
    #[bits(1, access = RO)]
    pub rx_empty: bool,
}

impl Register for FifoStatus {
    const ADDRESS: u8 = 0x17;
}

/// Enable dynamic payload length.
///
/// Address = 0x1C
#[bitfield(u8, order = Msb)]
pub struct Dynpd {
    #[bits(2)]
    __: u8,

    /// Enable dynamic payload length for data pipe 5.
    #[bits(1)]
    pub dpl_p5: bool,
    /// Enable dynamic payload length for data pipe 4.
    #[bits(1)]
    pub dpl_p4: bool,
    /// Enable dynamic payload length for data pipe 3.
    #[bits(1)]
    pub dpl_p3: bool,
    /// Enable dynamic payload length for data pipe 2.
    #[bits(1)]
    pub dpl_p2: bool,
    /// Enable dynamic payload length for data pipe 1.
    #[bits(1)]
    pub dpl_p1: bool,
    /// Enable dynamic payload length for data pipe 0.
    #[bits(1)]
    pub dpl_p0: bool,
}

impl Register for Dynpd {
    const ADDRESS: u8 = 0x1C;
}

/// Feature register. To activate this register, use the ACTIVATE command.
/// To deactivate this register, use the ACTIVATE command again.
///
/// Address = 0x01D
#[bitfield(u8, order = Msb)]
pub struct Feature {
    #[bits(5)]
    __: u8,

    /// Enables dynamic payload length feature.
    #[bits(1)]
    pub en_dpl: bool,

    /// Enables payload with ACK feature.
    #[bits(1)]
    pub en_ack_pay: bool,

    /// Enables the W_TX_PAYLOAD_NOACK command.
    #[bits(1)]
    pub en_dyn_ack: bool,
}

impl Register for Feature {
    const ADDRESS: u8 = 0x1D;
}
