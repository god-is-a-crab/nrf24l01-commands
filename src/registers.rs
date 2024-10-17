//! Register bitfields for the nRF24L01.
//!
//! ## Example with the CONFIG register
//! ```rust
//! use nrf24l01_commands::registers;
//!
//! // Default value
//! let reg = registers::Config::new();
//! assert_eq!(reg.into_bits(), 0b0000_1000);
//!
//! // Read fields
//! let reg = registers::Config::from_bits(0b0000_0110);
//! assert!(!reg.mask_rx_dr());
//! assert!(!reg.mask_tx_ds());
//! assert!(!reg.mask_max_rt());
//! assert!(!reg.en_crc());
//! assert!(reg.crco());
//! assert!(reg.pwr_up());
//! assert!(!reg.prim_rx());
//!
//! // Write fields
//! let reg = registers::Config::new()
//!     .with_mask_rx_dr(true)
//!     .with_mask_tx_ds(false)
//!     .with_mask_max_rt(false)
//!     .with_en_crc(false)
//!     .with_crco(true)
//!     .with_pwr_up(true)
//!     .with_prim_rx(false);
//! assert_eq!(reg.into_bits(), 0b0100_0110);
//! ```
use bitfield_struct::bitfield;

/// A trait for nRF24L01 registers. Defines the register's address.
pub trait Register {
    /// Register address.
    const ADDRESS: u8;
}

/// # CONFIG register
///
/// Address = `0x00`
///
/// ## Fields
///
/// #### `mask_rx_dr` | bit 6
/// Mask/unmask interrupt caused by __RX_DR__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `mask_tx_ds` | bit 5
/// Mask/unmask interrupt caused by __TX_DS__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `mask_max_rt` | bit 4
/// Mask/unmask interrupt caused by __MAX_RT__.
///
/// `0`: unmasked, interrupt reflected on IRQ
///
/// `1`: masked, interrupt not reflected on IRQ
///
/// #### `en_crc` | bit 3
/// Enable/disable CRC. Default value: `1` (enabled)
///
/// #### `crco` | bit 2
/// CRC encoding scheme.
///
/// `0`: 1 byte
///
/// `1`: 2 byte
///
/// #### `pwr_up` | bit 1
/// Power down/up.
///
/// `0`: Power down
///
/// `1`: Power up
///
/// #### `prim_rx` | bit 0
/// Set primary TX/RX.
///
/// `0`: primary TX
///
/// `1`: primary RX
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::Config::new();
/// assert_eq!(reg.into_bits(), 0b0000_1000);
///
/// // Write fields
/// let reg = registers::Config::new()
///     .with_mask_rx_dr(true)
///     .with_mask_tx_ds(false)
///     .with_mask_max_rt(false)
///     .with_en_crc(false)
///     .with_crco(true)
///     .with_pwr_up(true)
///     .with_prim_rx(false);
/// assert_eq!(reg.into_bits(), 0b0100_0110);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Config {
    #[bits(1)]
    __: bool,

    /// Mask/unmask interrupt caused by __RX_DR__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_rx_dr: bool,

    /// Mask/unmask interrupt caused by __TX_DS__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_tx_ds: bool,

    /// Mask/unmask interrupt caused by __MAX_RT__.
    ///
    /// `0`: unmasked, interrupt reflected on IRQ
    ///
    /// `1`: masked, interrupt not reflected on IRQ
    #[bits(1)]
    pub mask_max_rt: bool,

    /// Enable/disable CRC. Default value: `1` (enabled)
    #[bits(1, default = true)]
    pub en_crc: bool,

    /// CRC encoding scheme.
    ///
    /// `0`: 1 byte
    ///
    /// `1`: 2 byte
    #[bits(1)]
    pub crco: bool,

    /// Power down/up.
    ///
    /// `0`: Power down
    ///
    /// `1`: Power up
    #[bits(1)]
    pub pwr_up: bool,

    /// Set primary TX/RX.
    ///
    /// `0`: primary TX
    ///
    /// `1`: primary RX
    #[bits(1)]
    pub prim_rx: bool,
}

impl Register for Config {
    const ADDRESS: u8 = 0x00;
}

