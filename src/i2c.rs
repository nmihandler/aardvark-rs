use crate::AardvarkError;
use embedded_hal::i2c::ErrorType;
use embedded_hal::i2c::{I2c, Operation as I2cOperation, SevenBitAddress, TenBitAddress};
use aardvark_ffi::{aa_i2c_read, aa_i2c_write};
use std::fmt;
use std::ops::Deref;
pub struct I2CDevice {
    handle: Aardvark,
}

impl I2CDevice {
    pub fn new(handle: Aardvark) -> Self {
        Self { handle }
    }
}

impl I2c<TenBitAddress> for I2CDevice {
    fn transaction(
        &mut self,
        address: u16,
        operations: &mut [I2cOperation],
    ) -> Result<(), Self::Error> {
        for (_, operation) in operations.iter_mut().enumerate() {
            match operation {
                I2cOperation::Read(buffer) => {

                    let status = aa_i2c_read(
                        self.handle,
                        address,
                        AardvarkI2cFlags_AA_I2C_10_BIT_ADDR,
                        bytes,
                        buffer.as_mut_ptr(),
                    );

                    if status != 0 {
                        let error = AardvarkError::new(status);
                        return Err(I2CError(error));
                    } 
                    return Ok(());                
                }
                I2cOperation::Write(bytes) => {
                    let status = aa_i2c_write(
                        self.handle,
                        address,
                        AardvarkI2cFlags_AA_I2C_10_BIT_ADDR,
                        bytes,
                        data_out.as_ptr(),
                    );
                
                    if status != 0 {
                        let error = AardvarkError::new(status);
                        return Err(I2CError(error));
                    } 
                    return Ok(());                
                }
            }
        }
        Ok(())
    }
}
impl I2c<SevenBitAddress> for I2CDevice {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [I2cOperation],
    ) -> Result<(), Self::Error> {
        I2c::<TenBitAddress>::transaction(self, u16::from(address), operations)
    }
}
#[derive(Debug)]
pub struct I2CError(AardvarkError);

impl fmt::Display for I2CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for I2CError {}

impl ErrorType for I2CDevice {
    type Error = I2CError;
}

impl embedded_hal::i2c::Error for I2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}
