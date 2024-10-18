//! Generate SPI byte sequences for nRF24L01 commands.
//!
//! ## Example with writing the [`CONFIG`][registers::Config] register
//! ```rust
//! use nrf24l01_commands::{registers, commands};
//!
//! let config = registers::Config::new()
//!     .with_mask_rx_dr(true)
//!     .with_mask_tx_ds(false)
//!     .with_mask_max_rt(false)
//!     .with_en_crc(false)
//!     .with_crco(true)
//!     .with_pwr_up(true)
//!     .with_prim_rx(false);
//! let write_command = commands::WRegister(config);
//! let spi_bytes = write_command.bytes();
//! assert_eq!(spi_bytes, [0b0010_0000, 0b0100_0110]);
//! ```
use crate::registers::{self, Register};
use core::marker::PhantomData;

/// A trait for nRF24L01 commands. Defines the command's _command word_.
pub trait Command {
    /// Command word.
    const WORD: u8;
}

/// # R_REGISTER command
/// Read a register.
///
/// #### Type Parameter `R`
/// Register type.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{registers, commands};
///
/// // Generate SPI byte sequence for R_REGISTER on FIFO_STATUS register.
/// let bytes = commands::RRegister::<registers::FifoStatus>::bytes();
/// assert_eq!(bytes, [0 | 0x17, 0]);
/// ```
pub struct RRegister<R>(PhantomData<R>);

/// # W_REGISTER command
/// Write a register.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::{registers, commands};
///
/// // Generate SPI byte sequence for W_REGISTER on RF_CH register.
/// let rf_ch = registers::RfCh::new().with_rf_ch(85);
/// let bytes = commands::WRegister(rf_ch).bytes();
/// assert_eq!(bytes, [0b0010_0000 | 0x05, 85]);
///
/// // Generate SPI byte sequence for W_REGISTER on TX_ADDR register.
/// let tx_addr = registers::TxAddr::<5>::new().with_tx_addr(0x61DE7C320B);
/// let bytes = commands::WRegister(tx_addr).bytes();
/// assert_eq!(bytes, [0b0010_0000 | 0x10, 0x0B, 0x32, 0x7C, 0xDE, 0x61]);
/// ```
pub struct WRegister<R>(
    /// Register to write.
    pub R,
);

/// # R_RX_PAYLOAD command
/// Read RX payload.
///
/// #### Const Parameter `N`
/// Width of RX payload.
///
/// <div class="warning">
/// Must be 1 to 32 bytes.
/// </div>
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// // Generate SPI byte sequence for R_RX_PAYLOAD with 17 byte payload.
/// let bytes = commands::RRxPayload::<17>::bytes();
/// let mut expected_bytes = [0; 18];
/// expected_bytes[0] = 0b0110_0001;
/// assert_eq!(bytes, expected_bytes);
/// ```
pub struct RRxPayload<const N: usize>();

/// # W_TX_PAYLOAD command
/// Write TX payload.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let payload = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let bytes = commands::WTxPayload(payload).bytes();
/// assert_eq!(bytes, [0b1010_0000, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
/// ```
pub struct WTxPayload<const N: usize>(
    /// Payload to write.
    /// <div class="warning">
    /// Payload must be 1 to 32 bytes.
    /// </div>
    pub [u8; N],
);

/// # FLUSH_TX command
/// Flush TX FIFO. Used in TX mode.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands::{self, Command};
///
/// assert_eq!(commands::FlushTx::WORD, 0b1110_0001);
/// assert_eq!(commands::FlushTx::bytes(), [0b1110_0001]);
/// ```
pub struct FlushTx();

/// # FLUSH_RX command
/// Flush RX FIFO. Used in RX mode.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands::{self, Command};
///
/// assert_eq!(commands::FlushRx::WORD, 0b1110_0010);
/// assert_eq!(commands::FlushRx::bytes(), [0b1110_0010]);
/// ```
pub struct FlushRx();

/// # REUSE_TX_PL command
/// Reuse last transmitted payload. Packets are repeatedly transmitted as long
/// as CE is high. TX payload reuse is active until [`W_TX_PAYLOAD`][WTxPayload] or [`FLUSH_TX`][FlushTx]
/// is executed.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands::{self, Command};
///
/// assert_eq!(commands::ReuseTxPl::WORD, 0b1110_0011);
/// assert_eq!(commands::ReuseTxPl::bytes(), [0b1110_0011]);
/// ```
pub struct ReuseTxPl();

