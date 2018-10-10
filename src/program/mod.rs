//! Parser and interpreter for the syntax(es) of the shell.
//!
//! Both commands entered to the shell through STDIN and read from a file are
//! *programs*, and are parsed and handled by this module.
//!
//! ### POSIX Syntax
//! ### Modern Syntax Extensions

use std::io::Read;
use std::ffi::CString;

pub trait Parser {
    type Target;

    fn parse<R: Read>(reader: R) -> Self::Target;
}

pub trait Program {
    fn argv(&self) -> Vec<CString>;
}

// The various program grammars.
pub mod basic;
pub use self::basic::BasicProgram;
pub mod simple;
pub use self::simple::SimpleProgram;

// Some conveniant definitions.

pub type DefaultProgram = self::basic::BasicProgram;
// pub type DefaultProgram = self::simple::SimpleProgram;

pub fn parse<R: Read>(reader: R) -> DefaultProgram {
    DefaultProgram::parse(reader)
}

