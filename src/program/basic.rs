//! Single command programs with no features.
use std::io::Read;
use std::ffi::CString;


/// A basic program with only a single command.
pub struct BasicProgram(Vec<Box<BasicCommand>>);

impl super::Program for BasicProgram {
    type Command = BasicCommand;

    /// Create a new program from the given reader.
    ///
    /// ```
    /// use oursh::program::{Program, BasicProgram};
    ///
    /// BasicProgram::parse(b"ls" as &[u8]);
    /// ```
    fn parse<R: Read>(mut reader: R) -> Self {
        let mut command = String::new();
        reader.read_to_string(&mut command).expect("error reading");
        BasicProgram(vec![box BasicCommand(command)])
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