/// # ACTIVATE command
/// Activates the [`FEATURE`][registers::Feature] register.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let bytes = commands::Activate::bytes();
/// assert_eq!(bytes, [0b0101_0000, 0x73]);
/// ```
pub struct Activate();

/// # R_RX_PL_WID command
/// Read RX payload width for the top payload in RX FIFO.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let bytes = commands::RRxPlWid::bytes();
/// assert_eq!(bytes, [0b0110_0000, 0]);
/// ```
pub struct RRxPlWid();

/// # W_ACK_PAYLOAD command
/// Write payload to be transmitted with ACK packet on a data [`pipe`][WAckPayload::pipe]. Used in RX mode.
/// Maximum three ACK packet payloads can be pending. Payloads with the same [`pipe`][WAckPayload::pipe]
/// are handled first-in-first-out.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let pipe = 4;
/// let payload = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let bytes = commands::WAckPayload { pipe, payload }.bytes();
/// assert_eq!(bytes, [0b1010_1000 | pipe, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
/// ```
pub struct WAckPayload<const N: usize> {
    /// Data pipe this ACK payload is designated to.
    pub pipe: u8,
    /// Payload to send with ACK.
    /// <div class="warning">
    /// Payload must be 1 to 32 bytes.
    /// </div>
    pub payload: [u8; N],
}

/// # W_TX_PAYLOAD_NOACK command
/// Write TX payload with AUTOACK disabled.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let payload = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let bytes = commands::WTxPayloadNoack(payload).bytes();
/// assert_eq!(bytes, [0b1011_0000, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
/// ```
pub struct WTxPayloadNoack<const N: usize>(
    /// Payload to write.
    /// <div class="warning">
    /// Payload must be 1 to 32 bytes.
    /// </div>
    pub [u8; N],
);

/// # NOP command
/// No operation. Used to read the status register.
///
/// ## Example
/// ```rust
/// use nrf24l01_commands::commands::{self, Command};
///
/// assert_eq!(commands::Nop::WORD, 0xFF);
/// assert_eq!(commands::Nop::bytes(), [0xFF]);
/// ```
pub struct Nop();

impl<R> Command for RRegister<R> {
    const WORD: u8 = 0;
}
impl<R> Command for WRegister<R> {
    const WORD: u8 = 0b0010_0000;
}
impl<const N: usize> Command for RRxPayload<N> {
    const WORD: u8 = 0b0110_0001;
}
impl<const N: usize> Command for WTxPayload<N> {
    const WORD: u8 = 0b1010_0000;
}
impl Command for FlushTx {
    const WORD: u8 = 0b1110_0001;
}
impl Command for FlushRx {
    const WORD: u8 = 0b1110_0010;
}
impl Command for ReuseTxPl {
    const WORD: u8 = 0b1110_0011;
}
impl Command for Activate {
    const WORD: u8 = 0b0101_0000;
}
impl Command for RRxPlWid {
    const WORD: u8 = 0b0110_0000;
}
impl<const N: usize> Command for WAckPayload<N> {
    const WORD: u8 = 0b1010_1000;
}
impl<const N: usize> Command for WTxPayloadNoack<N> {
    const WORD: u8 = 0b1011_0000;
}
impl Command for Nop {
    const WORD: u8 = 0b1111_1111;
}

impl RRegister<registers::Config> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Config::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::EnAa> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::EnAa::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::EnRxaddr> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::EnRxaddr::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::SetupAw> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupAw::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::SetupRetr> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupRetr::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RfCh> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RfCh::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RfSetup> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RfSetup::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::Status> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Status::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::ObserveTx> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::ObserveTx::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::Cd> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Cd::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl<const N: usize> RRegister<registers::RxAddrP0<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP0::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; N + 1] {
        let mut bytes = [0; N + 1];
        bytes[0] = Self::word();
        bytes
    }
}

impl<const N: usize> RRegister<registers::RxAddrP1<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP1::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; N + 1] {
        let mut bytes = [0; N + 1];
        bytes[0] = Self::word();
        bytes
    }
}

