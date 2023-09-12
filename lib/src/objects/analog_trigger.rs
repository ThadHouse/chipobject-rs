use ni_fpga::{Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Output = types::AnalogTrigger_Output;
pub type SourceSelect = types::AnalogTrigger0_SourceSelect;

pub struct AnalogTrigger {
    pub source_select: Register<SourceSelect, ReadWrite>,
    pub upper_limit: Register<i32, ReadWrite>,
    pub lower_limit: Register<i32, ReadWrite>,
    pub output: Register<[Output; 8], ReadOnly>,
}

pub const NUM_ANALOG_TRIGGER_INTERFACES: u8 = 8;

impl AnalogTrigger {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        let output = take_register!(bitfile, AnalogTrigger_Output)?;

        Ok(match index {
            0 => Self {
                source_select: take_register!(bitfile, AnalogTrigger0_SourceSelect)?,
                upper_limit: take_register!(bitfile, AnalogTrigger0_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger0_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            1 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger1_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger1_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger1_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            2 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger2_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger2_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger2_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            3 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger3_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger3_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger3_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            4 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger4_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger4_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger4_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            5 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger5_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger5_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger5_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            6 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger6_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger6_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger6_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            7 => Self {
                source_select: unsafe {
                    take_register!(bitfile, AnalogTrigger7_SourceSelect)?.transmute_type()
                },
                upper_limit: take_register!(bitfile, AnalogTrigger7_UpperLimit)?,
                lower_limit: take_register!(bitfile, AnalogTrigger7_LowerLimit)?,
                output: unsafe { output.duplicate() },
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
