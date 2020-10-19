#![no_std]
use crate::commbus::CommBus;
pub mod commbus;

pub mod register;
use register::*;

use core::convert::TryFrom;
use core::fmt::Debug;

use embedded_hal as hal;

pub use accelerometer::{
    Accelerometer, RawAccelerometer, Error as AccelerometerError, 
    vector::{I16x3, F32x3}
};

#[derive(Debug)]
pub enum Error<E, PinError> {
    /// Communication error
    CommErr(E),

    /// Pin error
    PinError(PinError),

    /// Invalid data rate selection
    InvalidDataRate,

    /// Invalid operating mode selection
    InvalidMode,

    /// Invalid full-scale selection
    InvalidRange,

    /// Attempted to write to a read-only register
    WriteToReadOnly,

    /// Invalid address provided
    WrongAddress,

    /// Methods not implemented
    NotImplemented,
}

pub struct LIS3DSH<CB> {
    pub(crate) commbus: CB,
}

impl<CB, E, PinError> LIS3DSH<CB>
where
    CB: CommBus<CommError = crate::Error<E, PinError>>,
    PinError: Debug,
    E: Debug,
{
    pub fn new_with_interface<DELAY>(
        commbus: CB,
        delay: &mut DELAY,
    ) -> Result<LIS3DSH<CB>, Error<E, PinError>>
    where
        DELAY: hal::blocking::delay::DelayMs<u8>,
    {
        let mut x = LIS3DSH { commbus };
        x.init(delay)?;

        Ok(x)
    }

    fn init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), Error<E, PinError>>
    where
        DELAY: hal::blocking::delay::DelayMs<u8>,
    {
        self.commbus
            .write_register(Register::CTRL_REG3.write(), STRESET)?;
        delay.delay_ms(5_u8);

        self.commbus
            .write_register(Register::CTRL_REG3.write(), 0)?;
        delay.delay_ms(5_u8);

        self.commbus
            .write_register(Register::CTRL_REG5.write(), 0)?;

        self.commbus
            .write_register(Register::CTRL_REG4.write(), BDU)?;

        if self.get_device_id()? != DEVICE_ID {
            return Err(Error::WrongAddress);
        }

        self.set_datarate(DataRate::Hz_100)?;
        self.set_range(Range::G8)?;

        self.enable_axis((true, true, true))?;

        Ok(())
    }

    /// `WHO_AM_I` register.
    pub fn get_device_id(&mut self) -> Result<u8, Error<E, PinError>> {
        self.commbus.read_register(Register::WHOAMI.read())
    }

    pub fn get_status_reg(&mut self, reg: u8) -> Result<u8, Error<E, PinError>> {
        self.commbus.read_register(
            match reg {
                1 => Register::CTRL_REG1,
                2 => Register::CTRL_REG2,
                3 => Register::CTRL_REG3,
                4 => Register::CTRL_REG4,
                5 => Register::CTRL_REG5,
                6 => Register::CTRL_REG6,

                _ => return Err(Error::InvalidRange),
            }
            .read(),
        )
    }
    fn enable_axis(&mut self, (x, y, z): (bool, bool, bool)) -> Result<(), Error<E, PinError>> {
        let mut v = self.commbus.read_register(Register::CTRL_REG4.read())?;

        // Clear EN bits
        v &= !(X_EN | Y_EN | Z_EN);

        v |= if x { X_EN } else { 0 };
        v |= if y { Y_EN } else { 0 };
        v |= if z { Z_EN } else { 0 };

        self.commbus.write_register(Register::CTRL_REG4.write(), v)
    }

    pub fn set_datarate(&mut self, datarate: DataRate) -> Result<(), Error<E, PinError>> {
        let mut v = self.commbus.read_register(Register::CTRL_REG4.read())?;
        v &= !ODR_MASK;
        v |= datarate.bits() << ODR_OFFSET;
        self.commbus.write_register(Register::CTRL_REG4.write(), v)
    }

    fn get_datarate(&mut self) -> Result<DataRate, Error<E, PinError>> {
        let ctrl4 = self.commbus.read_register(Register::CTRL_REG4.read())?;
        let odr = (ctrl4 & ODR_MASK) >> ODR_OFFSET;

        DataRate::try_from(odr).map_err(|_| Error::InvalidDataRate)
    }

    fn set_range(&mut self, range: Range) -> Result<(), Error<E, PinError>> {
        let mut ctrl5 = self.commbus.read_register(Register::CTRL_REG5.read())?;

        ctrl5 &= !FS_MASK;
        ctrl5 |= (range.bits() << FS_OFFSET) & FS_MASK;

        // Keep sPI-mode as 4-wire
        ctrl5 &= !0x1;

        self.commbus
            .write_register(Register::CTRL_REG5.write(), ctrl5)
    }

    pub fn get_range(&mut self) -> Result<Range, Error<E, PinError>> {
        let ctrl5 = self.commbus.read_register(Register::CTRL_REG5.read())?;

        let fs = (ctrl5 & FS_MASK) >> FS_OFFSET;

        Range::try_from(fs).map_err(|_| Error::InvalidRange)
    }

    fn get_status(&mut self) -> Result<DataStatus, Error<E, PinError>> {
        let stat = self.commbus.read_register(Register::STATUS.read())?;

        Ok(DataStatus::from(stat))
    }

    pub fn has_data(&mut self) -> Result<bool, Error<E, PinError>> {
        Ok(self.get_status()?.zyxda())
    }
}

