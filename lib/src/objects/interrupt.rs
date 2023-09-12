use ni_fpga::{Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Config = types::Interrupt0_Config;

pub struct Interrupt {
    pub falling_timestamp: Register<u32, ReadOnly>,
    pub config: Register<Config, ReadWrite>,
    pub rising_timestamp: Register<u32, ReadOnly>,
}

pub const NUM_INTERRUPT_INTERFACES: u8 = 8;

impl Interrupt {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        Ok(match index {
            0 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt0_FallingTimeStamp)?,
                config: take_register!(bitfile, Interrupt0_Config)?,
                rising_timestamp: take_register!(bitfile, Interrupt0_RisingTimeStamp)?,
            },
            1 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt1_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt1_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt1_RisingTimeStamp)?,
            },
            2 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt2_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt2_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt2_RisingTimeStamp)?,
            },
            3 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt3_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt3_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt3_RisingTimeStamp)?,
            },
            4 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt4_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt4_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt4_RisingTimeStamp)?,
            },
            5 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt5_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt5_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt5_RisingTimeStamp)?,
            },
            6 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt6_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt6_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt6_RisingTimeStamp)?,
            },
            7 => Self {
                falling_timestamp: take_register!(bitfile, Interrupt7_FallingTimeStamp)?,
                config: unsafe { take_register!(bitfile, Interrupt7_Config)?.transmute_type() },
                rising_timestamp: take_register!(bitfile, Interrupt7_RisingTimeStamp)?,
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
