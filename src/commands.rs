use crate::registers::{self, Register};
use core::marker::PhantomData;

/// Defines an nRF24L01 command with a command word.
pub trait Command {
    const WORD: u8;
}

/// Read a register.
///
/// Type parameter:
/// - `R`: register type
/// ```rust
/// use nrf24l01_commands::{registers, commands};
/// let bytes = commands::ReadRegister::<registers::FifoStatus>::bytes();
/// assert_eq!(bytes, [0x17, 0]);
/// ```
pub struct ReadRegister<R>(PhantomData<R>);

/// Write a register.
///
/// Field 0: register to write
/// ```rust
/// use nrf24l01_commands::{registers, commands};
///
/// let reg = registers::TxAddr::new().with_tx_addr(0x61DE7C320B);
/// let bytes = commands::WriteRegister(reg).bytes();
/// assert_eq!(bytes, [0b0010_0000 | 0x10, 0x0B, 0x32, 0x7C, 0xDE, 0x61]);
/// ```
pub struct WriteRegister<R>(pub R);

/// Read RX-payload.
///
/// Const parameter:
/// - `N`: Width of RX payload
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let bytes = commands::ReadRxPayload::<32>::bytes();
/// let mut expected_bytes = [0; 33];
/// expected_bytes[0] = 0b0110_0001;
/// assert_eq!(bytes, expected_bytes);
/// ```
pub struct ReadRxPayload<const N: usize>();

/// Write TX-payload.
///
/// Field 0: payload to write
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let payload = [b'a'; 32];
/// let bytes = commands::WriteTxPayload(payload).bytes();
/// let mut expected_bytes = [b'a'; 33];
/// expected_bytes[0] = 0b1010_0000;
/// assert_eq!(bytes, expected_bytes);
/// ```
pub struct WriteTxPayload<const N: usize>(pub [u8; N]);

/// Flush TX FIFO. Used in TX mode.
pub struct FlushTx();

/// Flush RX FIFO. Used in RX mode.
pub struct FlushRx();

/// Reuse last transmitted payload. Packets are repeatedly transmitted as long
/// as CE is high. TX payload reuse is active until W_TX_PAYLOAD or FLUSH_TX
/// is executed.
pub struct ReuseTxPayload();

/// Activate command. Activates the following features:
/// - R_RX_PL_WID (ReadRxPayloadWidth)
/// - W_ACK_PAYLOAD (WriteAckPayload)
/// - W_TX_PAYLOAD_NOACK (WriteTxPayloadNoAck)
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let bytes = commands::Activate::bytes();
/// assert_eq!(bytes, [0b0101_0000, 0x73]);
/// ```
pub struct Activate();

/// Read RX-payload width for the top payload in RX FIFO.
pub struct ReadRxPayloadWidth();

/// Write payload to be transmitted with ACK packet on PIPE. Used in RX mode.
/// Maximum three ACK packet payload can be pending. Payloads with the same PIPE
/// are handled FIFO. Write payload 1-32 bytes.
///
/// Fields:
/// - `pipe`: data pipe 000 - 101
/// - `payload`: payload to write
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let payload = [b'a'; 32];
/// let bytes = commands::WriteAckPayload{ pipe: 4, payload: payload }.bytes();
/// let mut expected_bytes = [b'a'; 33];
/// expected_bytes[0] = 0b1010_1100;
/// assert_eq!(bytes, expected_bytes);
/// ```
pub struct WriteAckPayload<const N: usize> {
    pub pipe: u8,
    pub payload: [u8; N],
}

/// Write TX-payload with AUTOACK disabled.
///
/// Field 0: payload to write
/// ```rust
/// use nrf24l01_commands::commands;
///
/// let payload = [b'a'; 32];
/// let bytes = commands::WriteTxPayloadNoAck(payload).bytes();
/// let mut expected_bytes = [b'a'; 33];
/// expected_bytes[0] = 0b1011_0000;
/// assert_eq!(bytes, expected_bytes);
/// ```
pub struct WriteTxPayloadNoAck<const N: usize>(pub [u8; N]);

/// Nop command. Used to read the status register.
/// ```rust
/// use nrf24l01_commands::commands::{self, Command};
///
/// let command_word = commands::Nop::WORD;
/// assert_eq!(command_word, 0xFF);
/// ```
pub struct Nop();

impl<R> Command for ReadRegister<R> {
    const WORD: u8 = 0;
}
impl<R> Command for WriteRegister<R> {
    const WORD: u8 = 0b0010_0000;
}
impl<const N: usize> Command for ReadRxPayload<N> {
    const WORD: u8 = 0b0110_0001;
}
impl<const N: usize> Command for WriteTxPayload<N> {
    const WORD: u8 = 0b1010_0000;
}
impl Command for FlushTx {
    const WORD: u8 = 0b1110_0001;
}
impl Command for FlushRx {
    const WORD: u8 = 0b1110_0010;
}
impl Command for ReuseTxPayload {
    const WORD: u8 = 0b1110_0011;
}
impl Command for Activate {
    const WORD: u8 = 0b0101_0000;
}
impl Command for ReadRxPayloadWidth {
    const WORD: u8 = 0b0110_0000;
}
impl<const N: usize> Command for WriteAckPayload<N> {
    const WORD: u8 = 0b1010_1000;
}
impl<const N: usize> Command for WriteTxPayloadNoAck<N> {
    const WORD: u8 = 0b1011_0000;
}
impl Command for Nop {
    const WORD: u8 = 0b1111_1111;
}

