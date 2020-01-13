//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::{
    borrow::Cow,
    process::exit,
    ffi::CString,
    cell::RefCell,
    rc::Rc,
    os::unix::io::RawFd,
};
use nix::{
    unistd::{self, execvp, dup2, Pid, ForkResult},
    sys::wait::{waitpid, WaitStatus, WaitPidFlag},
};

#[derive(Debug, Copy, Clone)]
pub struct IO(pub [RawFd; 3]);

impl Default for IO {
    fn default() -> Self {
        IO([0, 1, 2])
    }
}

/// A job to be executed by various means.
///
/// The shell's main job (pun intended) is to run commands. Each job has various arguments, and
/// rules about what things should be done.
///
/// - TODO #4: Redirection example.
/// - TODO #6: Background example.
/// - TODO #4: Environment example?
pub struct Job {
    argv: Vec<CString>,
    // TODO: Call this pid?
    child: Option<Pid>,
}

impl Job {
    /// Create a new job from the given command.
    // TODO #4: Return result.
    pub fn new(argv: Vec<CString>) -> Self {
        Job {
            argv,
            child: None,
        }
    }

    pub fn body(&self) -> String {
        self.argv.iter().map(|a| {
            a.to_string_lossy()
        }).collect::<Vec<Cow<str>>>().join(" ")
    }

    pub fn pid(&self) -> Option<Pid> {
        self.child
    }

    pub fn status(&self) -> nix::Result<WaitStatus> {
        match self.child {
            Some(child) => {
                waitpid(child, Some(WaitPidFlag::WNOHANG))
            },
            _ => unimplemented!(),
        }
    }

    /// Run a shell job in the background.
    pub fn fork(&mut self, io: IO) -> nix::Result<WaitStatus> {
        match unistd::fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.status()
            },
            Ok(ForkResult::Child) => {
                // TODO: DRY and generalize?
                dup2(io.0[0], 0)?;
                dup2(io.0[1], 1)?;
                dup2(io.0[2], 2)?;
                // TODO #20: When running with raw mode we could buffer
                // this and print it later, all at once in suspended raw mode.
                if let Err(_) = self.exec() {
                    exit(127);
                } else {
                    self.status()
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Run a shell job, waiting for the command to finish.
    pub fn fork_and_wait(&mut self, io: IO) -> nix::Result<WaitStatus> {
        match unistd::fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.wait()
            },
            Ok(ForkResult::Child) => {
                // TODO: DRY and generalize?
                dup2(io.0[0], 0)?;
                dup2(io.0[1], 1)?;
                dup2(io.0[2], 2)?;
                if let Err(_) = self.exec() {
                    exit(127);
                } else {
                    // TODO: Waiting in the child?
                    unimplemented!();
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

    fn wait(&mut self) -> nix::Result<WaitStatus> {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        // TODO #4: Cover other cases?
                        Ok(WaitStatus::StillAlive) => {},
                        s @ Ok(WaitStatus::Exited(_, 127)) => {
                            let name = self.argv[0].to_string_lossy();
                            eprintln!("oursh: {}: command not found", name);
                            return s;
                        },
                        s => return s,
                    };
                }
            },
            _ => unimplemented!(),
        }
    }
}

pub type Jobs = Rc<RefCell<Vec<(String, Job)>>>;
