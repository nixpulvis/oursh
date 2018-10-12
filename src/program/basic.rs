//! Single command programs with no features.
use std::io::BufRead;
use std::ffi::CString;


/// A basic program with only a single command.
pub struct Program(Vec<Box<BasicCommand>>);

impl super::Program for Program {
    type Command = BasicCommand;

    /// Create a new program from the given reader.
    ///
    /// ```
    /// use oursh::program::{Program, BasicProgram};
    ///
    /// BasicProgram::parse(b"ls" as &[u8]);
    /// ```
    fn parse<R: BufRead>(mut reader: R) -> Result<Self, ()> {
        let mut command = String::new();
        if reader.read_to_string(&mut command).is_err() {
            return Err(());
        }
        Ok(Program(vec![box BasicCommand(command)]))
    }

    /// Return the single parsed command.
    fn commands(&self) -> &[Box<Self::Command>] {
        &self.0[..]
    }
}


/// A single poorly parsed command.
#[derive(Clone)]
pub struct BasicCommand(String);

impl super::Command for BasicCommand {
    /// Treat each space blindly as an argument delimiter.
    fn argv(&self) -> Vec<CString> {
        self.0.split_whitespace().map(|a| {
            CString::new(a).expect("error reading argument")
        }).collect()
    }
}
