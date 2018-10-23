//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::process::exit;
use std::ffi::CString;
use nix::unistd::{execvp, fork, Pid, ForkResult};
use nix::sys::wait::{waitpid, WaitStatus};

///
/// - TODO #4: Redirection example.
/// - TODO #4: Environment example?
pub struct Exec {
    argv: Vec<CString>,
    // TODO: Call this pid?
    child: Option<Pid>,
    // TODO: Status should be part of `child`.
    // TODO: Use our own type, so downstream use doesn't need `nix`.
    status: Option<WaitStatus>,
}

impl Exec {
    /// Create a new executable from the given arguments.
    pub fn new(argv: Vec<CString>) -> Self {
        Exec {
            argv: argv,
            child: None,
            status: None,
        }
    }

    /// Run the executable, waiting for it to finish.
    pub fn run(&mut self) -> nix::Result<WaitStatus> {
        self.fork_and_wait()
    }

    /// Run the executable, not waiting, and returning a handle.
    pub fn run_background(&mut self) -> nix::Result<()> {
        self.fork()
    }

    fn fork(&mut self) -> Result<(), nix::Error> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                Ok(())
            },
            Ok(ForkResult::Child) => {
                // TODO #20: When running with raw mode we could buffer
                // this and print it later, all at once in suspended raw mode.
                if let Err(_) = self.exec() {
                    exit(127);
                } else {
                    Ok(())
                }
            },
            Err(e) => Err(e),
        }
    }

    fn fork_and_wait(&mut self) -> nix::Result<WaitStatus> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.wait()
            },
            Ok(ForkResult::Child) => {
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
        execvp(&self.argv[0], &self.argv).map(|_| ())
    }

    fn wait(&self) -> nix::Result<WaitStatus> {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        // TODO #4: Cover other cases?
                        Ok(WaitStatus::StillAlive) => {},
                        s => return s,
                    };
                }
            },
            _ => unimplemented!(),
        }
    }
}
