use ni_fpga::{Error, ReadWrite};

use crate::{registers::FpgaBitfile, take_register, Register};

pub struct Alarm {
    pub enable: Register<bool, ReadWrite>,
    pub trigger_time: Register<u32, ReadWrite>,
}

impl Alarm {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            enable: take_register!(bitfile, Alarm_Enable)?,
            trigger_time: take_register!(bitfile, Alarm_TriggerTime)?,
        })
    }
}
