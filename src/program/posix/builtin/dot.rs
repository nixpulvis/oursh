use std::{
    io::Read,
    fs::File,
    ffi::CString,
};
use nix::{
    unistd::Pid,
    sys::wait::WaitStatus,
};
use crate::{
    program::posix::builtin::Builtin,
    program::{Result, Runtime, parse_and_run},
};

/// Execute commands from `file` in the current environment
///
/// TODO:
/// If file does not contain a `/`, the shell shall use the search path
/// specified by `PATH` to find the directory containing file. Unlike normal
/// command search, however, the file searched for by the `.` utility need not
/// be executable. If no readable file is found, a non-interactive shell shall
/// abort; an interactive shell shall write a diagnostic message to standard
/// error, but this condition shall not be considered a syntax error.
pub struct Dot;

impl Builtin for Dot {
    fn run(self, argv: Vec<CString>, runtime: &mut Runtime) -> Result<WaitStatus> {
        match argv.len() {
            0 => unreachable!(),
            1 => {
                eprintln!("filename argument required");
                Ok(WaitStatus::Exited(Pid::this(), 2))
            }
            2 => {
                let path = argv[1].to_str().unwrap();
                if let Ok(mut file) = File::open(path) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        parse_and_run(&contents, runtime)
                    } else {
                        Ok(WaitStatus::Exited(Pid::this(), 1))
                    }
                } else {
                    Ok(WaitStatus::Exited(Pid::this(), 1))
                }
            },
            _ => unreachable!(),

        }
    }
}
