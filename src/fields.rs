//! Enums for certain nRF24L01+ register fields.

/// A trait for certain multi-bit register fields that are represented as enums.
#[const_trait]
pub trait EnumField {
    /// Convert the field to its bits representation.
    fn into_bits(self) -> u8;
    /// Convert bits to the field.
    fn from_bits(bits: u8) -> Self;
}

/// CRC encoding scheme.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crco {
    /// 1 byte CRC
    OneByte = 0,
    /// 2 byte CRC
    TwoByte = 1,
}
impl const EnumField for Crco {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-1
        unsafe { core::mem::transmute(bits & 1) }
    }
}

/// RX/TX address field width in bytes.
/// LSByte is used if address width is below 5 bytes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressWidth {
    Illegal = 0,
    ThreeByte = 0b01,
    FourByte = 0b10,
    FiveByte = 0b11,
}
impl const EnumField for AddressWidth {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-3
        unsafe { core::mem::transmute(bits & 0b11) }
    }
}

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoRetransmitDelay {
    US250 = 0b0000,
    US500 = 0b0001,
    US750 = 0b0010,
    US1000 = 0b0011,
    US1250 = 0b0100,
    US1500 = 0b0101,
    US1750 = 0b0110,
    US2000 = 0b0111,
    US2250 = 0b1000,
    US2500 = 0b1001,
    US2750 = 0b1010,
    US3000 = 0b1011,
    US3250 = 0b1100,
    US3500 = 0b1101,
    US3750 = 0b1110,
    US4000 = 0b1111,
}
impl const EnumField for AutoRetransmitDelay {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-15
        unsafe { core::mem::transmute(bits & 0b1111) }
    }
}

/// High speed data rate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RfDrHigh {
    Mbps1 = 0,
    Mbps2 = 1,
}
impl const EnumField for RfDrHigh {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-1
        unsafe { core::mem::transmute(bits & 1) }
    }
}

/// Set RF output power in TX mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RfPower {
    /// -18 dBm
    Neg18Dbm = 0b00,
    /// -12 dBm
    Neg12Dbm = 0b01,
    /// -6 dBm
    Neg6Dbm = 0b10,
    /// 0 dBm
    Dbm0 = 0b11,
}
impl const EnumField for RfPower {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-3
        unsafe { core::mem::transmute(bits & 0b11) }
    }
}

/// Data pipe number for the payload available from reading RX FIFO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxPipeNo {
    Pipe0 = 0,
    Pipe1 = 1,
    Pipe2 = 2,
    Pipe3 = 3,
    Pipe4 = 4,
    Pipe5 = 5,
    NotUsed = 0b110,
    RxFifoEmpty = 0b111,
}
impl const EnumField for RxPipeNo {
    fn into_bits(self) -> u8 {
        self as _
    }
    fn from_bits(bits: u8) -> Self {
        // SAFETY: The result is guaranteed to be in the range of 0-7
        unsafe { core::mem::transmute(bits & 0b111) }
    }
}
