use core::default;

use crate::sensor::{self, Veml6030};
use embedded_hal::blocking::i2c::{Write, WriteRead};
pub struct VemlBuilder<I2C>
where
    I2C: Write + WriteRead,
{
    sensor: Veml6030<I2C>,
    config_data: [u16; 5],
}

impl<I2C> VemlBuilder<I2C>
where
    I2C: Write + WriteRead,
{
    pub fn new(bus: I2C, addr: u8) -> Self {
        let sensor = Veml6030::new(bus, addr);
        VemlBuilder {
            sensor: sensor,
            config_data: [0; 5],
        }
    }

    pub fn builder()
}