/// # EN_AA register
/// Enable 'Auto Acknowledgement' on data pipes 0-5.
///
/// Address = `0x01`
///
/// ## Fields
/// All fields default to 1.
///
/// #### `enaa_pN` | bit `N`
/// Enable 'Auto Acknowledgement' on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::EnAa::new();
/// assert_eq!(reg.into_bits(), 0b0011_1111);
///
/// // Write fields
/// let reg = registers::EnAa::new()
///     .with_enaa_p5(true)
///     .with_enaa_p4(true)
///     .with_enaa_p3(false)
///     .with_enaa_p2(false)
///     .with_enaa_p1(false)
///     .with_enaa_p0(false);
/// assert_eq!(reg.into_bits(), 0b0011_0000);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnAa {
    #[bits(2)]
    __: u8,
    /// Enable 'Auto Acknowledgement' on data pipe 5.
    #[bits(1, default = true)]
    pub enaa_p5: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 4.
    #[bits(1, default = true)]
    pub enaa_p4: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 3.
    #[bits(1, default = true)]
    pub enaa_p3: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 2.
    #[bits(1, default = true)]
    pub enaa_p2: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 1.
    #[bits(1, default = true)]
    pub enaa_p1: bool,
    /// Enable 'Auto Acknowledgement' on data pipe 0.
    #[bits(1, default = true)]
    pub enaa_p0: bool,
}

impl Register for EnAa {
    const ADDRESS: u8 = 0x01;
}

/// # EN_RXADDR register
/// Enable RX address on data pipes 0-5.
///
/// Address = `0x02`
///
/// ## Fields
///
/// `erx_p0` and `erx_p1` default to 1.
///
/// #### `erx_pN` | bit `N`
/// Enable RX adddress on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::EnRxaddr::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = registers::EnRxaddr::new()
///     .with_erx_p5(true)
///     .with_erx_p4(false)
///     .with_erx_p3(false)
///     .with_erx_p2(false)
///     .with_erx_p1(true)
///     .with_erx_p0(false);
/// assert_eq!(reg.into_bits(), 0b0010_0010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct EnRxaddr {
    #[bits(2)]
    __: u8,
    /// Enable RX address for data pipe 5.
    #[bits(1)]
    pub erx_p5: bool,
    /// Enable RX address for data pipe 4.
    #[bits(1)]
    pub erx_p4: bool,
    /// Enable RX address for data pipe 3.
    #[bits(1)]
    pub erx_p3: bool,
    /// Enable RX address for data pipe 2.
    #[bits(1)]
    pub erx_p2: bool,
    /// Enable RX address for data pipe 1.
    #[bits(1, default = true)]
    pub erx_p1: bool,
    /// Enable RX address for data pipe 0.
    #[bits(1, default = true)]
    pub erx_p0: bool,
}

impl Register for EnRxaddr {
    const ADDRESS: u8 = 0x02;
}

/// # SETUP_AW register
/// Set up address width. This applies to [`TxAddr`] and all RX addresses for data pipes.
///
/// Address = `0x03`
///
/// ## Fields
///
/// #### `aw` | bits 1:0
/// Address width. Default value: `11` (5 byte address).
///
/// `00`: Illegal
///
/// `01`: 3 bytes
///
/// `10`: 4 bytes
///
/// `11`: 5 bytes
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::SetupAw::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = registers::SetupAw::new().with_aw(0b10);
/// assert_eq!(reg.into_bits(), 0b0000_0010);
/// ```
#[bitfield(u8, order = Msb)]
pub struct SetupAw {
    #[bits(6)]
    __: u8,

    /// Address width. Default value: `11` (5 byte address).
    ///
    /// `00`: Illegal
    ///
    /// `01`: 3 bytes
    ///
    /// `10`: 4 bytes
    ///
    /// `11`: 5 bytes
    #[bits(2, default = 3)]
    pub aw: u8,
}

impl Register for SetupAw {
    const ADDRESS: u8 = 0x03;
}

