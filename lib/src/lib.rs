pub mod objects {
    pub mod global;
    pub mod duty_cycle;
    pub mod hmb;
    pub mod led;
    pub mod sys_watchdog;
}

mod registers;

pub use ni_fpga;

type Register<T, P> = ni_fpga::Register<T, P, StoredOffset>;

use ni_fpga::{StoredOffset, Error, Session, session_lifetimes::ArcStorage};
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

pub fn open_fpga() -> Result<(FpgaBitfile, Session<ArcStorage>), Error> {
    let session = FpgaBitfile::session_builder()?.build_arc()?;
    let bitfile = FpgaBitfile::take(&session)?;
    Ok((bitfile, session))
}
