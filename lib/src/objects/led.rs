use ni_fpga::{fxp::FXP, Error, HmbDefinition, ReadOnly, ReadWrite};

use crate::{registers::FpgaBitfile, take_register, Register};

pub struct Led {
    pub string_length: Register<u16, ReadWrite>,
    pub pixel_output_index: Register<u16, ReadOnly>,
    pub low_bit_tick_timing: Register<[u8; 2], ReadWrite>,
    pub pixel_write_index: Register<i16, ReadOnly>,
    pub start: Register<bool, ReadWrite>,
    pub high_bit_tick_timing: Register<[u8; 2], ReadWrite>,
    pub active: Register<bool, ReadOnly>,
    pub reset: Register<bool, ReadWrite>,
    pub load: Register<bool, ReadWrite>,
    pub sync_timing: Register<FXP<16, 16, false>, ReadWrite>,
    pub abort: Register<bool, ReadWrite>,
    pub output_select: Register<FXP<4, 4, false>, ReadWrite>,
    pub memory_definition: HmbDefinition,
}

impl Led {
    pub fn take(bitfile: &mut FpgaBitfile) -> Result<Self, Error> {
        Ok(Self {
            string_length: take_register!(bitfile, LED_StringLength)?,
            pixel_output_index: take_register!(bitfile, LED_PixelOutputIndex)?,
            low_bit_tick_timing: take_register!(bitfile, LED_LowBitTickTiming)?,
            pixel_write_index: take_register!(bitfile, LED_PixelWriteIndex)?,
            start: take_register!(bitfile, LED_Start)?,
            high_bit_tick_timing: take_register!(bitfile, LED_HighBitTickTiming)?,
            active: take_register!(bitfile, LED_Active)?,
            reset: take_register!(bitfile, LED_Reset)?,
            load: take_register!(bitfile, LED_Load)?,
            abort: take_register!(bitfile, LED_Abort)?,
            sync_timing: take_register!(bitfile, LED_SyncTiming)?,
            output_select: take_register!(bitfile, LED_OutputSelect)?,
            memory_definition: bitfile
                .hmb_definitions
                .HMB_0_LED
                .take()
                .ok_or(Error::ResourceAlreadyTaken)?,
        })
    }
}
