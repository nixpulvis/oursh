//! Hand-parsed program syntax.
use std::io::Read;
use std::ffi::CString;

/// Source program representation, used mainly for parsing.
pub struct BasicProgram {
    source: String,
}

impl super::Program for BasicProgram {
    type Command = Vec<CString>;

    /// Create a new program from the given reader.
    ///
    /// ```
    /// use oursh::program::Program;
    /// use oursh::program::basic::BasicProgram;
    ///
    /// let program = BasicProgram::parse(b"ls" as &[u8]);
    /// ```
    fn parse<R: Read>(mut reader: R) -> Self {
        let mut source = String::new();
        reader.read_to_string(&mut source).expect("TODO");

        BasicProgram {
            source: source,
        }
    }

    /// Return the single parsed command.
    fn commands(&self) -> Vec<Self::Command> {
        vec![self.source.split_whitespace().map(|a| {
            CString::new(a).expect("error reading string argument")
        }).collect()]
    }
}


// TODO: impl Iterator<Item=R: Read> for BasicProgram?


#[cfg(test)]
mod tests {
    use super::*;
    use program::Program;

    // TODO: Should this work?
    #[test]
    fn test_empty_program() {
        let program = BasicProgram::parse(b"" as &[u8]);
    }
}
