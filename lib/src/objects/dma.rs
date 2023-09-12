use ni_fpga::{Error, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Config = types::DMA_Config;
pub type Trigger = types::Trigger;

pub struct Dma {
    pub rate: Register<u32, ReadWrite>,
    pub config: Register<Config, ReadWrite>,
    pub external_triggers_0: Register<[Trigger; 4], ReadWrite>,
    pub external_triggers_1: Register<[Trigger; 4], ReadWrite>,
}

impl Dma {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            rate: take_register!(bitfile, DMA_Rate)?,
            config: take_register!(bitfile, DMA_Config)?,
            external_triggers_0: take_register!(bitfile, DMA_ExternalTriggers0)?,
            external_triggers_1: take_register!(bitfile, DMA_ExternalTriggers1)?,
        })
    }
}
