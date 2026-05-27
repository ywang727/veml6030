use crate::error::{Error, Result};
use crate::fmt::*;
use crate::regs::*;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c as I2cTrait;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as I2cTrait;

#[derive(Debug)]
pub struct Veml6030<I2C> {
    pub(crate) i2c: I2C,
    addr: u8,
    pub(crate) als_config: AlsConfig,
}

impl From<u16> for IntStatus {
    fn from(val: u16) -> Self {
        let status = AlsIntStatus(val);
        Self {
            int_th_low: status.int_th_low(),
            int_th_high: status.int_th_high(),
        }
    }
}

impl<I2C> Veml6030<I2C>
where
    I2C: I2cTrait,
{
    //by default, the sensor is in shutdown state
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            als_config: AlsConfig::default(),
        }
    }

    pub(crate) fn new_with_config(i2c: I2C, addr: u8, als_config: AlsConfig) -> Self {
        Self {
            i2c,
            addr,
            als_config,
        }
    }

    pub fn resolution(&self) -> f32 {
        self.als_config.calculate_resolution()
    }
}

#[cfg(feature = "async")]
impl<I2C> Veml6030<I2C>
where
    I2C: I2cTrait,
{
    pub async fn read_reg(&mut self, reg: u8) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.addr, &[reg], &mut buf)
            .await
            .map_err(|_| Error::I2CReadError)?;
        let val = ((buf[1] as u16) << 8) | buf[0] as u16;
        trace!("VEML6030: read reg 0x{:02x} = 0x{:04x}", reg, val);
        Ok(val)
    }

    pub async fn write_reg(&mut self, reg: u8, val: u16) -> Result<()> {
        trace!("VEML6030: write reg 0x{:02x} = 0x{:04x}", reg, val);
        let buf = [reg, (val & 0xff) as u8, (val >> 8) as u8];
        self.i2c
            .write(self.addr, &buf)
            .await
            .map_err(|_| Error::I2CWriteError)
    }

    pub async fn device_id(&mut self) -> Result<DeviceIDResult> {
        let val = self.read_reg(REG_DEVICE_ID).await?;
        Ok(val.into())
    }

    pub async fn read_als(&mut self) -> Result<u16> {
        self.read_reg(REG_ALS).await
    }

    pub async fn read_lux(&mut self) -> Result<f32> {
        let raw = self.read_als().await?;
        Ok(raw as f32 * self.resolution())
    }

    pub async fn set_thresholds_lux(&mut self, low_lux: f32, high_lux: f32) -> Result<()> {
        let res = self.resolution();
        let low = (low_lux / res) as u16;
        let high = (high_lux / res) as u16;
        self.set_thresholds(low, high).await
    }

    pub async fn read_white(&mut self) -> Result<u16> {
        self.read_reg(REG_WHITE).await
    }

    pub async fn read_int_status(&mut self) -> Result<IntStatus> {
        let val = self.read_reg(REG_ALS_INT).await?;
        let data = AlsIntStatus(val);
        Ok(IntStatus {
            int_th_low: data.int_th_low(),
            int_th_high: data.int_th_high(),
        })
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.als_config.set_als_sd(1);
        self.write_reg(REG_ALS_CONFIG, self.als_config.0).await
    }

    pub async fn start(&mut self) -> Result<()> {
        self.als_config.set_als_sd(0);
        self.write_reg(REG_ALS_CONFIG, self.als_config.0).await
    }

    pub async fn enable_interrupt(&mut self, enable: bool) -> Result<()> {
        let val = self.read_reg(REG_ALS_CONFIG).await?;
        let new_val = if enable { val | 0x0002 } else { val & 0xfffd };
        self.write_reg(REG_ALS_CONFIG, new_val).await
    }

    pub async fn set_thresholds(&mut self, low: u16, high: u16) -> Result<()> {
        self.write_reg(REG_ALS_WINDOWS_LOW, low).await?;
        self.write_reg(REG_ALS_WINDOW_HIGHT, high).await
    }

    pub async fn wait_interrupt<P>(&mut self, pin: &mut P) -> Result<()>
    where
        P: embedded_hal_async::digital::Wait,
    {
        pin.wait_for_low().await.map_err(|_| Error::InterruptError)
    }
}

