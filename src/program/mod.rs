//! Parsing and handling program syntax(es) of the shell.
//!
//! Both, commands entered to the shell interactively through STDIN, and read
//! from a file, are *programs*. Our shell provides multiple discrete languages
//! to write programs in, each with a corresponding implementation of this
//! module's `Program` trait.
//!
//! ### POSIX Shell Language
//!
//! The basic, portable POSIX shell language. For detailed information read the
//! [`posix`](crate::program::posix) module docs.
//!
//! ```sh
//! for ((i=0; i<10; i++)); do echo $i; done
//! ```
//!
//! ### Modern Shell Language
//!
//! A more modern and ergonomic language. For more detailed information read
//! the [`modern`](crate::program::modern) module docs.
//!
//! ```sh
//! # WIP
//! for i in (0..10) { echo $i }
//! ```
//!
//! ### Default Syntax
//!
//! Our shell has a `PrimaryProgram` which is in charge of parsing programs
//! which are not passed in via a `{#}` language block. This can be configured
//! to your preference. This does not effect the shell when launched in
//! POSIX compatibility mode, or when a specific default language is passed
//! as a flag.
//!
//! ### `{#}` Language Blocks
//!
//! Both the [`posix`](crate::program::posix) and
//! [`modern`](crate::program::modern) languages have support for a special
//! expression which treats the body as a program from another language. This
//! forms the basis of oursh's modern features, and backwards compatibility.
//! While the primary goal of this syntax is to be able to mix both POSIX and
//! non-POSIX shell scripts, the feature is much more powerful.  Any
//! interperator can be used, just like with `#!`.
//!
//! ```sh
//! date      # Call `date` in the primary syntax.
//! {# date}  # Specifies the alternate syntax.
//!
//! # Use ruby, why not...
//! {#!ruby
//!     require 'date'
//!     puts Date.today
//! }
//! ```
//!
//! Strict POSIX compatibility can be enabled by removing this feature alone.
//!
//! - TODO #5: Parse sequence of programs from stream.
//! - TODO #5: Partial parses for readline-ish / syntax highlighting.

use crate::process::jobs;
use nix::{sys::wait::WaitStatus, unistd::Pid};
use std::{ffi::CString, fmt::Debug, io::BufRead, result};

/// Convenience type for results with program errors.
pub type Result<T> = result::Result<T, Error>;

/// A comprehensive error type for the operation of programs.
#[derive(Debug)]
pub enum Error {
    /// A general issue reading the program.
    // TODO: Wrap an io error?
    Read,
    /// An error within the lexer or parser.
    // TODO: Wrap both our lex::Error and ParseError.
    Parse,
    /// An error encountered during the evaluation of a program.
    // TODO: Propagate status.
    // TODO: Just wrap an Wait/ExitStatus?
    Runtime,
}

pub trait Run {
    fn run(&self, runtime: &mut Runtime) -> Result<WaitStatus>;
}

/// A program is as large as a file or as small as a line.
///
/// Each program is to be treated as a complete single language entity, with
/// the explicit exception of `{#}` blocks, which act as bridges between
/// programs.
///
/// ### Working Thoughts
///
/// - Is simply iterating a collection of `commands` really the correct
///   semantics for all the types of programs we want?
/// - What language information do we still need to store?
pub trait Program: Sized + Debug + Run {
    /// The type of each of this program's commands.
    type Command: Command;

    /// Parse a whole program from the given `reader`.
    fn parse<R: BufRead>(reader: R) -> Result<Self>;

    /// Return a list of all the commands in this program.
    fn commands(&self) -> &[Self::Command];
}

impl<P: Program> Run for P {
    fn run(&self, runtime: &mut Runtime) -> Result<WaitStatus> {
        let mut last = WaitStatus::Exited(Pid::this(), 0);
        for command in self.commands().iter() {
            last = command.run(runtime)?;
        }
        Ok(last)
    }
}

/// A command is a task given by the user as part of a [`Program`](Program).
///
/// Each command is handled by a [`Process`](crate::process::Process), and a
/// single command may be run multiple times each as a new `Process`. Each time
/// a command is run, the conditions within the control of the shell are
/// reproduced; IO redirection, working directory, and even the environment are
/// each faithfully preserved.
pub trait Command: Sized + Debug + Run {
    /// Return the name of this command.
    ///
    /// This name *may* not be the same as the name given to the process by
    /// the running [`Process`](crate::process::Process).
    // TODO: Ids?
    fn name(&self) -> CString {
        CString::new(format!("{:?}", self)).expect("error in UTF-8 of format")
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
/// parse_primary(b"ls | wc" as &[u8]);
/// ```
pub fn parse_primary<R: BufRead>(reader: R) -> Result<PrimaryProgram> {
    PrimaryProgram::parse(reader)
}

/// Parse a program of the alternate type.
///
/// # Examples
///
/// ```
/// use oursh::program::parse_alternate;
///
/// parse_alternate(b"ls" as &[u8]);
/// ```
pub fn parse_alternate<R: BufRead>(reader: R) -> Result<AlternateProgram> {
    AlternateProgram::parse(reader)
}

/// Parse a program of the given type.
///
/// # Examples
///
/// ```
/// use oursh::program::{parse, PosixProgram, BasicProgram};
///
/// let program = b"sleep 1; date & date";
/// assert!(parse::<PosixProgram, &[u8]>(program).is_ok());
/// ```
pub fn parse<P: Program, R: BufRead>(reader: R) -> Result<P> {
    P::parse(reader)
}

// The various program grammars.
//
// If reading this code were like sking, you'd now be hitting blues. ASTs and
// language semantics are somewhat tricky subjects.

pub mod runtime;
pub use self::runtime::Runtime;

pub mod basic;
pub use self::basic::Program as BasicProgram;
pub mod posix;
pub use self::posix::Program as PosixProgram;
pub mod modern;
pub use self::modern::Program as ModernProgram;

// TODO: Replace program::Result
pub fn parse_and_run(text: &str, runtime: &mut Runtime) -> crate::program::Result<WaitStatus> {
    // Parse with the primary grammar and run each command in order.
    let program = match parse_primary(text.as_bytes()) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{:?}: {:#?}", e, text);
            return Err(e);
        }
    };

    #[cfg(feature = "history")]
    runtime.history.add(text, 1);

    // Print the program if the flag is given.
    if runtime.args.get_bool("--ast") {
        eprintln!("{:#?}", program);
    }

    // Run it!
    let result = program.run(runtime);
    // Check up on the background jobs.
    jobs::retain_alive(runtime.jobs);
    result
}
