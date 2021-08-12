//! Single command programs with no features.
use std::{
    io::BufRead,
    ffi::CString,
};
use nix::sys::wait::WaitStatus;
use crate::{
    process::{ProcessGroup, Process},
    program::{Runtime, Result, Error},
};


/// A basic program with only a single command.
#[derive(Debug)]
pub struct Program(Vec<Command>);

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
        Ok(Program(vec![Command(command)]))
    }

    /// Return the single parsed command.
    fn commands(&self) -> &[Self::Command] {
        &self.0[..]
    }
}


/// A single poorly parsed command.
#[derive(Debug)]
pub struct Command(String);

impl super::Command for Command {}

impl super::Run for Command {
    fn run(&self, runtime: &mut Runtime) -> Result<WaitStatus> {
        let mut job = Process::new(self.0.split_whitespace().map(|a| {
            CString::new(a).expect("error reading argument")
        }).collect());

        let status = if runtime.background {
            let status = job.fork(runtime.io);
            runtime.jobs.borrow_mut().push(("???".into(), ProcessGroup(job)));
            status
        } else {
            job.fork_and_wait(runtime.io)
        };
        match status {
            Ok(WaitStatus::Exited(p, c)) if c == 0 => {
                Ok(WaitStatus::Exited(p, c))
            },
            _ => Err(Error::Runtime),
        }
    }
}
