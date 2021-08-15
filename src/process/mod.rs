//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.
//!
//! - Job control
//! - Threads?
//! - Signals
//! - Process groups
//! - Sessions
//!
//! More resources from the source at:
//! [www.win.tue.nl/~aeb/linux/lk/lk-10](https://www.win.tue.nl/~aeb/linux/lk/lk-10.html).

use std::{
    borrow::Cow,
    process::exit,
    ffi::CString,
};
use nix::{
    errno::Errno,
    unistd::{self, execvp, getpid, Pid, ForkResult},
    sys::wait::{waitpid, WaitStatus, WaitPidFlag},
};

mod io;
pub use self::io::IO;
pub mod jobs;
pub use self::jobs::Jobs;
mod session;
mod signal;
mod thread;


/// A process to be executed by various means
///
/// The shell's main job is to run commands. Each job has various arguments, and rules about what
/// things should be done.
///
/// - TODO #4: Redirection example.
/// - TODO #6: Background example.
/// - TODO #4: Environment example?
#[derive(Debug)]
pub struct Process {
    argv: Vec<CString>,
    pid: Pid,
}

impl Process {
    /// Create a new job from the given command.
    // TODO #4: Return result.
    pub fn new(argv: Vec<CString>) -> Self {
        Process {
            argv,
            pid: getpid(),
        }
    }

    pub fn body(&self) -> String {
        self.argv.iter().map(|a| {
            a.to_string_lossy()
        }).collect::<Vec<Cow<str>>>().join(" ")
    }

    pub fn pid(&self) -> Pid {
        self.pid
    }

    /// Run a shell job in the background.
    pub fn fork(argv: Vec<CString>, io: IO) -> Result<Self, nix::Error> {
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Parent { child }) => {
                Ok(Process {
                    argv,
                    pid: child,
                })
            },
            Ok(ForkResult::Child) => {
                let process = Process {
                    argv,
                    pid: getpid(),
                };
                io.dup()?;
                if let Err(e) = process.exec() {
                    match e {
                        Errno::ENOENT => {
                            let name = process.argv[0].to_string_lossy();
                            eprintln!("oursh: {}: command not found", name);
                            exit(127);
                        },
                        _ => exit(128),
                    }
                } else {
                    unreachable!()
                }
            },
            Err(e) => Err(e),
        }
    }

    fn exec(&self) -> Result<(), nix::Error> {
        execvp(&self.argv[0], &self.argv.iter()
                                        .map(|a| a.as_c_str())
                                        .collect::<Vec<_>>()[..]).map(|_| ())
    }
}

pub trait Wait {
    fn wait(&self) -> nix::Result<WaitStatus>;
    fn status(&self) -> nix::Result<WaitStatus>;
}

impl Wait for Pid {
    fn wait(&self) -> nix::Result<WaitStatus> {
        waitpid(Some(*self), None)
    }

    fn status(&self) -> nix::Result<WaitStatus> {
        waitpid(Some(*self), Some(WaitPidFlag::WNOHANG))
    }
}

impl Wait for Process {
    fn wait(&self) -> nix::Result<WaitStatus> {
        self.pid.wait()
    }

    fn status(&self) -> nix::Result<WaitStatus> {
        self.pid.status()
    }
}


/// Processes groups are used for things like pipelines and background jobs
///
/// The system call `int setpgid(pid_t pid, pid_t pgid)` is used to set.
///
/// Every process is member of a unique process group, identified by its process group ID. (When
/// the process is created, it becomes a member of the process group of its parent.) By convention,
/// the process group ID of a process group equals the process ID of the first member of the
/// process group, called the process group leader. A process finds the ID of its process group
/// using the system call getpgrp(), or, equivalently, getpgid(0). One finds the process group ID
/// of process p using getpgid(p).
#[derive(Debug)]
pub struct ProcessGroup(pub Process); // TODO: Make a sorted vector of Process?

impl ProcessGroup {
    pub fn leader(&self) -> &Process {
        &self.0
    }

    pub fn leader_mut(&mut self) -> &mut Process {
        &mut self.0
    }
}
