use crate::commbus::CommBus;
use crate::Error;

use embedded_hal::blocking::spi;
use embedded_hal::digital::v2::OutputPin;

pub struct SPIBus<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> SPIBus<SPI, CS> {
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

impl<SPI, CS, E, PinError> CommBus for SPIBus<SPI, CS>
where
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
    CS: OutputPin<Error = PinError>,
{
    type CommError = Error<E, PinError>;

    fn read_bytes(&mut self, register: u8, bytes: &mut [u8]) -> Result<(), Error<E, PinError>> {
        self.cs.set_low().map_err(Error::PinError)?;

        let res = self.spi.write(&[register]).map_err(Error::CommErr);
        let res2 = self.spi.transfer(bytes).map_err(Error::CommErr);

        self.cs.set_high().map_err(Error::PinError)?;

        if res.is_err() {
            return Err(res.unwrap_err());
        }
        if res2.is_err() {
            return Err(res2.unwrap_err());
        }

        Ok(())
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E, PinError>> {
        self.cs.set_low().map_err(Error::PinError)?;
        let mut bytes = [register, 0];
        let res = self.spi.transfer(&mut bytes).map_err(Error::CommErr);
        self.cs.set_high().map_err(Error::PinError)?;

        match res {
            Err(e) => Err(e),
            Ok(r) => Ok(r[1]),
        }
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E, PinError>> {
        self.cs.set_low().map_err(Error::PinError)?;
        let mut bytes = [register, value];
        let res = self.spi.write(&mut bytes).map_err(Error::CommErr);
        self.cs.set_high().map_err(Error::PinError)?;

        if res.is_err() {
            return Err(res.unwrap_err());
        }

        Ok(())
    }
}
