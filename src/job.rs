//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::process::exit;
use std::ffi::CString;
use nix::unistd::{self, execvp, Pid, ForkResult};
use nix::sys::wait::{waitpid, WaitStatus};

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
    // TODO: Status should be part of `child`.
    // TODO: Use our own type, so downstream use doesn't need `nix`.
    status: Option<WaitStatus>,
}

impl Job {
    /// Create a new job from the given command.
    // TODO #4: Return result.
    pub fn new(argv: Vec<CString>) -> Self {
        Job {
            argv: argv,
            child: None,
            status: None,

        }
    }

    /// Run a shell job, waiting for the command to finish.
    pub fn run(&mut self) -> nix::Result<WaitStatus> {
        self.fork_and_wait()
    }

    /// Run a shell job in the background.
    pub fn run_background(&mut self) -> nix::Result<()> {
        self.fork()
    }

    fn fork(&mut self) -> Result<(), nix::Error> {
        match unistd::fork() {
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
        match unistd::fork() {
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
