//! Commands that are run from the shell directly, without forking another
//! process.
//!
//! These commands take precedence over any executables with the same name
//! in the `$PATH`.
use std::{
    env::{self, set_var},
    process,
    ffi::CString,
};
use nix::{
    unistd::{chdir, Pid},
    sys::wait::WaitStatus,
};
use crate::{
    program::{Result, Error},
    process::Jobs as JobsRef,
};

/// A builtin is a custom shell command, often changing the state of the
/// shell in some way.
pub trait Builtin {
    /// Execute the shell builtin command, returning a retult of the
    /// completion.
    fn run(argv: Vec<CString>, jobs: &mut JobsRef) -> Result<WaitStatus>;
}

/// Exit builtin, alternative to ctrl-d.
pub struct Exit;

impl Builtin for Exit {
    fn run(argv: Vec<CString>, _: &mut JobsRef) -> Result<WaitStatus> {
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

/// Export builtin, used to set global variables.
pub struct Export;

impl Builtin for Export {
    fn run(argv: Vec<CString>, _: &mut JobsRef) -> Result<WaitStatus> {
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

/// Change directory (`cd`) builtin.
pub struct Cd;

impl Builtin for Cd {
    fn run(argv: Vec<CString>, _: &mut JobsRef) -> Result<WaitStatus> {
        match argv.len() {
            0 => {
                panic!("command name not passed in argv[0]");
            },
            1 => {
                let home = match env::var("HOME") {
                    Ok(path) => path,
                    Err(_) => return Err(Error::Runtime),
                };
                let dst = home.as_str();
                chdir(dst).map(|_| {
                    set_var("PWD", &dst);
                    WaitStatus::Exited(Pid::this(), 0)
                })
                          .map_err(|_| Error::Runtime)
            },
            2 => {
                let dst = argv[1].to_string_lossy();
                chdir(dst.as_ref()).map(|_| {
                        set_var("PWD", dst.as_ref());
                        WaitStatus::Exited(Pid::this(), 0)
                    })
                    .map_err(|_| Error::Runtime)
            },
            _ => {
                eprintln!("too many arguments");
                Ok(WaitStatus::Exited(Pid::this(), 1))
            }
        }
    }
}

/// Noop builtin, same idea as `true`.
pub struct Null;

impl Builtin for Null {
    fn run(_: Vec<CString>, _: &mut JobsRef) -> Result<WaitStatus> {
        Ok(WaitStatus::Exited(Pid::this(), 0))
    }
}

/// Background job information.
pub struct Jobs;

impl Builtin for Jobs {
    fn run(_: Vec<CString>, jobs: &mut JobsRef) -> Result<WaitStatus> {
        for (id, job) in jobs.borrow().iter() {
            println!("[{}]\t{}\t\t{}",
                     id, job.leader().pid(), job.leader().body());
        }
        Ok(WaitStatus::Exited(Pid::this(), 0))
    }
}
