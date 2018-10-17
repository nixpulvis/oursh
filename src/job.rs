//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::process::exit;
use std::ffi::CString;
use nix::unistd::{execvp, fork, Pid, ForkResult};
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
    ///
    /// This function also does a simple lookup for builtin functions.
    // TODO #4: Return result.
    pub fn run(&mut self) -> Result<(), ()> {
        // TODO #4: Proper builtins, in program module.
        if self.argv.len() > 0 && self.argv[0].to_bytes() == b"exit" {
            exit(0);
        }

        // // TODO #4: This is a awful background parse :P
        // if self.argv.last().map(|s| s.to_bytes()) == Some(b"&") {
        //     self.argv.pop();
        //     self.fork()
        // } else {
            self.fork_and_wait()
        // }
    }

    fn fork(&mut self) -> Result<(), ()> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                Ok(())
            },
            Ok(ForkResult::Child) => {
                if let Err(()) = self.exec() {
                    exit(127);
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                println!("error: {}", e);
                Err(())
            }
        }
    }

    fn fork_and_wait(&mut self) -> Result<(), ()> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                // TODO: Check status.
                self.wait().map(|s| ()).map_err(|e| ())
            },
            Ok(ForkResult::Child) => {
                if let Err(()) = self.exec() {
                    exit(127);
                } else {
                    Ok(())
                }
            },
            Err(e) => {
                // println!("error: {}", e);
                Err(())
            }
        }
    }

    fn exec(&self) -> Result<(), ()> {
        // TODO #4: Where should we handle empty commands?
        if self.argv.len() == 0 {
            return Err(());
        }

        match execvp(&self.argv[0], &self.argv) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("error: {}", e);
                Err(())
            }
        }
    }

    fn wait(&self) -> Result<(), ()> {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        // TODO #4: Cover other cases?
                        Ok(WaitStatus::StillAlive) => {},
                        Ok(_) => return Ok(()),
                        Err(_) => return Err(()),
                    };
                }
                unreachable!();
            },
            _ => Err(()),
        }
    }
}