impl<CB, E, PinError> Accelerometer for LIS3DSH<CB>
where
    CB: CommBus<CommError = crate::Error<E, PinError>>,
    PinError: Debug,
    E: Debug,
{
    type Error = Error<E, PinError>;

    fn accel_norm(&mut self) -> Result<F32x3, AccelerometerError<Self::Error>> {
        let scale = match self.get_range()? {
            Range::G2 => 0.06,
            Range::G4 => 0.12,
            Range::G6 => 0.18,
            Range::G8 => 0.24,
            Range::G16 => 0.73,
        } / 1000.0;

        let acc_raw = self.accel_raw()?;
        let x = (acc_raw.x as f32) * scale;
        let y = (acc_raw.y as f32) * scale;
        let z = (acc_raw.z as f32) * scale;

        Ok(F32x3::new(x, y, z))
    }

    /// Get the sample rate of the accelerometer data.
    fn sample_rate(&mut self) -> Result<f32, AccelerometerError<Self::Error>> {
        Ok(self.get_datarate()?.sample_rate())
    }
}

impl<CB, E, PinError> RawAccelerometer<I16x3> for LIS3DSH<CB>
where
    CB: CommBus<CommError = crate::Error<E, PinError>>,
    PinError: Debug,
    E: Debug,
{
    type Error = Error<E, PinError>;

    fn accel_raw(&mut self) -> Result<I16x3, AccelerometerError<Self::Error>> {
        let mut accel_bytes = [0u8; 6];

        let res = self
            .commbus
            .read_bytes(Register::OUT_X_L.read(), &mut accel_bytes);
        if res.is_err() {
            return Err(AccelerometerError::new(
                accelerometer::error::ErrorKind::Bus,
            ));
        }

        let x: i16 = (((accel_bytes[1] as u16) << 8) | (accel_bytes[0] as u16)) as i16;
        let y: i16 = (((accel_bytes[3] as u16) << 8) | (accel_bytes[2] as u16)) as i16;
        let z: i16 = (((accel_bytes[5] as u16) << 8) | (accel_bytes[4] as u16)) as i16;

        /*
        let x:i16 = (((accel_bytes[0] as u16) << 8) | (accel_bytes[1] as u16)) as i16;
        let y:i16 = (((accel_bytes[2] as u16) << 8) | (accel_bytes[3] as u16)) as i16;
        let z:i16 = (((accel_bytes[4] as u16) << 8) | (accel_bytes[5] as u16)) as i16;
        */
        Ok(I16x3::new(x, y, z))
    }
}
