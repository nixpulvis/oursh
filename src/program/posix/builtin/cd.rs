use crate::{
    program::posix::builtin::Builtin,
    program::{Error, Result, Runtime},
};
use nix::{
    sys::wait::WaitStatus,
    unistd::{chdir, Pid},
};
use std::{
    env::{self, set_var},
    ffi::CString,
};

/// Change directory (`cd`) builtin.
pub struct Cd;

impl Builtin for Cd {
    fn run(self, argv: Vec<CString>, _: &mut Runtime) -> Result<WaitStatus> {
        match argv.len() {
            0 => {
                panic!("command name not passed in argv[0]");
            }
            1 => {
                let home = match env::var("HOME") {
                    Ok(path) => path,
                    Err(_) => return Err(Error::Runtime),
                };
                let dst = home.as_str();
                chdir(dst)
                    .map(|_| {
                        set_var("PWD", &dst);
                        WaitStatus::Exited(Pid::this(), 0)
                    })
                    .map_err(|_| Error::Runtime)
            }
            2 => {
                let dst = argv[1].to_string_lossy();
                chdir(dst.as_ref())
                    .map(|_| {
                        set_var("PWD", dst.as_ref());
                        WaitStatus::Exited(Pid::this(), 0)
                    })
                    .map_err(|_| Error::Runtime)
            }
            _ => {
                eprintln!("too many arguments");
                Ok(WaitStatus::Exited(Pid::this(), 1))
            }
        }
    }
}
