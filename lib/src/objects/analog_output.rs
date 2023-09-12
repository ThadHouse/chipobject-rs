use ni_fpga::{Error, ReadWrite};

use crate::{registers::FpgaBitfile, take_register, Register};

pub struct AnalogOutput {
    pub mxp0: Register<u16, ReadWrite>,
    pub mxp1: Register<u16, ReadWrite>,
}

impl AnalogOutput {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            mxp0: take_register!(bitfile, AO_MXP0)?,
            mxp1: take_register!(bitfile, AO_MXP1)?,
        })
    }
}