/// # SETUP_RETR register
/// Set up 'Automatic Retransmission'.
///
/// Address = `0x04`
///
/// ## Fields
///
/// #### `ard` | bits 7:4
/// Auto retransmit delay.
///
/// `0000`: Wait 250µS
///
/// `0001`: Wait 500µS
///
/// `0010`: Wait 750µS
///
/// ……
///
/// `1111`: Wait 4000µS
///
/// #### `arc` | bits 3:0
/// Maximum auto retransmits. Default value: `0011` (3 retransmits)
///
/// `0000`: Auto retransmit disabled
///
/// `0001`: Up to 1 retransmit
///
/// `0010`: Up to 2 retransmits
///
/// ……
///
/// `1111`: Up to 15 retransmits
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::SetupRetr::new();
/// assert_eq!(reg.into_bits(), 0b0000_0011);
///
/// // Write fields
/// let reg = registers::SetupRetr::new()
///     .with_ard(0b10)
///     .with_arc(0b1111);
/// assert_eq!(reg.into_bits(), 0b0010_1111);
/// ```
#[bitfield(u8, order = Msb)]
pub struct SetupRetr {
    /// Auto retransmit delay.
    ///
    /// `0000`: Wait 250µS
    ///
    /// `0001`: Wait 500µS
    ///
    /// `0010`: Wait 750µS
    ///
    /// ……
    ///
    /// `1111`: Wait 4000µS
    #[bits(4)]
    pub ard: u8,

    /// Maximum auto retransmits. Default value: `0011` (3 retransmits)
    ///
    /// `0000`: Auto retransmit disabled
    ///
    /// `0001`: Up to 1 retransmit
    ///
    /// `0010`: Up to 2 retransmits
    ///
    /// ……
    ///
    /// `1111`: Up to 15 retransmits
    #[bits(4, default = 3)]
    pub arc: u8,
}

impl Register for SetupRetr {
    const ADDRESS: u8 = 0x04;
}

/// # RF_CH register
/// Set RF channel.
///
/// Address = `0x05`
///
/// ## Fields
/// #### `rf_ch` | bits 6:0
/// Sets the frequency channel to operate on 0 - 125. Default value: `2`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RfCh::new();
/// assert_eq!(reg.into_bits(), 0b0000_0010);
///
/// // Write fields
/// let reg = registers::RfCh::new().with_rf_ch(89);
/// assert_eq!(reg.into_bits(), 89);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RfCh {
    #[bits(1)]
    __: bool,

    /// Sets the frequency channel to operate on 0 - 125. Default value: `2`.
    #[bits(7, default = 2)]
    pub rf_ch: u8,
}

impl Register for RfCh {
    const ADDRESS: u8 = 0x05;
}

/// # RF_SETUP register
/// Set RF air data rate, output power and LNA gain.
///
/// Address = `0x06`
///
/// ## Fields
/// #### `pll_lock` | bit 4
/// Force PLL lock signal. Only used in test.
///
/// #### `rf_dr` | bit 3
/// Air data rate. Default value: `1` (2 Mbps)
///
/// `0`: 1 Mbps
///
/// `1`: 2 Mbps
///
/// #### `rf_pwr` | bits 2:1
/// RF output power in TX mode. Default value: `11` (0 dBm)
///
/// `00`: -18 dBm
///
/// `01`: -12 dBm
///
/// `10`: -6 dBm
///
/// `11`: 0 dbm
///
/// #### `lna_hcurr` | bit 0
/// Set LNA gain. Default value: `1` (higher gain).
///
/// `0`: lower gain and current consumption
///
/// `1`: higher gain and current consumption
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RfSetup::new();
/// assert_eq!(reg.into_bits(), 0b0000_1111);
///
/// // Write fields
/// let reg = registers::RfSetup::new()
///     .with_pll_lock(false)
///     .with_rf_dr(false)
///     .with_rf_pwr(0b10)
///     .with_lna_hcurr(true);
/// assert_eq!(reg.into_bits(), 0b0000_0101);
/// ```
#[bitfield(u8, order = Msb)]
pub struct RfSetup {
    #[bits(3)]
    __: u8,

    /// Force PLL lock signal. Only used in test.
    #[bits(1)]
    pub pll_lock: bool,

    /// Air data rate. Default value: `1` (2 Mbps)
    ///
    /// `0`: 1 Mbps
    ///
    /// `1`: 2 Mbps
    #[bits(1, default = true)]
    pub rf_dr: bool,

