use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime},
};
use nix::{sys::wait::WaitStatus, unistd::Pid};
use std::ffi::CString;

/// Noop builtin, same idea as `true`.
pub struct Return(pub i32);

impl Builtin for Return {
    fn run(self, _: Vec<CString>, _: &mut Runtime) -> Result<WaitStatus> {
        Ok(WaitStatus::Exited(Pid::this(), self.0))
    }
}
