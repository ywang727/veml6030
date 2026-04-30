use crate::error::{Error, Result};
use crate::regs::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};

const REG_ALS_CONFIG: u8 = 0x00;
const REG_ALS_WINDOW_HIGHT: u8 = 0x01;
const REG_ALS_WINDOWS_LOW: u8 = 0x02;
const REG_POWER_SAVING: u8 = 0x03;
const REG_ALS: u8 = 0x04;
const REG_WHITE: u8 = 0x05;
const REG_ALS_INT: u8 = 0x06;
const REG_DEVICE_ID: u8 = 0x07;

#[derive(Debug)]
pub struct DeviceID {
    pub option_code: u8,
    pub device_id: u8,
}

#[derive(Debug)]
pub struct IntStatus {
    pub int_th_low: bool,
    pub int_th_high: bool,
}

impl From<u16> for DeviceID {
    fn from(value: u16) -> Self {
        DeviceID {
            option_code: (value >> 8) as u8,
            device_id: (value & 0xff) as u8,
        }
    }
}

#[derive(Debug)]
pub struct Veml6030<I2C>
where
    I2C: Write,
{
    i2c: I2C,
    addr: u8,
}

impl<I2C> Veml6030<I2C>
where
    I2C: Write + WriteRead,
{
    pub fn new(bus: I2C, addr: u8) -> Self {
        Veml6030 {
            i2c: bus,
            addr: addr,
        }
    }

    // read a byte from a register
    fn read_reg(&mut self, reg: u8) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.burst_read(reg, &mut buf)
            .map_err(|_| Error::I2CReadError)?;
        let res = ((buf[1] as u16) << 8) + buf[0] as u16;
        Ok(res)
    }

    // write a byte into  a register
    fn write_reg(&mut self, reg: u8, val: u16) -> Result<()> {
        let buf = [reg, (val >> 8) as u8, (val & 0xff) as u8];
        self.i2c
            .write(self.addr, &buf)
            .map_err(|_| Error::I2CWriteError)
    }
    // read a bunch of bytes from an address started at start_reg ,
    // as for how many bytes being read, it's determined by the size of the buffer
    fn burst_read<'b>(&mut self, start_reg: u8, buf: &'b mut [u8]) -> Result<&'b [u8]> {
        self.i2c
            .write_read(self.addr, &[start_reg], buf)
            .map_err(|_| Error::I2CWriteError)?;
        Ok(buf)
    }

    pub fn device_id(&mut self) -> Result<DeviceID> {
        let val = self.read_reg(REG_DEVICE_ID)?;
        Ok(val.into())
    }

    pub fn read_als(&mut self) -> Result<u16> {
        let val = self.read_reg(REG_ALS)?;
        Ok(val)
    }
    pub fn read_white(&mut self) -> Result<u16> {
        let val = self.read_reg(REG_WHITE)?;
        Ok(val)
    }

    pub fn read_int_status(&mut self) -> Result<IntStatus> {
        let val = self.read_reg(REG_ALS_INT)?;

        let data = AlsIntStatus(val);
        Ok(IntStatus {
            int_th_low: data.int_th_low() > 0,
            int_th_high: data.int_th_high() > 0,
        })
    }

    pub fn stop(&mut self) -> Result<()> {
        let val = self.read_reg(REG_ALS_CONFIG)?;
        self.write_reg(REG_ALS_CONFIG, val | 0x0001)
    }

    pub fn start(&mut self) -> Result<()> {
        let val = self.read_reg(REG_ALS_CONFIG)?;
        self.write_reg(REG_ALS_CONFIG, val & 0xfffe)
    }
}
