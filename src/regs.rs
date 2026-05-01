use bitfield::bitfield;

pub const REG_ALS_CONFIG: u8 = 0x00;
pub const REG_ALS_WINDOW_HIGHT: u8 = 0x01;
pub const REG_ALS_WINDOWS_LOW: u8 = 0x02;
pub const REG_POWER_SAVING: u8 = 0x03;
pub const REG_ALS: u8 = 0x04;
pub const REG_WHITE: u8 = 0x05;
pub const REG_ALS_INT: u8 = 0x06;
pub const REG_DEVICE_ID: u8 = 0x07;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceIDResult {
    pub option_code: u8,
    pub device_id: u8,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IntStatus {
    pub int_th_low: bool,
    pub int_th_high: bool,
}

impl From<u16> for DeviceIDResult {
    fn from(value: u16) -> Self {
        DeviceIDResult {
            option_code: (value >> 8) as u8,
            device_id: (value & 0xff) as u8,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ALSGain {
    AlsX1,
    AlsX2,
    AlsDiv8,
    AlsDiv4,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ALSIt {
    It25ms,
    It50ms,
    It100ms,
    It200ms,
    It400ms,
    It800ms,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ALSPers {
    Ap1,
    Ap2,
    Ap4,
    Ap8,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum PsmMode {
    PSMMode1,
    PSMMode2,
    PSMMode3,
    PSMMode4,
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct AlsConfig(u16);
    impl Debug;
    u8, reserved3, _: 15, 13;
    pub u8, from into ALSGain, als_gain, set_als_gain: 12, 11;
    u8, reserved2, _: 10, 10;
    pub u8, from into ALSIt, als_it, set_als_it: 9, 6;
    pub u8, from into ALSPers, als_pers, set_als_pers: 5, 4;
    u8, reserved1, _: 3, 2;
    pub u8, als_int_en, set_als_int_en: 1, 1;
    pub u8, als_sd, set_als_sd: 0, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct AlsWHigh(u16);
    impl Debug;
    pub u8, als_wh_msb, set_als_wh_msb: 15, 8;
    pub u8, als_wh_lsb, set_als_wh_lsb: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct AlsWLow(u16);
    impl Debug;
    pub u8, als_wl_msb, set_als_wl_msb: 15, 8;
    pub u8, als_wl_lsb, set_als_wl_lsb: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct PowerSaving(u16);
    impl Debug;
    pub u8, from into PsmMode, psm, set_psm: 2, 1;
    pub u8, psm_en, set_psm_en: 0, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct Als(u16);
    impl Debug;
    pub u8, als_high, _: 15, 8;
    pub u8, als_low, _: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct White(u16);
    impl Debug;
    pub u8, white_high, _: 15, 8;
    pub u8, white_low, _: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct AlsIntStatus(u16);
    impl Debug;
    pub int_th_low, _: 15, 15;
    pub int_th_high, _: 14, 14;
    u16, reserved, _: 13, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct DeviceID(u16);
    impl Debug;
    pub u8, slave_addr_op, _: 15, 8;
    pub u8, device_id, _: 7, 0;
}

impl AlsConfig {
    pub fn calculate_resolution(&self) -> f32 {
        let gain = match self.als_gain() {
            ALSGain::AlsX1 => 1.0,
            ALSGain::AlsX2 => 2.0,
            ALSGain::AlsDiv8 => 0.125,
            ALSGain::AlsDiv4 => 0.25,
        };

        let it_ms = match self.als_it() {
            ALSIt::It25ms => 25.0,
            ALSIt::It50ms => 50.0,
            ALSIt::It100ms => 100.0,
            ALSIt::It200ms => 200.0,
            ALSIt::It400ms => 400.0,
            ALSIt::It800ms => 800.0,
        };

        0.0672 * (1.0 / gain) * (100.0 / it_ms)
    }
}

impl Default for AlsConfig {
    fn default() -> Self {
        let mut config = Self(0);
        config.set_als_sd(1);
        config
    }
}

impl From<u8> for PsmMode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => PsmMode::PSMMode1,
            0b01 => PsmMode::PSMMode2,
            0b10 => PsmMode::PSMMode3,
            0b11 => PsmMode::PSMMode4,
            _ => PsmMode::PSMMode4,
        }
    }
}
impl From<PsmMode> for u8 {
    fn from(val: PsmMode) -> Self {
        match val {
            PsmMode::PSMMode1 => 0b00,
            PsmMode::PSMMode2 => 0b01,
            PsmMode::PSMMode3 => 0b10,
            PsmMode::PSMMode4 => 0b11,
        }
    }
}
impl From<u8> for ALSPers {
    fn from(value: u8) -> Self {
        match value {
            0b00 => ALSPers::Ap1,
            0b01 => ALSPers::Ap2,
            0b10 => ALSPers::Ap4,
            _ => ALSPers::Ap8,
        }
    }
}
impl From<ALSPers> for u8 {
    fn from(val: ALSPers) -> Self {
        match val {
            ALSPers::Ap1 => 0x00,
            ALSPers::Ap2 => 0x01,
            ALSPers::Ap4 => 0x02,
            ALSPers::Ap8 => 0x03,
        }
    }
}
impl From<u8> for ALSIt {
    fn from(value: u8) -> Self {
        match value {
            0b1100 => ALSIt::It25ms,
            0b1000 => ALSIt::It50ms,
            0b0000 => ALSIt::It100ms,
            0b0001 => ALSIt::It200ms,
            0b0010 => ALSIt::It400ms,
            _ => ALSIt::It800ms,
        }
    }
}
impl From<ALSIt> for u8 {
    fn from(val: ALSIt) -> Self {
        match val {
            ALSIt::It25ms => 0x0c,
            ALSIt::It50ms => 0x08,
            ALSIt::It100ms => 0x00,
            ALSIt::It200ms => 0x01,
            ALSIt::It400ms => 0x02,
            ALSIt::It800ms => 0x03,
        }
    }
}
impl From<u8> for ALSGain {
    fn from(value: u8) -> Self {
        match value {
            0b00 => ALSGain::AlsX1,
            0b01 => ALSGain::AlsX2,
            0b10 => ALSGain::AlsDiv8,
            _ => ALSGain::AlsDiv4,
        }
    }
}
impl From<ALSGain> for u8 {
    fn from(val: ALSGain) -> Self {
        match val {
            ALSGain::AlsX1 => 0b00,
            ALSGain::AlsX2 => 0b01,
            ALSGain::AlsDiv8 => 0b10,
            ALSGain::AlsDiv4 => 0b11,
        }
    }
}
