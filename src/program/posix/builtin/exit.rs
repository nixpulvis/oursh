use std::{
    process,
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

/// Exit builtin, alternative to ctrl-d.
pub struct Exit;

impl Builtin for Exit {
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        if argv.len() == 1 || argv.len() == 2 {
            if let Some(rl) = runtime.rl.as_mut() {
                rl.save_history(&runtime.history_path).unwrap();
            }
        }

        match argv.len() {
            0 => {
                panic!("command name not passed in argv[0]");
            },
            1 => {
                process::exit(0)
            },
            2 => {
                if let Ok(n) = str::parse(argv[1].to_str().unwrap()) {
                    process::exit(n)
                } else {
                    process::exit(2)
                }
            },
            _ => {
                eprintln!("too many arguments");
                Ok(WaitStatus::Exited(Pid::this(), 1))
            }
        }
    }
}
