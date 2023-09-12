pub mod objects {
    pub mod accel;
    pub mod accumulator;
    pub mod alarm;
    pub mod analog_input;
    pub mod analog_output;
    pub mod analog_trigger;
    pub mod counter;
    pub mod dio;
    pub mod dma;
    pub mod duty_cycle;
    pub mod encoder;
    pub mod global;
    pub mod hmb;
    pub mod interrupt;
    pub mod led;
    pub mod power;
    pub mod pwm;
    pub mod relay;
    pub mod spi;
    pub mod sys_watchdog;
}

pub mod registers;
mod strobe;

pub use strobe::StrobeRegister;

pub use ni_fpga;

type Register<T, P> = ni_fpga::Register<T, P, StoredOffset>;

use ni_fpga::{session_lifetimes::ArcStorage, Error, Session, StoredOffset};
use registers::FpgaBitfile;

#[macro_export]
macro_rules! take_register {
    ( $bitfile:ident, $func:ident ) => {
        $crate::try_unwrap_register($bitfile.$func.take(), stringify!($func))
    };
}

pub(crate) fn try_unwrap_register<T>(reg: Option<T>, _name: impl AsRef<str>) -> Result<T, Error> {
    reg.ok_or(Error::ResourceAlreadyTaken)
}

#[cfg(not(target_arch = "arm"))]
pub fn open_fpga(
    _bitfile_location: impl AsRef<str>,
) -> Result<(FpgaBitfile, Session<ArcStorage>), Error> {
    let session = FpgaBitfile::session_builder()?.build_arc()?;
    let bitfile = FpgaBitfile::take(&session)?;
    Ok((bitfile, session))
}

#[cfg(target_arch = "arm")]
pub fn open_fpga(
    bitfile_location: impl AsRef<str>,
) -> Result<(FpgaBitfile, Session<ArcStorage>), Error> {
    let session = FpgaBitfile::session_builder()?
        .bitfile_path(bitfile_location)?
        .build_arc()?;
    let bitfile = FpgaBitfile::take(&session)?;
    Ok((bitfile, session))
}
