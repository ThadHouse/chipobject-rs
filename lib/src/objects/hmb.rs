use ni_fpga::{fxp::FXP, Error, HmbDefinition, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Config = types::HMB_Config;

pub struct Hmb {
    pub config: Register<Config, ReadWrite>,
    pub loop_count: Register<i32, ReadOnly>,
    pub write_data: Register<u32, ReadOnly>,
    pub read_data: Register<u32, ReadOnly>,
    pub write_address: Register<FXP<9, 9, false, false>, ReadOnly>,
    pub force_once: Register<bool, ReadWrite>,
    pub write_count: Register<u32, ReadOnly>,
    pub req_ready_for_input: Register<bool, ReadOnly>,
    pub write_ready_for_input: Register<bool, ReadOnly>,

    pub memory_definition: HmbDefinition,
}

impl Hmb {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            config: take_register!(bitfile, HMB_Config)?,
            loop_count: take_register!(bitfile, HMB_LoopCount)?,
            write_data: take_register!(bitfile, HMB_WriteData)?,
            read_data: take_register!(bitfile, HMB_ReadData)?,
            write_address: take_register!(bitfile, HMB_WriteAddress)?,
            force_once: take_register!(bitfile, HMB_ForceOnce)?,
            write_count: take_register!(bitfile, HMB_WriteCount)?,
            req_ready_for_input: take_register!(bitfile, HMB_ReqReadyForInput)?,
            write_ready_for_input: take_register!(bitfile, HMB_WriteReadyForInput)?,
            memory_definition: bitfile
                .hmb_definitions
                .HMB_0_RAM
                .take()
                .ok_or(Error::ResourceAlreadyTaken)?,
        })
    }
}
