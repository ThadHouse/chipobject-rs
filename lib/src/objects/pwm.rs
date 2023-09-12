use ni_fpga::{fxp::FXP, Error, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Config = types::PWM_Config;

pub struct Pwm {
    pub cycle_start_time_lower: Register<u32, ReadOnly>,
    pub cycle_start_time_upper: Register<u32, ReadOnly>,
    pub config: Register<Config, ReadWrite>,
    pub loop_timing: Register<u16, ReadOnly>,
    pub period_scale_mxp: Register<[FXP<2, 2, false>; 10], ReadWrite>,
    pub period_scale_hdr: Register<[FXP<2, 2, false>; 10], ReadWrite>,
    pub zero_latch: Register<[bool; 20], ReadWrite>,
}

impl Pwm {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            cycle_start_time_lower: take_register!(bitfile, PWM_CycleStartTime)?,
            cycle_start_time_upper: take_register!(bitfile, PWM_CycleStartTimeUpper)?,
            config: take_register!(bitfile, PWM_Config)?,
            loop_timing: take_register!(bitfile, PWM_LoopTiming)?,
            period_scale_mxp: take_register!(bitfile, PWM_PeriodScaleMXP)?,
            period_scale_hdr: take_register!(bitfile, PWM_PeriodScaleHdr)?,
            zero_latch: take_register!(bitfile, PWM_ZeroLatch)?,
        })
    }
}
