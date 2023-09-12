use ni_fpga::{Error, ReadOnly, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type Status = types::Power_Status;
pub type FaultCounts = types::Power_FaultCounts;
pub type Disable = types::Power_Disable;

pub struct Power {
    pub user_voltage_3v3: Register<u16, ReadOnly>,
    pub status: Register<Status, ReadOnly>,
    pub user_voltage_6v: Register<u16, ReadOnly>,
    pub on_chip_temperature: Register<u16, ReadOnly>,
    pub user_voltage_5v: Register<u16, ReadOnly>,
    pub reset_fault_counts: StrobeRegister,
    pub integrated_io: Register<u16, ReadOnly>,
    pub mxp_dio_voltage: Register<u16, ReadOnly>,
    pub user_current_3v3: Register<u16, ReadOnly>,
    pub vin_voltage: Register<u16, ReadOnly>,
    pub user_current_6v: Register<u16, ReadOnly>,
    pub user_current_5v: Register<u16, ReadOnly>,
    pub brownout_voltage_250mv: Register<u8, ReadWrite>,
    pub ao_voltage: Register<u16, ReadOnly>,
    pub fault_counts: Register<FaultCounts, ReadOnly>,
    pub vin_current: Register<u16, ReadOnly>,
    pub disable: Register<Disable, ReadWrite>,
}

impl Power {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            user_voltage_3v3: take_register!(bitfile, Power_UserVoltage3V3)?,
            status: take_register!(bitfile, Power_Status)?,
            user_voltage_6v: take_register!(bitfile, Power_UserVoltage6V)?,
            on_chip_temperature: take_register!(bitfile, Power_OnChipTemperature)?,
            user_voltage_5v: take_register!(bitfile, Power_UserVoltage5V)?,
            reset_fault_counts: take_register!(bitfile, Power_ResetFaultCounts)?.into(),
            integrated_io: take_register!(bitfile, Power_IntegratedIO)?,
            mxp_dio_voltage: take_register!(bitfile, Power_MXP_DIOVoltage)?,
            user_current_3v3: take_register!(bitfile, Power_UserCurrent3V3)?,
            vin_voltage: take_register!(bitfile, Power_VinVoltage)?,
            user_current_6v: take_register!(bitfile, Power_UserCurrent6V)?,
            user_current_5v: take_register!(bitfile, Power_UserCurrent5V)?,
            brownout_voltage_250mv: take_register!(bitfile, Power_BrownoutVoltage250mV)?,
            ao_voltage: take_register!(bitfile, Power_AOVoltage)?,
            fault_counts: take_register!(bitfile, Power_FaultCounts)?,
            vin_current: take_register!(bitfile, Power_VinCurrent)?,
            disable: take_register!(bitfile, Power_Disable)?,
        })
    }
}
