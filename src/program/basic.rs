use std::io::Read;
use std::ffi::CString;

/// Source program representation, used mainly for parsing.
///
/// TODO: Build AST instead of String?
/// TODO: Parse sequence of programs from stream.
/// TODO: POSIX and Modern varients.
pub struct BasicProgram {
    /// TODO: This should be removed, and/or made private.
    pub source: String,
}

impl super::Parser for BasicProgram {
    type Target = BasicProgram;

    /// Create a new program from a line of the given reader.
    ///
    /// ```
    /// use oursh::program::basic::BasicProgram;
    /// use oursh::program::Parser;
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
}

impl super::Program for BasicProgram {
    /// Return an `exec` style argv vector for this program.
    // TODO: Proper parsing should have already collected this.
    fn argv(&self) -> Vec<CString> {
        self.source.split_whitespace().map(|a| {
            CString::new(a).expect("error reading string argument")
        }).collect()
    }
}


// TODO: impl Iterator<Item=R: Read> for BasicProgram?


#[cfg(test)]
mod tests {
    use super::*;
    use program::Parser;

    // TODO: Should this work?
    #[test]
    fn test_empty_program() {
        let program = BasicProgram::parse(b"" as &[u8]);
    }
}
