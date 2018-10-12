//! Parsing and handling program syntax(es) of the shell.
//!
//! Both, commands entered to the shell interactively through STDIN, and read
//! from a file, are *programs*. Our shell provides multiple discrete languages
//! to write programs in, each with a corresponding implementation of this
//! module's `Program` trait.
//!
//! ### `{@}` Language Blocks
//!
//! Both the [`posix`](posix) and [`modern`](modern) languages have support for
//! a special expression which treats the body as a program from another
//! language. This forms the basis of oursh's modern features, and backwards
//! compatibility. While the primary goal of this syntax is to be able to mix
//! both POSIX and non-POSIX shell scripts, the feature is much more powerful.
//! Any interperator can be used, just like with `#!`.
//!
//! ```sh
//! date      # Call `date` in the primary syntax.
//! {@ date}  # Specifies the alternate syntax.
//!
//! # Use ruby, why not...
//! {@!ruby
//!     require 'date'
//!     puts Date.today
//! }
//! ```
//!
//! Strict POSIX compatibility can be enabled by removing this feature alone.
//!
//! ### Default Syntax
//!
//! Our shell has a `PrimaryProgram` which is in charge of parsing programs
//! which are not passed in via a `{@}` language block. This can be configured
//! to your preference. This does not effect the shell when launched in
//! POSIX compatibility mode, or when a specific default language is passed
//! as a flag.
//!
//! ### POSIX Syntax
//!
//! The basic, portable POSIX shell language. For detailed information read the
//! [`posix`](posix) module docs.
//!
//! ```sh
//! a=0
//! b=1
//! for ((i=0; i<10; i++))
//! do
//!     echo -n "$a "
//!     fn=$((a + b))
//!     a=$b
//!     b=$fn
//! done
//! echo
//! ```
//!
//! ### Modern Syntax
//!
//! A more modern and ergonomic language. For more detailed information read
//! the [`modern`](modern) module docs.
//!
//! ```sh
//! # WIP
//! var a = 0
//! var b = 1
//! for i in 0..10 {
//!     echo -n "$a "
//!     let fn = $((a + b))
//!     a = $b
//!     b = $fn
//! }
//! echo
//! ```
//!
//! ### TODO
//!
//! - Parse sequence of programs from stream.
//! - Partial parses for readline-ish / syntax highlighting.

use std::io::BufRead;
use std::ffi::CString;
use job::Job;

/// A command is a task given by the user as part of a [`Program`](Program).
///
/// Each command is handled by a `Job`, and a single command may be run multiple
/// times each as a new `Job`. Each time a command is run, the conditions
/// within the control of the shell are reproduced; IO redirection, working
/// directory, and even the environment are each faithfully preserved.
///
// TODO: We can reasonably reproduce the redirects, pwd... but is it
// sane to try this with ENV too?
pub trait Command {
    /// Return the command's arguments (including it's name).
    ///
    /// This function returns a vector as expected by the `exec(3)` family of
    /// functions.
    fn argv(&self) -> Vec<CString>;

    /// Return the name of this command.
    ///
    /// This name *may* not be the same as the name given to the process by
    /// the running `Job`.
    fn name(&self) -> CString {
        self.argv()[0].clone()
    }
}


/// A program is as large as a file or as small as a line.
///
/// Each program is to be treated as a complete single language entity, with
/// the explicit exception of `{@}` blocks, which act as bridges between
/// programs.
///
/// ### Working Thoughts
///
/// - Is simply iterating a collection of `commands` really the correct
/// semantics for all the types of programs we want?
/// - What language information do we still need to store?
pub trait Program: Sized {
    /// The type of each of this program's commands.
    type Command: Command;

    /// Parse a whole program from the given `reader`.
    fn parse<R: BufRead>(reader: R) -> Result<Self, ()>;

    /// Return a list of all the commands in this program.
    fn commands(&self) -> &[Box<Self::Command>];

    /// Run the program sequentially.
    fn run(&self) -> Result<(), ()> {
        for command in self.commands().iter() {
            Job::new(&**command).run();
        }
        Ok(())
    }
}


/// The primary program type, used for unannotated blocks.
// TODO: This should be `ModernProgram`.
pub type PrimaryProgram = PosixProgram;

/// TODO: alt explain
// TODO: This should be `PosixProgram`.
pub type AlternateProgram = BasicProgram;

/// Parse a program of the primary type.
///
/// # Examples
///
/// ```
/// use oursh::program::parse_primary;
///
/// parse_primary(b"ls" as &[u8]);
/// ```
pub fn parse_primary<R: BufRead>(reader: R) -> Result<PrimaryProgram, ()> {
    PrimaryProgram::parse(reader)
}

/// Parse a program of the given type.
///
/// # Examples
///
/// ```
/// use oursh::program::{parse, BasicProgram};
///
/// parse::<BasicProgram, &[u8]>(b"ls");
/// ```
pub fn parse<P: Program, R: BufRead>(reader: R) -> Result<P, ()> {
    P::parse(reader)
}


/// Abstract Syntax Tree for programs between multiple languages.
pub mod ast {
    /// Either explicit or implicit declaration of the interperator for
    /// a bridged program.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// {@ ...}
    /// {@ruby ...}
    /// ```
    #[derive(Debug)]
    pub enum Interpreter {
        Primary,
        Alternate,
        Other(String),
    }

    /// A program's text and the interperator to be used.
    // TODO: Include grammar seperate from interperator?
    #[derive(Debug)]
    pub struct BridgedProgram(pub Interpreter, pub String);
}

// Language bridge grammar macro.
lalrpop_mod!(lalrpop, "/program/mod.rs");

// The various program grammars.

pub mod basic;
pub use self::basic::Program as BasicProgram;
pub mod posix;
pub use self::posix::Program as PosixProgram;
