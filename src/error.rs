use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I2C read failure")]
    I2CReadError,
    #[error("I2C write failure")]
    I2CWriteError,
}

pub type Result<T> = core::result::Result<T, Error>;