    /// RF output power in TX mode. Default value: `11` (0 dBm)
    ///
    /// `00`: -18 dBm
    ///
    /// `01`: -12 dBm
    ///
    /// `10`: -6 dBm
    ///
    /// `11`: 0 dbm
    #[bits(2, default = 3)]
    pub rf_pwr: u8,

    /// Set LNA gain. Default value: `1` (higher gain).
    ///
    /// `0`: lower gain and current consumption
    ///
    /// `1`: higher gain and current consumption
    #[bits(1, default = true)]
    pub lna_hcurr: bool,
}

impl Register for RfSetup {
    const ADDRESS: u8 = 0x06;
}

/// # STATUS register
///
/// Address = `0x07`
///
/// ## Fields
/// #### `rx_dr` | bit 6
/// Data ready RX FIFO interrupt. Asserted when new data arrives in RX FIFO. Write 1 to clear bit.
///
/// #### `tx_ds` | bit 5
/// Data sent TX FIFO interrupt. Asserted when packet is transmitted. If AUTO_ACK is activated, ACK must be received before interrupt goes high. Write 1 to clear bit.
///
/// #### `max_rt` | bit 4
/// Maximum number of TX retransmits interrupt. If MAX_RT is asserted it must be cleared before communication can continue. Write 1 to clear bit.
///
/// #### `rx_p_no` | bits 3:1
/// Data pipe number for the payload available from reading RX FIFO. This field is read-only.
///
/// `000`-`101`: Data pipe number
///
/// `110`: Not used
///
/// `111`: RX FIFO empty
///
/// #### `tx_full` | bit 0
/// TX FIFO full flag. This field is read-only.
///
/// `0`: Not full
///
/// `1`: TX FIFO full
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::Status::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = registers::Status::from_bits(0b0011_0101);
/// assert!(!reg.rx_dr());
/// assert!(reg.tx_ds());
/// assert!(reg.max_rt());
/// assert_eq!(reg.rx_p_no(), 2);
/// assert!(reg.tx_full());
///
/// // Write fields
/// let reg = registers::Status::new()
///     .with_rx_dr(false)
///     .with_tx_ds(true)
///     .with_max_rt(false);
/// assert_eq!(reg.into_bits(), 0b0010_0000);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Status {
    #[bits(1)]
    __: bool,

    /// Data ready RX FIFO interrupt. Asserted when new data arrives in RX FIFO. Write 1 to clear bit.
    #[bits(1)]
    pub rx_dr: bool,

    /// Data sent TX FIFO interrupt. Asserted when packet is transmitted. If AUTO_ACK is activated, ACK must be received before interrupt goes high. Write 1 to clear bit.
    #[bits(1)]
    pub tx_ds: bool,

    /// Maximum number of TX retransmits interrupt. If MAX_RT is asserted it must be cleared before communication can continue. Write 1 to clear bit.
    #[bits(1)]
    pub max_rt: bool,

    /// Data pipe number for the payload available from reading RX FIFO. This field is read-only.
    ///
    /// `000`-`101`: Data pipe number
    ///
    /// `110`: Not used
    ///
    /// `111`: RX FIFO empty
    #[bits(3, access = RO)]
    pub rx_p_no: u8,

    /// TX FIFO full flag. This field is read-only.
    ///
    /// `0`: Not full
    ///
    /// `1`: TX FIFO full
    #[bits(1, access = RO)]
    pub tx_full: bool,
}

impl Register for Status {
    const ADDRESS: u8 = 0x07;
}

/// # OBSERVE_TX register
/// Transmit observe register.
///
/// Address = `0x08`
///
/// #### `plos_cnt` | bits 7:4
/// Count lost packets. This counter is overflow protected to 15,
/// and continues at max until reset. This counter is reset by writing
/// to [`RfCh`]. This field is read-only.
///
/// #### `arc_cnt` | 3:0
/// Count retransmitted packets. The counter is reset when transmission
/// of a new packet starts. This field is read-only.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::ObserveTx::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = registers::ObserveTx::from_bits(0b1010_1100);
/// assert_eq!(reg.plos_cnt(), 10);
/// assert_eq!(reg.arc_cnt(), 12);
/// ```
#[bitfield(u8, order = Msb)]
pub struct ObserveTx {
    /// Count lost packets. This counter is overflow protected to 15,
    /// and continues at max until reset. This counter is reset by writing
    /// to __RF_CH__. This field is read-only.
    #[bits(4, access = RO)]
    pub plos_cnt: u8,

