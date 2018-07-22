//! Parser and interpreter for the syntax(es) of the shell.
//!
//! Both commands entered to the shell through STDIN and read from a file are
//! *programs*, and are parsed and handled by this module.
//!
//! ### POSIX Syntax
//! ### Modern Syntax Extensions

use std::io::Read;

/// Source program representation, used mainly for parsing.
///
/// TODO: Build AST instead of String?
/// TODO: Parse sequence of programs from stream.
/// TODO: POSIX and Modern varients.
pub struct Program {
    /// TODO: This should be removed, and/or made private.
    pub source: String,
}

impl Program {
    /// Create a new program from a line of the given reader.
    ///
    /// ```
    /// use oursh::program::Program;
    ///
    /// let program = Program::parse(b"ls" as &[u8]);
    /// ```
    pub fn parse<R: Read>(mut reader: R) -> Self {
        let mut source = String::new();
        reader.read_to_string(&mut source).expect("TODO");

        Program {
            source: source,
        }
    }
}

// TODO: impl Iterator<Item=R: Read> for Program?

#[cfg(test)]
mod tests {
    use super::*;

    // TODO
}
