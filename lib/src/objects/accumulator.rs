use ni_fpga::{Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types::Accumulator0_Output, FpgaBitfile},
    take_register, Register,
};

pub type Output = Accumulator0_Output;

pub struct Accumulator {
    pub output: Register<Output, ReadOnly>,
    pub center: Register<i32, ReadWrite>,
    pub deadband: Register<i32, ReadWrite>,
}

pub const NUM_ACCUMULATOR_INTERFACES: u8 = 2;

impl Accumulator {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        Ok(match index {
            0 => Self {
                output: take_register!(bitfile, Accumulator0_Output)?,
                center: take_register!(bitfile, Accumulator0_Center)?,
                deadband: take_register!(bitfile, Accumulator0_Deadband)?,
            },
            1 => Self {
                output: unsafe { take_register!(bitfile, Accumulator1_Output)?.transmute_type() },
                center: take_register!(bitfile, Accumulator1_Center)?,
                deadband: take_register!(bitfile, Accumulator1_Deadband)?,
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