    /// Count retransmitted packets. The counter is reset when transmission
    /// of a new packet starts. This field is read-only.
    #[bits(4, access = RO)]
    pub arc_cnt: u8,
}

impl Register for ObserveTx {
    const ADDRESS: u8 = 0x08;
}

/// # CD register
/// Carrier detect register.
///
/// Address = `0x09`
///
/// ## Fields
/// #### `cd` | bit 0
/// Carrier detect. The carrier detect is a signal that is set high
/// when an RF signal is detected inside the receiving frequency channel. This field is read-only.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// let reg = registers::Cd::from_bits(1);
/// assert!(reg.cd());
/// ```
#[bitfield(u8, order = Msb)]
pub struct Cd {
    #[bits(7)]
    __: u8,

    /// Carrier detect. The carrier detect is a signal that is set high
    /// when an RF signal is detected inside the receiving frequency channel. This field is read-only.
    #[bits(1, access = RO)]
    pub cd: bool,
}

impl Register for Cd {
    const ADDRESS: u8 = 0x09;
}

/// # RX_ADDR_P0 register
/// RX address data pipe 0.
///
/// Address = `0x0A`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `rx_addr_p0` | bits 39:0
/// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP0::<4>::new();
/// assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
///
/// // Write fields
/// let reg = registers::RxAddrP0::<5>::new().with_rx_addr_p0(0xC2840DF659);
/// assert_eq!(reg.into_bits(), 0xC2840DF659);
///
/// // Convert to little-endian bytes
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // 3 byte address width
/// let reg = registers::RxAddrP0::<3>::new().with_rx_addr_p0(0xC2840DF659);
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct RxAddrP0<const N: usize>(RxAddrP0Fields);

#[bitfield(u64, order = Msb)]
struct RxAddrP0Fields {
    #[bits(24)]
    __: u32,

    /// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
    #[bits(40, default = 0xE7E7E7E7E7)]
    rx_addr_p0: u64,
}

impl<const N: usize> Register for RxAddrP0<N> {
    const ADDRESS: u8 = 0x0A;
}

/// Convert u64 address to little-endian bytes.
/// Const parameter `N`: address width in bytes. Constraint: `N` in {3, 4, 5}.
#[inline(always)]
const fn address_into_bytes<const N: usize>(addr: u64) -> [u8; N] {
    let le_bytes: [u8; 8] = addr.to_le_bytes();
    let mut bytes = [0; N];
    let mut i = 0;
    while i < N {
        bytes[i] = le_bytes[i];
        i += 1;
    }
    bytes
}

impl<const N: usize> RxAddrP0<N> {
    /// Creates a new default initialized bitfield.
    pub const fn new() -> Self {
        Self(RxAddrP0Fields::new())
    }

    /// Convert from bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(RxAddrP0Fields::from_bits(bits))
    }

    /// Convert into bits.
    pub const fn into_bits(self) -> u64 {
        self.0.into_bits()
    }

    /// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
    pub const fn rx_addr_p0(&self) -> u64 {
        self.0.rx_addr_p0()
    }

    /// RX address data pipe 0. Default value: `0xE7E7E7E7E7`.
    pub const fn with_rx_addr_p0(mut self, value: u64) -> Self {
        self.0 = self.0.with_rx_addr_p0(value);
        self
    }

    /// Convert into bytes ordered by LSByte first.
    pub const fn into_bytes(self) -> [u8; N] {
        address_into_bytes(self.0 .0)
    }
}

impl<const N: usize> Default for RxAddrP0<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// # RX_ADDR_P1 register
/// RX address data pipe 1.
///
/// Address = `0x0B`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `rx_addr_p1` | bits 39:0
/// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP1::<4>::new();
/// assert_eq!(reg.into_bits(), 0xC2C2C2C2C2);
///
/// // Write fields
/// let reg = registers::RxAddrP1::<5>::new().with_rx_addr_p1(0xC2840DF659);
/// assert_eq!(reg.into_bits(), 0xC2840DF659);
///
/// // Convert to little-endian bytes
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // 3 byte address width
/// let reg = registers::RxAddrP1::<3>::new().with_rx_addr_p1(0xC2840DF659);
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct RxAddrP1<const N: usize>(RxAddrP1Fields);

