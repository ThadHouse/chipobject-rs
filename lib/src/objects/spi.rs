use ni_fpga::{fxp::FXP, Error, ReadWrite};

use crate::{
    registers::{types, FpgaBitfile},
    take_register, Register, StrobeRegister,
};

pub type StallConfig = types::SPI_StallConfig;
pub type AutoTriggerConfig = types::SPI_AutoTriggerConfig;
pub type AutoByteCount = types::SPI_AutoByteCount;
pub type ChipSelectActiveHigh = types::SPI_ChipSelectActiveHigh;

pub struct Spi {
    pub stall_config: Register<StallConfig, ReadWrite>,
    pub auto_trigger_config: Register<AutoTriggerConfig, ReadWrite>,
    pub auto_chip_select: Register<u8, ReadWrite>,
    pub auto_byte_count: Register<AutoByteCount, ReadWrite>,
    pub auto_spi1_select: Register<bool, ReadWrite>,
    pub auto_rate: Register<u32, ReadWrite>,
    pub enable_dio: Register<FXP<5, 5, false>, ReadWrite>,
    pub chip_select_active_high: Register<ChipSelectActiveHigh, ReadWrite>,
    pub auto_force_one: StrobeRegister,
    pub auto_tx0: Register<[u8; 4], ReadWrite>,
    pub auto_tx1: Register<[u8; 4], ReadWrite>,
    pub auto_tx2: Register<[u8; 4], ReadWrite>,
    pub auto_tx3: Register<[u8; 4], ReadWrite>,
    pub auto_tx4: Register<[u8; 4], ReadWrite>,
    pub auto_tx5: Register<[u8; 4], ReadWrite>,
}

impl Spi {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            stall_config: take_register!(bitfile, SPI_StallConfig)?,
            auto_trigger_config: take_register!(bitfile, SPI_AutoTriggerConfig)?,
            auto_chip_select: take_register!(bitfile, SPI_AutoChipSelect)?,
            auto_byte_count: take_register!(bitfile, SPI_AutoByteCount)?,
            auto_spi1_select: take_register!(bitfile, SPI_AutoSPI1Select)?,
            auto_rate: take_register!(bitfile, SPI_AutoRate)?,
            enable_dio: take_register!(bitfile, SPI_EnableDIO)?,
            chip_select_active_high: take_register!(bitfile, SPI_ChipSelectActiveHigh)?,
            auto_force_one: take_register!(bitfile, SPI_AutoForceOne)?.into(),
            auto_tx0: take_register!(bitfile, SPI_AutoTx0)?,
            auto_tx1: take_register!(bitfile, SPI_AutoTx1)?,
            auto_tx2: take_register!(bitfile, SPI_AutoTx2)?,
            auto_tx3: take_register!(bitfile, SPI_AutoTx3)?,
            auto_tx4: take_register!(bitfile, SPI_AutoTx4)?,
            auto_tx5: take_register!(bitfile, SPI_AutoTx5)?,
        })
    }
}
