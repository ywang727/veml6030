use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("I2C read failure")]
    I2CReadError,
    #[error("I2C write failure")]
    I2CWriteError,
    #[error("Interrupt pin error")]
    InterruptError,
    #[error("Timeout error")]
    Timeout,
}

pub type Result<T> = core::result::Result<T, Error>;