#[bitfield(u64, order = Msb)]
struct RxAddrP1Fields {
    #[bits(24)]
    __: u32,

    /// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
    #[bits(40, default = 0xC2C2C2C2C2)]
    rx_addr_p1: u64,
}

impl<const N: usize> Register for RxAddrP1<N> {
    const ADDRESS: u8 = 0x0B;
}

impl<const N: usize> RxAddrP1<N> {
    /// Creates a new default initialized bitfield.
    pub const fn new() -> Self {
        Self(RxAddrP1Fields::new())
    }

    /// Convert from bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(RxAddrP1Fields::from_bits(bits))
    }

    /// Convert into bits.
    pub const fn into_bits(self) -> u64 {
        self.0.into_bits()
    }

    /// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
    pub const fn rx_addr_p1(&self) -> u64 {
        self.0.rx_addr_p1()
    }

    /// RX address data pipe 1. Default value: `0xC2C2C2C2C2`.
    pub const fn with_rx_addr_p1(mut self, value: u64) -> Self {
        self.0 = self.0.with_rx_addr_p1(value);
        self
    }

    /// Convert into bytes ordered by LSByte first.
    pub const fn into_bytes(self) -> [u8; N] {
        address_into_bytes(self.0 .0)
    }
}

impl<const N: usize> Default for RxAddrP1<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// # RX_ADDR_P2 register
/// RX address data pipe 2. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0C`
///
/// ## Fields
/// #### `rx_addr_p2` | bits 7:0
/// RX address data pipe 2. Default value: `0xC3`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP2::new();
/// assert_eq!(reg.into_bits(), 0xC3);
///
/// // Write fields
/// let reg = registers::RxAddrP2::new().with_rx_addr_p2(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP2 {
    /// RX address data pipe 2. Default value: `0xC3`.
    #[bits(8, default = 0xC3)]
    pub rx_addr_p2: u8,
}

impl Register for RxAddrP2 {
    const ADDRESS: u8 = 0x0C;
}

/// # RX_ADDR_P3 register
/// RX address data pipe 3. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0D`
///
/// ## Fields
/// #### `rx_addr_p3` | bits 7:0
/// RX address data pipe 3. Default value: `0xC4`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP3::new();
/// assert_eq!(reg.into_bits(), 0xC4);
///
/// // Write fields
/// let reg = registers::RxAddrP3::new().with_rx_addr_p3(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP3 {
    /// RX address data pipe 3. Default value: `0xC4`.
    #[bits(8, default = 0xC4)]
    pub rx_addr_p3: u8,
}

impl Register for RxAddrP3 {
    const ADDRESS: u8 = 0x0D;
}

/// # RX_ADDR_P4 register
/// RX address data pipe 4. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0E`
///
/// ## Fields
/// #### `rx_addr_p4` | bits 7:0
/// RX address data pipe 4. Default value: `0xC5`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP4::new();
/// assert_eq!(reg.into_bits(), 0xC5);
///
/// // Write fields
/// let reg = registers::RxAddrP4::new().with_rx_addr_p4(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP4 {
    /// RX address data pipe 4. Default value: `0xC5`.
    #[bits(8, default = 0xC5)]
    pub rx_addr_p4: u8,
}

impl Register for RxAddrP4 {
    const ADDRESS: u8 = 0x0E;
}

/// # RX_ADDR_P5 register
/// RX address data pipe 5. Only LSByte is stored.
/// MSBytes are equal to [`RxAddrP1`] bits 39:8.
///
/// Address = `0x0F`
///
/// ## Fields
/// #### `rx_addr_p5` | bits 7:0
/// RX address data pipe 5. Default value: `0xC6`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxAddrP5::new();
/// assert_eq!(reg.into_bits(), 0xC6);
///
/// // Write fields
/// let reg = registers::RxAddrP5::new().with_rx_addr_p5(172);
/// assert_eq!(reg.into_bits(), 172);
/// ```
#[bitfield(u8)]
pub struct RxAddrP5 {
    /// RX address data pipe 5. Default value: `0xC6`.
    #[bits(8, default = 0xC6)]
    pub rx_addr_p5: u8,
}

