use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime},
};
use nix::{sys::wait::WaitStatus, unistd::Pid};
use std::{ffi::CString, process};

/// Exit builtin, alternative to ctrl-d.
pub struct Exit;

impl Builtin for Exit {
    #[allow(unused_variables)]
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        #[cfg(feature = "history")]
        if argv.len() == 1 || argv.len() == 2 {
            runtime.history.save().unwrap();
        }

        match argv.len() {
            0 => {
                panic!("command name not passed in argv[0]");
            }
            1 => process::exit(0),
            2 => {
                if let Ok(n) = str::parse(argv[1].to_str().unwrap()) {
                    process::exit(n)
                } else {
                    process::exit(2)
                }
            }
            _ => {
                eprintln!("too many arguments");
                Ok(WaitStatus::Exited(Pid::this(), 1))
            }
        }
    }
}
