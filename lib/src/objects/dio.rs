use ni_fpga::{fxp::FXP, Error, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Outputs = types::DIO_DO;
pub type Inputs = types::DIO_DI;
pub type OutputEnable = types::DIO_OutputEnable;
pub type Pulse = types::DIO_Pulse;

pub struct DIO {
    pub outputs: Register<Outputs, ReadWrite>,
    pub pwm_duty_cycle_a: Register<[u8; 4], ReadWrite>,
    pub pwm_duty_cycle_b: Register<[u8; 2], ReadWrite>,
    pub filter_select_hdr: Register<[FXP<2, 2, false>; 16], ReadWrite>,
    pub filter_select_mxp: Register<[FXP<2, 2, false>; 16], ReadWrite>,
    pub output_enable: Register<OutputEnable, ReadWrite>,
    pub pwm_output_select: Register<[FXP<5, 5, false>; 6], ReadWrite>,
    pub pulse: Register<Pulse, ReadWrite>,
    pub inputs: Register<Inputs, ReadOnly>,
    pub enable_mxp_special_function: Register<u16, ReadWrite>,
    pub pulse_length: Register<u16, ReadWrite>,
    pub pwm_period_power: Register<u16, ReadWrite>,
    pub filter_period_mxp0: Register<FXP<24, 24, false>, ReadWrite>,
    pub filter_period_mxp1: Register<FXP<24, 24, false>, ReadWrite>,
    pub filter_period_mxp2: Register<FXP<24, 24, false>, ReadWrite>,
    pub filter_period_hdr0: Register<FXP<24, 24, false>, ReadWrite>,
    pub filter_period_hdr1: Register<FXP<24, 24, false>, ReadWrite>,
    pub filter_period_hdr2: Register<FXP<24, 24, false>, ReadWrite>,
}

impl DIO {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            outputs: take_register!(bitfile, DIO_DO)?,
            pwm_duty_cycle_a: take_register!(bitfile, DIO_PWMDutyCycleA)?,
            pwm_duty_cycle_b: take_register!(bitfile, DIO_PWMDutyCycleB)?,
            filter_select_hdr: take_register!(bitfile, DIO_FilterSelectHdr)?,
            filter_select_mxp: take_register!(bitfile, DIO_FilterSelectMXP)?,
            output_enable: take_register!(bitfile, DIO_OutputEnable)?,
            pwm_output_select: take_register!(bitfile, DIO_PWMOutputSelect)?,
            pulse: take_register!(bitfile, DIO_Pulse)?,
            inputs: take_register!(bitfile, DIO_DI)?,
            enable_mxp_special_function: take_register!(bitfile, DIO_EnableMXPSpecialFunction)?,
            pulse_length: take_register!(bitfile, DIO_PulseLength)?,
            pwm_period_power: take_register!(bitfile, DIO_PWMPeriodPower)?,
            filter_period_mxp0: take_register!(bitfile, DIO_FilterPeriodMXP0)?,
            filter_period_hdr0: take_register!(bitfile, DIO_FilterPeriodHdr0)?,
            filter_period_mxp1: take_register!(bitfile, DIO_FilterPeriodMXP1)?,
            filter_period_hdr1: take_register!(bitfile, DIO_FilterPeriodHdr1)?,
            filter_period_mxp2: take_register!(bitfile, DIO_FilterPeriodMXP2)?,
            filter_period_hdr2: take_register!(bitfile, DIO_FilterPeriodHdr2)?,
        })
    }
}
