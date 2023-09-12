use core::ffi::FromBytesUntilNulError;
use std::{str::Utf8Error, sync::PoisonError};

use chipobject_rs::ni_fpga::Status;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[error("An Ni FPGA Error Occured {0}")]
    NiFpga(chipobject_rs::ni_fpga::Error),
    #[error("A mutex is poisoned {0}. This is unrecoverable")]
    MutexPoisoned(String),
    #[error("A netcomm error occured {0}")]
    Netcomm(i32),
    #[error("Bad String")]
    BadString,
    #[error("The token provided was incorrect")]
    BadToken,
    #[error("The library is not initialized")]
    NotInitialized,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::MutexPoisoned(value.to_string())
    }
}

impl From<chipobject_rs::ni_fpga::Error> for Error {
    fn from(value: chipobject_rs::ni_fpga::Error) -> Self {
        Self::NiFpga(value)
    }
}

impl From<dlopen::Error> for Error {
    fn from(value: dlopen::Error) -> Self {
        match value {
            // Map the explicit opening errors to what the C API
            // returns for the same errors.
            dlopen::Error::OpeningLibraryError(_) => Error::NiFpga(
                chipobject_rs::ni_fpga::Error::FPGA(Status::ResourceNotFound),
            ),
            dlopen::Error::SymbolGettingError(_) => {
                Error::NiFpga(chipobject_rs::ni_fpga::Error::FPGA(Status::VersionMismatch))
            }
            // Map unknowns (Which are impossible to hit)
            // just as a generic error. All 3 other enum states
            // for this are impossible with the FPGA library.
            e => panic!("{}", e),
        }
    }
}

impl From<FromBytesUntilNulError> for Error {
    fn from(_: FromBytesUntilNulError) -> Self {
        Error::BadString
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Error::BadString
    }
}
