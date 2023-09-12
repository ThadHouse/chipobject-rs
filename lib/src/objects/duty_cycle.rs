use ni_fpga::{fxp::FXP, Error, ReadOnly, ReadWrite, Status};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register,
};

pub type Source = types::DutyCycle0_Source;

pub struct DutyCycle {
    pub source: Register<Source, ReadWrite>,
    pub output: Register<FXP<31, 31, false, true>, ReadOnly>,
    pub high_ticks: Register<FXP<20, 20, false, true>, ReadOnly>,
    pub frequency: Register<FXP<11, 11, false, true>, ReadOnly>,
}

pub const NUM_DUTY_CYCLE_INTERFACES: u8 = 8;

impl DutyCycle {
    pub fn take(bitfile: &mut FpgaBitfile, index: u8) -> Result<Self, Error> {
        Ok(match index {
            0 => Self {
                source: take_register!(bitfile, DutyCycle0_Source)?,
                output: take_register!(bitfile, DutyCycle0_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle0_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle0_Frequency)?,
            },
            1 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle1_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle1_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle1_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle1_Frequency)?,
            },
            2 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle2_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle2_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle2_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle2_Frequency)?,
            },
            3 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle3_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle3_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle3_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle3_Frequency)?,
            },
            4 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle4_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle4_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle4_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle4_Frequency)?,
            },
            5 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle5_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle5_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle5_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle5_Frequency)?,
            },
            6 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle6_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle6_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle6_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle6_Frequency)?,
            },
            7 => Self {
                source: unsafe { take_register!(bitfile, DutyCycle7_Source)?.transmute_type() },
                output: take_register!(bitfile, DutyCycle7_Output)?,
                high_ticks: take_register!(bitfile, DutyCycle7_HighTicks)?,
                frequency: take_register!(bitfile, DutyCycle7_Frequency)?,
            },
            _ => return Err(Error::FPGA(Status::InvalidParameter)),
        })
    }
}
