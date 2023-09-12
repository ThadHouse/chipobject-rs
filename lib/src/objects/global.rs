use ni_fpga::{Error, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type LEDs = types::LEDs;

pub struct GlobalRead {
    pub local_time_upper: Register<u32, ReadOnly>,
    pub local_time_lower: Register<u32, ReadOnly>,
    pub version: Register<u16, ReadOnly>,
    pub user_button: Register<bool, ReadOnly>,
    pub revision: Register<u32, ReadOnly>,
}

pub struct GlobalReadWrite {
    pub interrupt_force_number: Register<u8, ReadWrite>,
    pub interrupt_force_once: StrobeRegister,
    pub leds: Register<LEDs, ReadWrite>,
}

impl GlobalRead {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            local_time_upper: take_register!(bitfile, LocalTimeUpper)?,
            local_time_lower: take_register!(bitfile, LocalTime)?,
            version: take_register!(bitfile, Version)?,
            user_button: take_register!(bitfile, UserButton)?,
            revision: take_register!(bitfile, Revision)?,
        })
    }
}

impl GlobalReadWrite {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            interrupt_force_number: take_register!(bitfile, InterruptForceNumber)?,
            interrupt_force_once: take_register!(bitfile, InterruptForceOnce)?.into(),
            leds: take_register!(bitfile, LEDs)?,
        })
    }
}
