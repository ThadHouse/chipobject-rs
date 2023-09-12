use ni_fpga::{fxp::UnsignedPackedNumber, Error, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type Config = types::AI_Config;
pub type ReadSelect = types::AI_ReadSelect;

pub struct AnalogInput {
    pub output: Register<i32, ReadOnly>,
    pub config: Register<Config, ReadWrite>,
    pub loop_timing: Register<u32, ReadOnly>,
    pub oversample_bits: Register<[UnsignedPackedNumber<4>; 8], ReadWrite>,
    pub average_bits: Register<[UnsignedPackedNumber<4>; 8], ReadWrite>,
    pub scan_list: Register<[UnsignedPackedNumber<3>; 8], ReadWrite>,
    pub latch_output: StrobeRegister,
    pub read_select: Register<ReadSelect, ReadWrite>,
}

impl AnalogInput {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            output: take_register!(bitfile, AI_Output)?,
            config: take_register!(bitfile, AI_Config)?,
            loop_timing: take_register!(bitfile, AI_LoopTiming)?,
            oversample_bits: take_register!(bitfile, AI_OversampleBits)?,
            average_bits: take_register!(bitfile, AI_AverageBits)?,
            scan_list: take_register!(bitfile, AI_ScanList)?,
            latch_output: take_register!(bitfile, AI_LatchOutput)?.into(),
            read_select: take_register!(bitfile, AI_ReadSelect)?,
        })
    }
}