impl RRegister<registers::RxAddrP2> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP2::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxAddrP3> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP3::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxAddrP4> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP4::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxAddrP5> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP5::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl<const N: usize> RRegister<registers::TxAddr<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::TxAddr::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; N + 1] {
        let mut bytes = [0; N + 1];
        bytes[0] = Self::word();
        bytes
    }
}

impl RRegister<registers::RxPwP0> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP0::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxPwP1> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP1::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxPwP2> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP2::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxPwP3> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP3::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxPwP4> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP4::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::RxPwP5> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP5::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::FifoStatus> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::FifoStatus::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::Dynpd> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Dynpd::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl RRegister<registers::Feature> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Feature::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl WRegister<registers::Config> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Config::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::EnAa> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::EnAa::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::EnRxaddr> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::EnRxaddr::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::SetupAw> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupAw::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::SetupRetr> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupRetr::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RfCh> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RfCh::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RfSetup> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RfSetup::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::Status> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Status::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::ObserveTx> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::ObserveTx::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::Cd> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Cd::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

/// Concatenate the command word and address bytes into an array.
#[inline(always)]
const fn concat_word_addr<const N: usize>(word: u8, addr: [u8; N]) -> [u8; N + 1] {
    let mut bytes: [u8; N + 1] = [0; N + 1];
    bytes[0] = word;
    // Addr is already in little-endian byte-order
    let mut i = 1;
    while i < N + 1 {
        bytes[i] = addr[i - 1];
        i += 1;
    }
    bytes
}

impl<const N: usize> WRegister<registers::RxAddrP0<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP0::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_addr(Self::word(), self.0.into_bytes())
    }
}

impl<const N: usize> WRegister<registers::RxAddrP1<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP1::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_addr(Self::word(), self.0.into_bytes())
    }
}

impl WRegister<registers::RxAddrP2> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP2::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxAddrP3> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP3::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxAddrP4> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP4::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxAddrP5> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP5::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl<const N: usize> WRegister<registers::TxAddr<N>> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::TxAddr::<N>::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_addr(Self::word(), self.0.into_bytes())
    }
}

impl WRegister<registers::RxPwP0> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP0::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxPwP1> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP1::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxPwP2> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP2::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxPwP3> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP3::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxPwP4> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP4::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::RxPwP5> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP5::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::FifoStatus> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::FifoStatus::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::Dynpd> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Dynpd::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl WRegister<registers::Feature> {
    /// Get the command's _command word_.
    pub const fn word() -> u8 {
        Self::WORD | registers::Feature::ADDRESS
    }

    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; 2] {
        [Self::word(), self.0.into_bits()]
    }
}

impl<const N: usize> RRxPayload<N> {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; N + 1] {
        let mut bytes: [u8; N + 1] = [0; N + 1];
        bytes[0] = Self::WORD;
        bytes
    }
}

/// Concatenate the command word and payload bytes into an array.
#[inline(always)]
const fn concat_word_payload<const N: usize>(word: u8, payload: [u8; N]) -> [u8; N + 1] {
    let mut bytes: [u8; N + 1] = [0; N + 1];
    bytes[0] = word;
    // Reverse payload byte-order to little-endian
    let mut bytes_idx = 1;
    let mut payload_idx = N - 1;
    while bytes_idx < N + 1 {
        bytes[bytes_idx] = payload[payload_idx];
        bytes_idx += 1;
        payload_idx = payload_idx.wrapping_sub(1);
    }
    bytes
}

impl<const N: usize> WTxPayload<N> {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD, self.0)
    }
}

impl FlushTx {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

impl FlushRx {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

impl ReuseTxPl {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 1] {
        [Self::WORD]
    }
}

impl Activate {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::WORD, 0x73]
    }
}

impl RRxPlWid {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 2] {
        [Self::WORD, 0]
    }
}

impl<const N: usize> WAckPayload<N> {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD | self.pipe, self.payload)
    }
}

impl<const N: usize> WTxPayloadNoack<N> {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD, self.0)
    }
}

impl Nop {
    /// Generate the command's SPI byte sequence.
    pub const fn bytes() -> [u8; 1] {
        [Self::WORD]
    }
}
