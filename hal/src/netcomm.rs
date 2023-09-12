#![allow(non_snake_case)]

use std::ffi::c_void;

use dlopen::{
    utils::platform_file_name,
    wrapper::{Container, WrapperApi},
    Error,
};

use dlopen_derive::WrapperApi;

pub type NetcommContainer = Container<NetcommApi>;

#[derive(WrapperApi)]
pub struct NetcommApi {
    FRC_NetworkCommunication_Reserve: unsafe extern "C" fn(instance: *const c_void) -> i32,
    FRC_NetworkCommunication_getWatchdogActive: unsafe extern "C" fn() -> i32,
    FRC_NetworkCommunication_getFPGAFileName: unsafe extern "C" fn(file_name: *mut u8),
}

impl NetcommApi {
    pub fn load() -> Result<NetcommContainer, Error> {
        unsafe { Container::load(platform_file_name("FRC_NetworkCommunication")) }
    }
}
