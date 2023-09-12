use std::ffi::CStr;
use std::ptr::null;
use std::sync::{OnceLock, RwLock};

use chipobject_rs::ni_fpga::RegisterRead;
use chipobject_rs::ni_fpga::{session_lifetimes::ArcStorage, Session};
use chipobject_rs::objects::global::GlobalRead;
use chipobject_rs::open_fpga;
use chipobject_rs::registers::FpgaBitfile;
use errors::Error;
use netcomm::{NetcommApi, NetcommContainer};

mod errors;
mod hmb;
mod netcomm;

pub(crate) trait StatusHelper<T> {
    fn to_result(self) -> Result<T, Error>;
}

impl StatusHelper<()> for i32 {
    #[inline]
    fn to_result(self) -> Result<(), Error> {
        match self {
            0 => Ok(()),
            _ => Err(Error::Netcomm(self)),
        }
    }
}

impl<'a, T> StatusHelper<&'a T> for &'a Option<T> {
    #[inline]
    fn to_result(self) -> Result<&'a T, Error> {
        match self.as_ref() {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }
}

impl<'a, T> StatusHelper<&'a mut T> for &'a mut Option<T> {
    #[inline]
    fn to_result(self) -> Result<&'a mut T, Error> {
        match self.as_mut() {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }
}

struct GlobalState {
    _netcomm: NetcommContainer,
    _session: Session<ArcStorage>,
    _bitfile: FpgaBitfile,
}

struct FastGlobalRead {
    session: Session<ArcStorage>,
    global_read: GlobalRead,
}

static GLOBAL_STATE: OnceLock<Result<RwLock<GlobalState>, Error>> = OnceLock::new();

static GLOBAL_READERS: OnceLock<FastGlobalRead> = OnceLock::new();

pub fn initialize() -> Result<(), Error> {
    GLOBAL_STATE.get_or_init(|| {
        let netcomm = NetcommApi::load()?;

        unsafe {
            netcomm
                .FRC_NetworkCommunication_Reserve(null())
                .to_result()?
        };

        let mut buffer = [u8::default(); 256];

        unsafe { netcomm.FRC_NetworkCommunication_getFPGAFileName(buffer.as_mut_ptr()) };

        let buf_as_c = CStr::from_bytes_until_nul(&buffer)?;

        let (mut bitfile, session) = open_fpga(buf_as_c.to_str()?)?;

        let global_read = GlobalRead::take(&mut bitfile)?;

        GLOBAL_READERS.get_or_init(|| FastGlobalRead {
            session: session.clone(),
            global_read,
        });

        Ok(RwLock::new(GlobalState {
            _netcomm: netcomm,
            _bitfile: bitfile,
            _session: session,
        }))
    });

    Ok(())
}

pub fn fpga_time() -> Result<u64, Error> {
    let read = GLOBAL_READERS.get();
    let state = read.to_result()?;
    // TODO Make this actually read both parts.
    // Also make this HMB
    Ok(state.global_read.local_time_lower.read(&state.session)? as u64)
}
