use ni_fpga::{Error, ReadOnly};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Status = types::SysWatchdog_Status;

pub struct SysWatchdog {
    pub status: Register<Status, ReadOnly>,
}

impl SysWatchdog {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            status: take_register!(bitfile, SysWatchdog_Status)?,
        })
    }
}
