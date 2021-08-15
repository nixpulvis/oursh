use std::{
    env,
    ffi::CString,
};
use nix::{
    unistd::Pid,
    sys::wait::WaitStatus,
};
use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime},
};

/// Export builtin, used to set global variables.
pub struct Export;

impl Builtin for Export {
    fn run(self, argv: Vec<CString>, _: &mut Runtime) -> Result<WaitStatus> {
        match argv.len() {
            0 => unreachable!(),
            1 => {
                // TODO: Print all env vars.
                unimplemented!();
            }
            n => {
                for assignment in argv[1..n].iter() {
                    let mut split = assignment.to_str().unwrap().splitn(2, '=');
                    if let (Some(key), Some(value)) = (split.next(), split.next()) {
                        env::set_var(key, value);
                    }
                }
                Ok(WaitStatus::Exited(Pid::this(), 0))
            },
        }
    }
}
