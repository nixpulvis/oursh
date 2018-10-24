//! Single command programs with no features.
use std::{
    io::BufRead,
    ffi::CString,
};
use nix::sys::wait::WaitStatus;
use crate::{
    job::Job,
    program::{Result, Error},
};


/// A basic program with only a single command.
#[derive(Debug)]
pub struct Program(Vec<Box<Command>>);

impl super::Program for Program {
    type Command = Command;

    /// Create a new program from the given reader.
    ///
    /// ```
    /// use oursh::program::{Program, BasicProgram};
    ///
    /// BasicProgram::parse(b"ls" as &[u8]);
    /// ```
    fn parse<R: BufRead>(mut reader: R) -> Result<Self> {
        let mut command = String::new();
        if reader.read_to_string(&mut command).is_err() {
            return Err(Error::Read);
        }
        Ok(Program(vec![box Command(command)]))
    }

    /// Return the single parsed command.
    fn commands(&self) -> &[Box<Self::Command>] {
        &self.0[..]
    }
}


/// A single poorly parsed command.
#[derive(Debug)]
pub struct Command(String);

impl super::Command for Command {
    /// Treat each space blindly as an argument delimiter.
    fn run(&self) -> Result<WaitStatus> {
        let mut job = Job::new(self.0.split_whitespace().map(|a| {
            CString::new(a).expect("error reading argument")
        }).collect());

        match job.run() {
            Ok(WaitStatus::Exited(p, c)) if c == 0 => {
                Ok(WaitStatus::Exited(p, c))
            },
            _ => Err(Error::Runtime),
        }
    }
}
