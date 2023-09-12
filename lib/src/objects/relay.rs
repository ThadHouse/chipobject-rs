use ni_fpga::{Error, ReadWrite};

use crate::{
    registers::{
        types::{self},
        FpgaBitfile,
    },
    take_register, Register,
};

pub type Value = types::Relay_Value;

pub struct Relay {
    pub value: Register<Value, ReadWrite>,
}
impl Relay {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            value: take_register!(bitfile, Relay_Value)?,
        })
    }
}