impl ReadRegister<registers::Config> {
    pub const fn word() -> u8 {
        Self::WORD | registers::Config::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::EnAa> {
    pub const fn word() -> u8 {
        Self::WORD | registers::EnAa::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::EnRxaddr> {
    pub const fn word() -> u8 {
        Self::WORD | registers::EnRxaddr::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::SetupAw> {
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupAw::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::SetupRetr> {
    pub const fn word() -> u8 {
        Self::WORD | registers::SetupRetr::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RfCh> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RfCh::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RfSetup> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RfSetup::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::Status> {
    pub const fn word() -> u8 {
        Self::WORD | registers::Status::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::ObserveTx> {
    pub const fn word() -> u8 {
        Self::WORD | registers::ObserveTx::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::Cd> {
    pub const fn word() -> u8 {
        Self::WORD | registers::Cd::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxAddrP0> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP0::ADDRESS
    }

    pub const fn bytes() -> [u8; 6] {
        [Self::word(), 0, 0, 0, 0, 0]
    }
}

impl ReadRegister<registers::RxAddrP1> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP1::ADDRESS
    }

    pub const fn bytes() -> [u8; 6] {
        [Self::word(), 0, 0, 0, 0, 0]
    }
}

impl ReadRegister<registers::RxAddrP2> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP2::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxAddrP3> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP3::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxAddrP4> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP4::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxAddrP5> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxAddrP5::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::TxAddr> {
    pub const fn word() -> u8 {
        Self::WORD | registers::TxAddr::ADDRESS
    }

    pub const fn bytes() -> [u8; 6] {
        [Self::word(), 0, 0, 0, 0, 0]
    }
}

impl ReadRegister<registers::RxPwP0> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP0::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxPwP1> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP1::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxPwP2> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP2::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxPwP3> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP3::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxPwP4> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP4::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::RxPwP5> {
    pub const fn word() -> u8 {
        Self::WORD | registers::RxPwP5::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::FifoStatus> {
    pub const fn word() -> u8 {
        Self::WORD | registers::FifoStatus::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::Dynpd> {
    pub const fn word() -> u8 {
        Self::WORD | registers::Dynpd::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl ReadRegister<registers::Feature> {
    pub const fn word() -> u8 {
        Self::WORD | registers::Feature::ADDRESS
    }

    pub const fn bytes() -> [u8; 2] {
        [Self::word(), 0]
    }
}

impl WriteRegister<registers::Config> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::Config::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::EnAa> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::EnAa::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::EnRxaddr> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::EnRxaddr::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::SetupAw> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::SetupAw::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::SetupRetr> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::SetupRetr::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RfCh> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RfCh::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RfSetup> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RfSetup::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::Status> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::Status::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::ObserveTx> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::ObserveTx::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::Cd> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::Cd::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

#[inline(always)]
const fn concat_word_addr(word: u8, addr: [u8; 5]) -> [u8; 6] {
    let mut bytes: [u8; 6] = [0; 6];
    bytes[0] = word;
    // Addr is already in little-endian byte-order
    let mut i = 1;
    while i < 6 {
        bytes[i] = addr[i - 1];
        i += 1;
    }
    bytes
}

impl WriteRegister<registers::RxAddrP0> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP0::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_addr(self.word(), self.0.into_bytes())
    }
}

impl WriteRegister<registers::RxAddrP1> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP1::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_addr(self.word(), self.0.into_bytes())
    }
}

impl WriteRegister<registers::RxAddrP2> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP2::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxAddrP3> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP3::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxAddrP4> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP4::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxAddrP5> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP5::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::TxAddr> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::TxAddr::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_addr(self.word(), self.0.into_bytes())
    }
}

impl WriteRegister<registers::RxPwP0> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP0::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxPwP1> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP1::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxPwP2> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP2::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxPwP3> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP3::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxPwP4> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP4::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::RxPwP5> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxPwP5::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::FifoStatus> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::FifoStatus::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::Dynpd> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::Dynpd::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl WriteRegister<registers::Feature> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::Feature::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 2] {
        [self.word(), self.0.into_bits()]
    }
}

impl<const N: usize> ReadRxPayload<N> {
    pub const fn bytes() -> [u8; N + 1] {
        let mut bytes: [u8; N + 1] = [0; N + 1];
        bytes[0] = Self::WORD;
        bytes
    }
}

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

impl<const N: usize> WriteTxPayload<N> {
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD, self.0)
    }
}

impl Activate {
    pub const fn bytes() -> [u8; 2] {
        [Self::WORD, 0x73]
    }
}

impl ReadRxPayloadWidth {
    pub const fn bytes() -> [u8; 2] {
        [Self::WORD, 0]
    }
}

impl<const N: usize> WriteAckPayload<N> {
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD | self.pipe, self.payload)
    }
}

impl<const N: usize> WriteTxPayloadNoAck<N> {
    pub const fn bytes(&self) -> [u8; N + 1] {
        concat_word_payload(Self::WORD, self.0)
    }
}
