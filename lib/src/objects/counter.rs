use ni_fpga::{Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type Output = types::Counter0_Output;
pub type Config = types::Counter0_Config;
pub type TimerOutput = types::Counter0_TimerOutput;
pub type TimerConfig = types::Counter0_TimerConfig;

pub struct Counter {
    pub output: Register<Output, ReadOnly>,
    pub config: Register<Config, ReadWrite>,
    pub timer_output: Register<TimerOutput, ReadOnly>,
    pub reset: StrobeRegister,
    pub timer_config: Register<TimerConfig, ReadWrite>,
}

pub const NUM_COUNTER_INTERFACES: u8 = 8;

impl Counter {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        Ok(match index {
            0 => Self {
                output: take_register!(bitfile, Counter0_Output)?,
                config: take_register!(bitfile, Counter0_Config)?,
                timer_output: take_register!(bitfile, Counter0_TimerOutput)?,
                reset: take_register!(bitfile, Counter0_Reset)?.into(),
                timer_config: take_register!(bitfile, Counter0_TimerConfig)?,
            },
            1 => Self {
                output: unsafe { take_register!(bitfile, Counter1_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter1_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter1_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter1_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter1_TimerConfig)?.transmute_type()
                },
            },
            2 => Self {
                output: unsafe { take_register!(bitfile, Counter2_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter2_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter2_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter2_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter2_TimerConfig)?.transmute_type()
                },
            },
            3 => Self {
                output: unsafe { take_register!(bitfile, Counter3_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter3_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter3_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter3_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter3_TimerConfig)?.transmute_type()
                },
            },
            4 => Self {
                output: unsafe { take_register!(bitfile, Counter4_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter4_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter4_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter4_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter4_TimerConfig)?.transmute_type()
                },
            },
            5 => Self {
                output: unsafe { take_register!(bitfile, Counter5_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter5_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter5_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter5_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter5_TimerConfig)?.transmute_type()
                },
            },
            6 => Self {
                output: unsafe { take_register!(bitfile, Counter6_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter6_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter6_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter6_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter6_TimerConfig)?.transmute_type()
                },
            },
            7 => Self {
                output: unsafe { take_register!(bitfile, Counter7_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Counter7_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Counter7_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Counter7_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Counter7_TimerConfig)?.transmute_type()
                },
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