#[cfg(not(feature = "async"))]
impl<I2C> Veml6030<I2C>
where
    I2C: I2cTrait,
{
    pub fn read_lux(&mut self) -> Result<f32> {
        let raw = self.read_als()?;
        Ok(raw as f32 * self.resolution())
    }

    pub fn set_thresholds_lux(&mut self, low_lux: f32, high_lux: f32) -> Result<()> {
        let res = self.resolution();
        let low = (low_lux / res) as u16;
        let high = (high_lux / res) as u16;
        self.set_thresholds(low, high)
    }

    pub fn read_reg(&mut self, reg: u8) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.addr, &[reg], &mut buf)
            .map_err(|_| Error::I2CReadError)?;
        let val = ((buf[1] as u16) << 8) | buf[0] as u16;
        trace!("VEML6030: read reg 0x{:02x} = 0x{:04x}", reg, val);
        Ok(val)
    }

    pub fn write_reg(&mut self, reg: u8, val: u16) -> Result<()> {
        trace!("VEML6030: write reg 0x{:02x} = 0x{:04x}", reg, val);
        let buf = [reg, (val & 0xff) as u8, (val >> 8) as u8];
        self.i2c
            .write(self.addr, &buf)
            .map_err(|_| Error::I2CWriteError)
    }

    pub fn device_id(&mut self) -> Result<DeviceIDResult> {
        let val = self.read_reg(REG_DEVICE_ID)?;
        Ok(val.into())
    }

    pub fn read_als(&mut self) -> Result<u16> {
        self.read_reg(REG_ALS)
    }

    pub fn read_white(&mut self) -> Result<u16> {
        self.read_reg(REG_WHITE)
    }

    pub fn read_int_status(&mut self) -> Result<IntStatus> {
        let val = self.read_reg(REG_ALS_INT)?;
        let data = AlsIntStatus(val);
        Ok(IntStatus {
            int_th_low: data.int_th_low(),
            int_th_high: data.int_th_high(),
        })
    }

    pub fn stop(&mut self) -> Result<()> {
        self.als_config.set_als_sd(1);
        self.write_reg(REG_ALS_CONFIG, self.als_config.0)
    }

    pub fn start(&mut self) -> Result<()> {
        self.als_config.set_als_sd(0);
        self.write_reg(REG_ALS_CONFIG, self.als_config.0)
    }

    pub fn enable_interrupt(&mut self, enable: bool) -> Result<()> {
        let val = self.read_reg(REG_ALS_CONFIG)?;
        let new_val = if enable { val | 0x0002 } else { val & 0xfffd };
        self.write_reg(REG_ALS_CONFIG, new_val)
    }

    pub fn set_thresholds(&mut self, low: u16, high: u16) -> Result<()> {
        self.write_reg(REG_ALS_WINDOWS_LOW, low)?;
        self.write_reg(REG_ALS_WINDOW_HIGHT, high)
    }
}

#[cfg(test)]
#[cfg(not(feature = "async"))]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};

    const ADDR: u8 = 0x10;

    #[test]
    fn test_read_reg() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x04], vec![0xAB, 0xCD])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Veml6030::new(i2c, ADDR);

        let val = sensor.read_reg(0x04).unwrap();
        assert_eq!(val, 0xCDAB);

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_write_reg() {
        let expectations = [I2cTrans::write(ADDR, vec![0x00, 0x01, 0x02])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Veml6030::new(i2c, ADDR);

        sensor.write_reg(0x00, 0x0201).unwrap();

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_read_lux_calculation() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x04], vec![100, 0])];
        let i2c = I2cMock::new(&expectations);

        let mut config = AlsConfig::default();
        config.set_als_sd(0);
        let mut sensor = Veml6030::new_with_config(i2c, ADDR, config);

        let lux = sensor.read_lux().unwrap();
        // 100 * 0.0672 = 6.72
        assert!((lux - 6.72).abs() < 0.0001);

        let mut i2c = sensor.i2c;
        i2c.done();
    }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod async_tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};
    use futures::executor::block_on;

    const ADDR: u8 = 0x10;

    #[test]
    fn test_read_reg_async() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x04], vec![0xAB, 0xCD])];
        let i2c = I2cMock::new(&expectations);
        let mut sensor = Veml6030::new(i2c, ADDR);

        let val = block_on(sensor.read_reg(0x04)).unwrap();
        assert_eq!(val, 0xCDAB);

        let mut i2c = sensor.i2c;
        i2c.done();
    }

    #[test]
    fn test_read_lux_async() {
        let expectations = [I2cTrans::write_read(ADDR, vec![0x04], vec![100, 0])];
        let i2c = I2cMock::new(&expectations);
        let mut config = AlsConfig::default();
        config.set_als_sd(0);
        let mut sensor = Veml6030::new_with_config(i2c, ADDR, config);

        let lux = block_on(sensor.read_lux()).unwrap();
        assert!((lux - 6.72).abs() < 0.0001);

        let mut i2c = sensor.i2c;
        i2c.done();
    }
}
