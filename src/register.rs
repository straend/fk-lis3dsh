use num_enum::TryFromPrimitive;

/// Enumerate all device registers.
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    OUT_T = 0x0C,
    INFO1 = 0x0D,
    INFO2 = 0x0E,
    WHOAMI = 0x0F,

    OFF_X = 0x10,
    OFF_Y = 0x11,
    OFF_Z = 0x12,

    CS_X = 0x13,
    CS_Y = 0x14,
    CS_Z = 0x15,

    LC_L = 0x16,
    LC_H = 0x17,

    STAT = 0x18,

    //
    CTRL_REG4 = 0x20,
    CTRL_REG1 = 0x21,
    CTRL_REG2 = 0x22,
    CTRL_REG3 = 0x23,
    CTRL_REG5 = 0x24,
    CTRL_REG6 = 0x26,

    STATUS = 0x27, // 0xA7 read

    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
    pub fn read(self) -> u8 {
        self as u8 | 0x01 << 7
    }
    pub fn write(self) -> u8 {
        self as u8 & 0b0111_1111
    }
}

/// Full-scale selection.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Range {
    /// ±16g
    G16 = 0b100,

    /// ±8g
    G8 = 0b011,

    ///  ±6g
    G6 = 0b010,

    /// ±4g
    G4 = 0b001,

    /// ±2g (Default)
    G2 = 0b000,
}

impl Range {
    pub fn bits(self) -> u8 {
        self as u8
    }
}

/// Output data rate.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DataRate {
    Hz_1600 = 0b1001,
    Hz_800 = 0b1000,
    Hz_400 = 0b0111,
    Hz_100 = 0b0110,
    Hz_50 = 0b0101,
    Hz_25 = 0b0100,
    Hz_12 = 0b0011,
    Hz_6 = 0b0010,
    Hz_3 = 0b0001,

    /// Power down
    PowerDown = 0b0000,
}

impl DataRate {
    pub fn bits(self) -> u8 {
        self as u8
    }

    pub fn sample_rate(self) -> f32 {
        match self {
            DataRate::Hz_1600 => 1600.0,
            DataRate::Hz_800 => 800.0,
            DataRate::Hz_400 => 400.0,
            DataRate::Hz_100 => 100.0,
            DataRate::Hz_50 => 50.0,
            DataRate::Hz_25 => 25.0,
            DataRate::Hz_12 => 12.5,
            DataRate::Hz_6 => 6.25,
            DataRate::Hz_3 => 3.125,
            DataRate::PowerDown => 0.0,
        }
    }
}
pub struct DataStatus(u8);

impl DataStatus {
    const ZYXOR_MASK: u8 = 0x1 << 7;
    const ZOR_MASK: u8 = 0x1 << 6;
    const YOR_MASK: u8 = 0x1 << 5;
    const XOR_MASK: u8 = 0x1 << 4;

    const ZYXDA_MASK: u8 = 0x1 << 3;
    const ZDA_MASK: u8 = 0x1 << 2;
    const YDA_MASK: u8 = 0x1 << 1;
    const XDA_MASK: u8 = 0x1 << 0;

    /// New from STATUS register
    pub fn from(status: u8) -> DataStatus {
        DataStatus(status)
    }
    /// Data overrun on ZYX axis
    pub fn zyxor(&self) -> bool {
        self.0 & Self::ZYXOR_MASK != 0
    }
    /// Data overrun on Z axis
    pub fn zor(&self) -> bool {
        self.0 & Self::ZOR_MASK != 0
    }
    /// Data overrun on Y axis
    pub fn yor(&self) -> bool {
        self.0 & Self::YOR_MASK != 0
    }
    /// Data overrun on X axis
    pub fn xor(&self) -> bool {
        self.0 & Self::XOR_MASK != 0
    }

    /// Data available for all axes
    pub fn zyxda(&self) -> bool {
        self.0 & Self::ZYXDA_MASK != 0
    }
    /// Data available for Z axis
    pub fn zda(&self) -> bool {
        self.0 & Self::ZDA_MASK != 0
    }
    /// Data available for Y axis
    pub fn yda(&self) -> bool {
        self.0 & Self::YDA_MASK != 0
    }
    /// Data available for X axis
    pub fn xda(&self) -> bool {
        self.0 & Self::XDA_MASK != 0
    }
}

pub const DEVICE_ID: u8 = 63;

pub const ODR_MASK: u8 = 0b1111_0000;
pub const ODR_OFFSET: u8 = 4;

pub const BDU: u8 = 0b0000_1000;
pub const Z_EN: u8 = 0b0000_0100;
pub const Y_EN: u8 = 0b0000_0010;
pub const X_EN: u8 = 0b0000_0001;

pub const FS_MASK: u8 = 0b0011_1000;
pub const FS_OFFSET: u8 = 3;

pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;

pub const STRESET: u8 = 0b1;
