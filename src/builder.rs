use crate::error::Result;
use crate::fmt::*;
use crate::regs::*;
use crate::sensor::Veml6030;
use core::marker::PhantomData;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c as I2cTrait;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as I2cTrait;

pub struct Ready;

pub struct NeedsThresholds;

pub struct VemlBuilder<I2C, S> {
    i2c: I2C,
    addr: u8,
    als_config: AlsConfig,
    power_saving: PowerSaving,
    th_low: u16,
    th_high: u16,
    _state: PhantomData<S>,
}

impl<I2C, S> VemlBuilder<I2C, S>
where
    I2C: I2cTrait,
{
    pub fn gain(mut self, gain: ALSGain) -> Self {
        self.als_config.set_als_gain(gain);
        self
    }

    pub fn integration_time(mut self, it: ALSIt) -> Self {
        self.als_config.set_als_it(it);
        self
    }

    pub fn persistence(mut self, pers: ALSPers) -> Self {
        self.als_config.set_als_pers(pers);
        self
    }

    pub fn power_saving(mut self, mode: PsmMode, enabled: bool) -> Self {
        self.power_saving.set_psm(mode);
        self.power_saving.set_psm_en(enabled as u8);
        self
    }

    fn transition<NewS>(self) -> VemlBuilder<I2C, NewS> {
        VemlBuilder {
            i2c: self.i2c,
            addr: self.addr,
            als_config: self.als_config,
            power_saving: self.power_saving,
            th_low: self.th_low,
            th_high: self.th_high,
            _state: PhantomData,
        }
    }
}

impl<I2C> VemlBuilder<I2C, Ready>
where
    I2C: I2cTrait,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            als_config: AlsConfig(1),
            power_saving: PowerSaving(0),
            th_low: 0,
            th_high: 0xFFFF,
            _state: PhantomData,
        }
    }

    pub fn interrupt_enabled(mut self) -> VemlBuilder<I2C, NeedsThresholds> {
        self.als_config.set_als_int_en(1);
        self.transition()
    }

    pub fn indoor_mode(self) -> Self {
        info!("VEML6030: applying indoor strategy (High Sensitivity)");
        self.gain(ALSGain::AlsX2).integration_time(ALSIt::It800ms)
    }

    pub fn outdoor_mode(self) -> Self {
        info!("VEML6030: applying outdoor strategy (Low Sensitivity)");
        self.gain(ALSGain::AlsDiv8).integration_time(ALSIt::It50ms)
    }

    pub fn low_power_mode(self) -> Self {
        info!("VEML6030: applying low power strategy (PSM Mode 4)");
        self.power_saving(PsmMode::PSMMode4, true)
    }

    #[cfg(feature = "async")]
    pub async fn build(mut self) -> Result<Veml6030<I2C>> {
        debug!("VEML6030: building sensor (async)");

        self.als_config.set_als_sd(1);
        let mut sensor = Veml6030::new_with_config(self.i2c, self.addr, self.als_config);

        sensor.write_reg(REG_ALS_CONFIG, self.als_config.0).await?;

        sensor
            .write_reg(REG_POWER_SAVING, self.power_saving.0)
            .await?;
        sensor.set_thresholds(self.th_low, self.th_high).await?;

        sensor.start().await?;

        Ok(sensor)
    }

    #[cfg(not(feature = "async"))]
    pub fn build(mut self) -> Result<Veml6030<I2C>> {
        self.als_config.set_als_sd(1);
        let mut sensor = Veml6030::new_with_config(self.i2c, self.addr, self.als_config);

        sensor.write_reg(REG_ALS_CONFIG, self.als_config.0)?;
        sensor.write_reg(REG_POWER_SAVING, self.power_saving.0)?;
        sensor.set_thresholds(self.th_low, self.th_high)?;

        sensor.start()?;

        Ok(sensor)
    }
}

impl<I2C> VemlBuilder<I2C, NeedsThresholds>
where
    I2C: I2cTrait,
{
    pub fn thresholds_in_lux(mut self, low_lux: f32, high_lux: f32) -> VemlBuilder<I2C, Ready> {
        let res = self.als_config.calculate_resolution();

        // 将 Lux 转换为寄存器原始数值 (counts)
        self.th_low = (low_lux / res) as u16;
        self.th_high = (high_lux / res) as u16;

        #[cfg(feature = "defmt")]
        info!(
            "VEML6030: thresholds set to Low: {} lx ({}), High: {} lx ({})",
            low_lux, self.th_low, high_lux, self.th_high
        );

        self.transition()
    }
}
