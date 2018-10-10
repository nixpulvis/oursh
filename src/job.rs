//! Subprocess execution management.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::ffi::CString;
use std::process::exit;
use nix::unistd::{execvp, fork, Pid, ForkResult};
use nix::sys::wait::{waitpid, WaitStatus};
use program::Command;

/// A job to be executed by various means.
///
/// The shell's main job (pun intended) is to run commands. Each job has various arguments, and
/// rules about what things should be done.
///
/// - TODO: Redirection example.
/// - TODO: Background example.
/// - TODO: Environment example?
pub struct Job {
    command: Command,
    // TODO: Call this pid?
    child: Option<Pid>,
}

impl Job {
    /// Create a new job from the given command.
    // TODO: Return result.
    pub fn new(command: &Command) -> Self {
        Job {
            command: command.to_owned(),
            child: None,
        }
    }

    /// Run a shell job, waiting for the command to finish.
    ///
    /// This function also does a simple lookup for builtin functions.
    // TODO: Return result.
    pub fn run(&mut self) {
        // TODO: Proper builtins, in program module.
        if self.command.len() > 0 && self.command[0].to_bytes() == b"exit" {
            exit(0);
        }

        // TODO: This is a awful background parse :P
        if self.command.last().map(|s| s.to_bytes()) == Some(b"&") {
            self.command.pop();
            self.fork();
        } else {
            self.fork_and_wait();
        }
    }

    fn fork(&mut self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
            },
            Ok(ForkResult::Child) => {
                self.exec();
            },
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    fn fork_and_wait(&mut self) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child = Some(child);
                self.wait();
            },
            Ok(ForkResult::Child) => {
                self.exec();
            },
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    fn exec(&self) {
        // TODO: Where should we handle empty commands?
        if self.command.len() == 0 {
            return;
        }

        match execvp(&self.command[0], &self.command) {
            Ok(_) => unreachable!(),
            Err(e @ _) => {
                println!("{}", e);
                exit(127);
            }
        }
    }

    fn wait(&self) {
        match self.child {
            Some(child) => {
                loop {
                    match waitpid(child, None) {
                        Ok(WaitStatus::StillAlive) => {},
                        // TODO: Cover other cases?
                        _ => break,
                    }
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Should this work?
    #[test]
    fn test_empty_program() {
        let mut job = Job::new(&vec![]);
        job.run();
    }
}
