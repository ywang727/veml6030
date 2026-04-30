use bitfield::bitfield;

macro_rules! sb {
    ($start:expr) => {
        ($start * 8)
    };
}

macro_rules! eb {
    ($end:expr) => {
        (sb!($end) + 7)
    };
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
// #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
// pub enum PsmState {
//     PsmEnable,
//     PsmDisable,
// }

bitfield! {
    pub struct AlsConfig(u16);        //reg 0
    impl Debug;

        u8 ,reserved3,_:15,13;
    pub u8 ,from into ALSGain ,als_gain,set_als_gain:12,11;
        u8 ,reserved2,_:10,10;
    pub u8 ,from into ALSIt ,als_it,set_als_it: 9,6;
    pub u8 ,from into ALSPers ,als_pers,set_als_pers: 5,4;
        u8 ,reserved1,_:3,2;
    pub u8 ,als_int_en,set_als_int_en:1,1;
    pub u8 ,als_sd,set_als_sd:0,0;
}

bitfield! {
    pub struct AlsWHigh(u16);        //reg 1
    impl Debug;

    pub u8 ,als_wh_msb,set_als_wh_msb: 15,8;
    pub u8 ,als_wh_lsb,set_als_wh_lsb: 7,0;
}

bitfield! {
    pub struct AlsWLow(u16);     //reg 2
    impl Debug;

    pub u8 ,als_wl_msb,set_als_wl_msb: 15,8;
    pub u8 ,als_wl_lsb,set_als_wl_lsb: 7,0;
}

bitfield! {
    pub struct PowerSaving(u16);        //reg 3
    impl Debug;

    pub u8 ,from into PsmMode, psm,set_psm: 2,1;
    pub u8 ,psm_en,set_psm_en:0,0;
}

bitfield! {
    pub struct Als(u16);        //reg 4
    impl Debug;


    pub u8 ,als_high,_: 15,8;
    pub u8 ,als_low,_:7,0;
}

bitfield! {
    pub struct White(u16);        //reg 5
    impl Debug;


    pub u8 ,white_high,_: 15,8;
    pub u8 ,white_low,_:7,0;
}

bitfield! {
    pub struct AlsIntStatus(u16);        //reg 6
    impl Debug;

    pub int_th_low,_: 15,15;
    pub int_th_high,_:14,14;
    u16 ,reserved,_:  13,0;
}

bitfield! {
    pub struct DeviceID(u16);        //reg 6
    impl Debug;
    pub u8 ,slave_addr_op,_:15,8;
    pub u8 ,device_id,_:7,0;

}

// impl From<u8> for PsmState {
//     fn from(value: u8) -> Self {
//         match value {
//             0b0 => PsmState::PsmDisable,
//             0b1 | _ => PsmState::PsmEnable,
//         }
//     }
// }
// impl Into<u8> for PsmState {
//     fn into(self) -> u8 {
//         match self {
//             PsmState::PsmDisable => 0b00,
//             PsmState::Psmenable => 0b01,
//         }
//     }
// }

impl From<u8> for PsmMode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => PsmMode::PSMMode1,
            0b01 => PsmMode::PSMMode2,
            0b10 => PsmMode::PSMMode3,
            0b11 | _ => PsmMode::PSMMode4,
        }
    }
}
impl Into<u8> for PsmMode {
    fn into(self) -> u8 {
        match self {
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
            0b11 | _ => ALSPers::Ap8,
        }
    }
}

impl Into<u8> for ALSPers {
    fn into(self) -> u8 {
        match self {
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
            0b0011 | _ => ALSIt::It800ms,
        }
    }
}

impl Into<u8> for ALSIt {
    fn into(self) -> u8 {
        match self {
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
            0b11 | _ => ALSGain::AlsDiv4,
        }
    }
}

impl Into<u8> for ALSGain {
    fn into(self) -> u8 {
        match self {
            ALSGain::AlsX1 => 0b00,
            ALSGain::AlsX2 => 0b01,
            ALSGain::AlsDiv8 => 0b10,
            ALSGain::AlsDiv4 => 0b11,
        }
    }
}
