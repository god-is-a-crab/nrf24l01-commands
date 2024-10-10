use crate::registers::{self, Register};
use core::marker::PhantomData;

pub trait Command {
    const WORD: u8;
}

pub struct ReadRegister<R>(PhantomData<R>);
pub struct WriteRegister<R>(pub R);
pub struct ReadRxPayload {}
pub struct WriteTxPayload {}
pub struct FlushTx {}
pub struct FlushRx {}
pub struct ReuseTxPayload {}
pub struct Activate {}
pub struct ReadRxPayloadWidth {}
pub struct WriteAckPayload(pub u8);
pub struct WriteTxPayloadNoAck {}
pub struct Nop {}

impl<R> Command for ReadRegister<R> {
    const WORD: u8 = 0;
}
impl<R> Command for WriteRegister<R> {
    const WORD: u8 = 0b0010_0000;
}
impl Command for ReadRxPayload {
    const WORD: u8 = 0b0110_0001;
}
impl Command for WriteTxPayload {
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
impl Command for WriteAckPayload {
    const WORD: u8 = 0b1010_1000;
}
impl Command for WriteTxPayloadNoAck {
    const WORD: u8 = 0b1011_0000;
}
impl Command for Nop {
    const WORD: u8 = 0b1111_1111;
}

impl<R: Register> ReadRegister<R> {
    pub const fn word() -> u8 {
        Self::WORD | R::ADDRESS
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

const fn concat_word_data(word: u8, data: [u8; 5]) -> [u8; 6] {
    let mut bytes: [u8; 6] = [0; 6];
    bytes[0] = word;
    let mut i = 1;
    while i < 6 {
        bytes[i] = data[i - 1];
        i += 1;
    }
    bytes
}

impl WriteRegister<registers::RxAddrP0> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP0::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_data(self.word(), self.0.into_bytes())
    }
}

impl WriteRegister<registers::RxAddrP1> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::RxAddrP1::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_data(self.word(), self.0.into_bytes())
    }
}

impl WriteRegister<registers::TxAddr> {
    pub const fn word(&self) -> u8 {
        Self::WORD | registers::TxAddr::ADDRESS
    }

    pub const fn bytes(&self) -> [u8; 6] {
        concat_word_data(self.word(), self.0.into_bytes())
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

impl WriteAckPayload {
    pub const fn word(&self) -> u8 {
        Self::WORD | self.0
    }
}
