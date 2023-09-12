use ni_fpga::{Error, ReadWrite, RegisterWrite, SessionAccess};

use crate::Register;

pub struct StrobeRegister {
    register: Register<bool, ReadWrite>,
}

impl StrobeRegister {
    pub fn strobe(&mut self, session: &impl SessionAccess) -> Result<(), Error> {
        self.register.write(session, true)
    }
}

impl From<Register<bool, ReadWrite>> for StrobeRegister {
    fn from(register: Register<bool, ReadWrite>) -> Self {
        StrobeRegister { register }
    }
}
