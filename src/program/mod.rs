//! Parser and interpreter for the syntax(es) of the shell.
//!
//! Both commands entered to the shell through STDIN and read from a file are
//! *programs*, and are parsed and handled by this module.
//!
//! - TODO: Parse sequence of programs from stream.
//! - TODO: POSIX and Modern varients.
//!
//! ### POSIX Syntax
//! ### Modern Syntax

use std::io::Read;
use std::ffi::CString;

/// A command is a task given by the user as part of a `Program`.
///
/// Each command is handled by a `Job`. A single command may be run
/// multiple times each as a new `Job` but as the same `Command`.
// TODO: Make this a trait too.
pub type Command = Vec<CString>;


/// A program is a collection of commands given by the user.
pub trait Program {
    /// Parse a whole program from the given `reader`.
    fn parse<R: Read>(reader: R) -> Self;

    /// Return a list of all the commands in this program.
    // NOTE: Execution should *not* simply be running each in order.
    fn commands(&self) -> Vec<Command>;
}


// Some conveniant definitions.

/// The default program type, used for unannotated blocks.
// pub type DefaultProgram = self::basic::BasicProgram;
// pub type DefaultProgram = self::simple::SimpleProgram;
pub type DefaultProgram = self::posix::PosixProgram;

/// Parse a program of the default type.
///
/// # Examples
///
/// ```
/// use oursh::program::parse_default;
///
/// let program = parse_default(b"ls" as &[u8]);
/// ```
pub fn parse_default<R: Read>(reader: R) -> DefaultProgram {
    DefaultProgram::parse(reader)
}

/// Parse a program of the given type.
///
/// # Examples
///
/// ```
/// use oursh::program::parse;
/// use oursh::program::simple::SimpleProgram;
///
/// let program = parse::<SimpleProgram, &[u8]>(b"ls" as &[u8]);
/// ```
pub fn parse<P: Program, R: Read>(reader: R) -> P {
    P::parse(reader)
}


// The various program grammars.

pub mod basic;
pub use self::basic::BasicProgram;
pub mod simple;
pub use self::simple::SimpleProgram;
pub mod posix;
pub use self::posix::PosixProgram;
