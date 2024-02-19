//! Single command programs with no features.
use crate::{
    process::{Process, ProcessGroup, Wait},
    program::{Error, Result, Runtime},
};
use nix::sys::wait::WaitStatus;
use std::{ffi::CString, io::BufRead};

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
        let argv = self
            .0
            .split_whitespace()
            .map(|a| CString::new(a).expect("error reading argument"))
            .collect();

        let status = if runtime.background {
            let job = Process::fork(argv, runtime.io).map_err(|_| Error::Runtime)?;
            let status = job.status();
            runtime
                .jobs
                .borrow_mut()
                .push(("???".into(), ProcessGroup(job)));
            status
        } else {
            let job = Process::fork(argv, runtime.io).map_err(|_| Error::Runtime)?;
            job.wait()
        };
        match status {
            Ok(WaitStatus::Exited(p, c)) if c == 0 => Ok(WaitStatus::Exited(p, c)),
            _ => Err(Error::Runtime),
        }
    }
}