impl Register for RxAddrP5 {
    const ADDRESS: u8 = 0x0F;
}

/// # TX_ADDR register
/// TX address. Set [`RxAddrP0`] equal to this address to handle ACK automatically.
///
/// Address = `0x10`
///
/// Const parameter `N`: address width in bytes.
/// <div class="warning">
/// N must be of {3, 4, 5}.
/// </div>
///
/// ## Fields
/// #### `tx_addr` | bits 39:0
/// TX address. Default value: `0xE7E7E7E7E7`.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::TxAddr::<4>::new();
/// assert_eq!(reg.into_bits(), 0xE7E7E7E7E7);
///
/// // Write fields
/// let reg = registers::TxAddr::<5>::new().with_tx_addr(0xC2840DF659);
/// assert_eq!(reg.into_bits(), 0xC2840DF659);
///
/// // Convert to little-endian bytes
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D, 0x84, 0xC2]);
///
/// // 3 byte address width
/// let reg = registers::TxAddr::<3>::new().with_tx_addr(0xC2840DF659);
/// assert_eq!(reg.into_bytes(), [0x59, 0xF6, 0x0D]);
/// ```
#[derive(Copy, Clone)]
pub struct TxAddr<const N: usize>(TxAddrFields);

#[bitfield(u64, order = Msb)]
struct TxAddrFields {
    #[bits(24)]
    __: u32,

    /// TX address. Default value: `0xE7E7E7E7E7`.
    #[bits(40, default = 0xE7E7E7E7E7)]
    tx_addr: u64,
}

impl<const N: usize> Register for TxAddr<N> {
    const ADDRESS: u8 = 0x10;
}

impl<const N: usize> TxAddr<N> {
    /// Creates a new default initialized bitfield.
    pub const fn new() -> Self {
        Self(TxAddrFields::new())
    }

    /// Convert from bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(TxAddrFields::from_bits(bits))
    }

    /// Convert into bits.
    pub const fn into_bits(self) -> u64 {
        self.0.into_bits()
    }

    /// TX address. Default value: `0xE7E7E7E7E7`.
    pub const fn tx_addr(&self) -> u64 {
        self.0.tx_addr()
    }

    /// TX address. Default value: `0xE7E7E7E7E7`.
    pub const fn with_tx_addr(mut self, value: u64) -> Self {
        self.0 = self.0.with_tx_addr(value);
        self
    }

    /// Convert into bytes ordered by LSByte first.
    pub const fn into_bytes(self) -> [u8; N] {
        address_into_bytes(self.0 .0)
    }
}

impl<const N: usize> Default for TxAddr<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// # RX_PW_P0 register
/// RX payload width for data pipe 0.
///
/// Address = `0x11`
///
/// ## Fields
/// #### `rx_pw_p0` | bits 7:0
/// RX payload width for data pipe 0. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP0::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP0::new().with_rx_pw_p0(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # RX_PW_P1 register
/// RX payload width for data pipe 1.
///
/// Address = `0x12`
///
/// ## Fields
/// #### `rx_pw_p1` | bits 7:0
/// RX payload width for data pipe 1. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP1::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP1::new().with_rx_pw_p1(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # RX_PW_P2 register
/// RX payload width for data pipe 2.
///
/// Address = `0x13`
///
/// ## Fields
/// #### `rx_pw_p2` | bits 7:0
/// RX payload width for data pipe 2. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP2::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP2::new().with_rx_pw_p2(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # RX_PW_P3 register
/// RX payload width for data pipe 3.
///
/// Address = `0x14`
///
/// ## Fields
/// #### `rx_pw_p3` | bits 7:0
/// RX payload width for data pipe 3. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP3::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP3::new().with_rx_pw_p3(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # RX_PW_P4 register
/// RX payload width for data pipe 4.
///
/// Address = `0x15`
///
/// ## Fields
/// #### `rx_pw_p4` | bits 7:0
/// RX payload width for data pipe 4. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP4::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP4::new().with_rx_pw_p4(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # RX_PW_P5 register
/// RX payload width for data pipe 5.
///
/// Address = `0x16`
///
/// ## Fields
/// #### `rx_pw_p5` | bits 7:0
/// RX payload width for data pipe 5. 1 - 32 bytes.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::RxPwP5::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::RxPwP5::new().with_rx_pw_p5(31);
/// assert_eq!(reg.into_bits(), 31);
/// ```
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

