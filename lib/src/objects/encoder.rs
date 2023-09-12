use ni_fpga::{Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type Output = types::Encoder0_Output;
pub type Config = types::Encoder0_Config;
pub type TimerOutput = types::Encoder0_TimerOutput;
pub type TimerConfig = types::Encoder0_TimerConfig;

pub struct Encoder {
    pub output: Register<Output, ReadOnly>,
    pub config: Register<Config, ReadWrite>,
    pub timer_output: Register<TimerOutput, ReadOnly>,
    pub reset: StrobeRegister,
    pub timer_config: Register<TimerConfig, ReadWrite>,
}

pub const NUM_ENCODER_INTERFACES: u8 = 8;

impl Encoder {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        Ok(match index {
            0 => Self {
                output: take_register!(bitfile, Encoder0_Output)?,
                config: take_register!(bitfile, Encoder0_Config)?,
                timer_output: take_register!(bitfile, Encoder0_TimerOutput)?,
                reset: take_register!(bitfile, Encoder0_Reset)?.into(),
                timer_config: take_register!(bitfile, Encoder0_TimerConfig)?,
            },
            1 => Self {
                output: unsafe { take_register!(bitfile, Encoder1_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder1_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder1_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder1_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder1_TimerConfig)?.transmute_type()
                },
            },
            2 => Self {
                output: unsafe { take_register!(bitfile, Encoder2_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder2_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder2_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder2_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder2_TimerConfig)?.transmute_type()
                },
            },
            3 => Self {
                output: unsafe { take_register!(bitfile, Encoder3_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder3_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder3_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder3_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder3_TimerConfig)?.transmute_type()
                },
            },
            4 => Self {
                output: unsafe { take_register!(bitfile, Encoder4_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder4_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder4_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder4_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder4_TimerConfig)?.transmute_type()
                },
            },
            5 => Self {
                output: unsafe { take_register!(bitfile, Encoder5_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder5_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder5_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder5_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder5_TimerConfig)?.transmute_type()
                },
            },
            6 => Self {
                output: unsafe { take_register!(bitfile, Encoder6_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder6_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder6_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder6_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder6_TimerConfig)?.transmute_type()
                },
            },
            7 => Self {
                output: unsafe { take_register!(bitfile, Encoder7_Output)?.transmute_type() },
                config: unsafe { take_register!(bitfile, Encoder7_Config)?.transmute_type() },
                timer_output: unsafe {
                    take_register!(bitfile, Encoder7_TimerOutput)?.transmute_type()
                },
                reset: take_register!(bitfile, Encoder7_Reset)?.into(),
                timer_config: unsafe {
                    take_register!(bitfile, Encoder7_TimerConfig)?.transmute_type()
                },
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
