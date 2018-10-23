//! Command jobs for foreground and background evaluation.
//!
//! All commands (both foreground and background) are created and executed as
//! a *job*. This helps manage the commands the shell runs.

use std::thread;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use program::{Error, Command};

pub struct Job<'a> {
    command: &'a Command,
    last_status: Option<WaitStatus>,
    is_background: bool,
}

/// A job to be executed by various means.
///
/// The shell's main job (pun intended) is to run commands. Each job has various arguments, and
/// rules about what things should be done.
///
/// - TODO #6: Background example.
impl<'a> Job<'a> {
    pub fn new(command: &'a Command) -> Self {
        Job {
            command: command,
            last_status: None,
            is_background: false,
        }
    }

    pub fn background(mut self, b: bool) -> Self {
        self.is_background = b;
        self
    }

    pub fn id() -> usize {
        1
    }

    pub fn status(&self) -> Option<WaitStatus> {
        self.last_status
    }

    pub fn is_background(&self) -> bool {
        self.is_background
    }

    pub fn run(&mut self) -> Result<WaitStatus, Error> {
        if self.is_background() {
            let handle = thread::spawn(move || {
                self.command.eval();
            });
            println!("{:?}", handle);
            Ok(WaitStatus::Exited(Pid::this(), 0))
        } else {
            self.command.eval()
        }
    }
}
