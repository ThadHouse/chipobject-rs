use ni_fpga::{Error, ReadOnly, ReadWrite};

use crate::{registers::FpgaBitfile, take_register, Register, StrobeRegister};

pub struct Accel {
    pub stat: Register<u8, ReadOnly>,
    pub dato: Register<u8, ReadWrite>,
    pub cntr: Register<u8, ReadWrite>,
    pub cnfg: Register<u8, ReadWrite>,
    pub cntl: Register<u8, ReadWrite>,
    pub dati: Register<u8, ReadOnly>,
    pub go: StrobeRegister,
    pub addr: Register<u8, ReadWrite>,
}

impl Accel {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            stat: take_register!(bitfile, Accel_STAT)?,
            dato: take_register!(bitfile, Accel_DATO)?,
            cntr: take_register!(bitfile, Accel_CNTR)?,
            cnfg: take_register!(bitfile, Accel_CNFG)?,
            cntl: take_register!(bitfile, Accel_CNTL)?,
            dati: take_register!(bitfile, Accel_DATI)?,
            go: take_register!(bitfile, Accel_GO)?.into(),
            addr: take_register!(bitfile, Accel_ADDR)?,
        })
    }
}
