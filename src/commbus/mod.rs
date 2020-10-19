pub mod spi;
pub use self::spi::SPIBus;

//pub mod i2c;
//pub use self::spi::SPIBus;

/// A method of communicating with the device
pub trait CommBus {
    /// Interface associated error type
    type CommError;

    fn read_bytes(&mut self, register: u8, bytes: &mut [u8]) -> Result<(), Self::CommError>;

    /// Write a byte to the given register.
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Self::CommError>;

    /// Read a byte from the given register.
    fn read_register(&mut self, register: u8) -> Result<u8, Self::CommError>;
}