/// # FIFO_STATUS register
/// Status of TX/RX FIFOs.
///
/// Address = `0x17`
///
/// ## Fields
/// All fields are read-only.
///
/// #### `tx_reuse` | bit 6
/// Reuse last transmitted data packet if set high.
/// The packet is repeatedly retransmitted as long as CE is high.
/// TX_REUSE is set by the [`REUSE_TX_PL`][crate::commands::ReuseTxPl] command and reset by
/// [`W_TX_PAYLOAD`][crate::commands::WTxPayloadNoack] or [`FLUSH_TX`][crate::commands::FlushTx].
///
/// #### `tx_full` | bit 5
/// TX FIFO full flag.
///
/// #### `tx_empty` | bit 4
/// TX FIFO empty flag.
///
/// #### `rx_full` | bit 1
/// RX FIFO full flag.
///
/// #### `rx_empty` | bit 0
/// RX FIFO empty flag.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::FifoStatus::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Read fields
/// let reg = registers::FifoStatus::from_bits(0b0010_0010);
/// assert!(!reg.tx_reuse());
/// assert!(reg.tx_full());
/// assert!(!reg.tx_empty());
/// assert!(reg.rx_full());
/// assert!(!reg.rx_empty());
/// ```
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

    /// TX FIFO full flag.
    #[bits(1, access = RO)]
    pub tx_full: bool,

    /// TX FIFO empty flag.
    #[bits(1, access = RO)]
    pub tx_empty: bool,

    #[bits(2)]
    __: u8,

    /// RX FIFO full flag.
    #[bits(1, access = RO)]
    pub rx_full: bool,

    /// RX FIFO empty flag.
    #[bits(1, access = RO)]
    pub rx_empty: bool,
}

impl Register for FifoStatus {
    const ADDRESS: u8 = 0x17;
}

/// # DYNPD register
/// Enable dynamic payload length for data pipes 0-5.
///
/// Address = `0x1C`
///
/// ## Fields
///
/// #### `dpl_pN` | bit `N`
/// Enable dynamic payload length on data pipes `N` = 0-5.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::Dynpd::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::Dynpd::new()
///     .with_dpl_p5(true)
///     .with_dpl_p4(false)
///     .with_dpl_p3(false)
///     .with_dpl_p2(false)
///     .with_dpl_p1(true)
///     .with_dpl_p0(false);
/// assert_eq!(reg.into_bits(), 0b0010_0010);
/// ```
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

/// # FEATURE register
/// Enable features 'Dynamic Payload Length', 'Payload with ACK' and 'W_TX_PAYLOAD_NO_ACK' command.
/// To activate this register, use the ACTIVATE command.
/// To deactivate this register, use the ACTIVATE command again.
///
/// Address = `0x01D`
///
/// ## Fields
/// #### `en_dpl` | bit 2
/// Enables _Dynamic Payload Length_.
///
/// #### `en_ack_pay` | bit 1
/// Enables _Payload with ACK_.
///
/// #### `en_dyn_ack` | bit 0
/// Enables 'W_TX_PAYLOAD_NO_ACK' command.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::registers;
///
/// // Default value
/// let reg = registers::Feature::new();
/// assert_eq!(reg.into_bits(), 0);
///
/// // Write fields
/// let reg = registers::Feature::new()
///     .with_en_dpl(false)
///     .with_en_ack_pay(true)
///     .with_en_dyn_ack(true);
/// assert_eq!(reg.into_bits(), 0b0000_0011);
/// ```
#[bitfield(u8, order = Msb)]
pub struct Feature {
    #[bits(5)]
    __: u8,

    /// Enables _Dynamic Payload Length_.
    #[bits(1)]
    pub en_dpl: bool,

    /// Enables _Payload with ACK_.
    #[bits(1)]
    pub en_ack_pay: bool,

    /// Enables 'W_TX_PAYLOAD_NO_ACK' command.
    #[bits(1)]
    pub en_dyn_ack: bool,
}

impl Register for Feature {
    const ADDRESS: u8 = 0x1D;
}
