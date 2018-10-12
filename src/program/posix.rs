//! The ubiquitous POSIX shell command language.
//!
//! This shell language (often called `sh`) is at the heart of the most popular
//! shells, namely `bash` and `zsh`. While shells typically implement many
//! extensions to the POSIX standard we'll be implementing only the most basic
//! set of functionality and offloading all extensions to the `modern` language.
//!
//! # Compatibility
//!
//! Shell languages like `bash` or `zsh` are **supersets** of the POSIX `sh`
//! language. This means two things:
//!
//! - All `sh` programs are valid `bash`, `zsh`, etc programs
//! - Not all `bash` programs, for example, are valid `sh` programs.
//!
//! This explains why some shell scripts will start with `#!/bin/sh` or
//! `#!/bin/bash`, depending on what features of the language the script needs.
//!
//! # Examples
//!
//! There are more than enough examples of `sh` scripts out there, but here is a
//! collection of examples tested in **this** shell's implementation of the
//! POSIX standard. This section will not be a complete description of the
//! syntax of the `sh` language, but will be updated with as many interesting
//! cases as possible.
//!
//! Running a command (like `date`) in a shell script is the simplest thing
//! you can do.
//!
//! ```sh
//! date
//! date --iso-8601
//!
//! # You can even run programs from outside the $PATH.
//! ./a.out
//! ```
//!
//! All variables start with a `$` when referring to them, but when assigning
//! you omit the `$`. It's also worth mentioning that the lack of whitespace
//! around the `=` in assignment **is required**. It's often conventional to
//! write your variable names in all caps, but this is not a limitation of the
//! language.
//!
//! ```sh
//! NAME="Nathan Lilienthal"
//! i=0
//!
//! echo $NAME
//! echo $i
//! ```
//!
//! In addition to variables beginning with a `$` being expanded to the value
//! they were set to, other syntax can perform expansion. See section 3§2.6 for
//! a complete description of word expansion.
//!
//! ```sh
//! # Same as echo $1.
//! echo ${1}
//! # Use a default.
//! echo ${1:-default}
//! # Assign and use a default.
//! echo ${1:=default}
//! # Fail with error if $1 is unset.
//! echo ${1:?}
//! # Replace if not null.
//! echo ${1:+new}
//! # String length of $1.
//! echo ${#1}
//! # Remove suffix/prefix strings.
//! echo ${1%.*}
//! echo ${1%%.*}
//! echo ${1#prefix_}
//! ```
//!
//! In addition to running a program at the top level, programs can be run in
//! a subshell with mechanisms to capture the output. This is called command
//! substitution.
//!
//! ```sh
//! # Assign $files to be the output of ls.
//! files=`ls`
//! files=$(ls)  # Same as above.
//! ```
//!
//! Conditionals in the wild are often written with the non-POSIX `[[` syntax,
//! traditional conditional checks use either `test` or `[`.
//!
//! ```sh
//! # Check if $1 is absent.
//! if test -z "$1"; then
//!     exit 1
//! fi
//!
//! # Check if $1 is equal to "foo".
//! if [ "$1" -eq "foo" ]; then
//!     echo "bar"
//! fi
//! ```
//!
//! # Specification
//!
//! The syntax and semantics of this module are strictly defined by the POSIX
//! (IEEE Std 1003.1) standard, in section 3§2 of "The Open Group Base
//! Specifications" [[1]].
//!
//! [1]: http://pubs.opengroup.org/onlinepubs/9699919799/

use std::io::BufRead;
use std::ffi::CString;

pub use self::ast::Program;
pub use self::ast::Command;

impl super::Program for Program {
    type Command = Command;

    fn parse<R: BufRead>(mut reader: R) -> Self {
        let mut string = String::new();
        reader.read_to_string(&mut string).unwrap();
        let parsed = lalrpop::ProgramParser::new().parse(&string).unwrap();
        parsed
    }

    fn commands(&self) -> &[Box<Self::Command>] {
        &self.0[..]
    }
}

impl super::Command for Command {
    fn argv(&self) -> Vec<CString> {
        match *self {
            Command::Simple(ref words) => {
                words.iter().map(|w| {
                    CString::new(&w.0 as &str).unwrap()
                }).collect()
            },
            _ => unimplemented!(),
        }
    }
}

/// Abstract Syntax Tree for the POSIX language.
pub mod ast {
    /// A program is the result of parsing a sequence of commands.
    #[derive(Debug)]
    pub struct Program(pub Vec<Box<Command>>);

    /// A command is a highly recursive node with the main features
    /// of the POSIX language.
    #[derive(Debug)]
    pub enum Command {
        /// Performs boolean negation to the status code of the inner
        /// command.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// ! grep 'password' data.txt
        /// ```
        Not(Box<Command>),
        /// Perform the first command, conditionally running the next
        /// upon success.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// mkdir tmp && cd tmp
        /// ```
        And(Box<Command>, Box<Command>),
        /// Perform the first command, conditionally running the next
        /// upon failure.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// kill $1 || kill -9 $1
        /// ```
        Or(Box<Command>, Box<Command>),
        /// Run the inner **program** in a sub-shell environment.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// DATE=(date)
        /// ```
        Subshell(Box<Program>),
        /// Run a command's output through to the input of another.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// cat $1 | wc -l
        /// ```
        Pipeline(Box<Command>, Box<Command>),
        /// Run a command in the background.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// while true; do
        ///   sleep 1; echo "ping";
        /// done &
        /// ```
        Background(Box<Command>),
        /// Just a single command, with it's arguments.
        ///
        /// ### Examples
        ///
        /// ```sh
        /// date --iso-8601
        /// ```
        // TODO: Simple should not just be a vec of words.
        Simple(Vec<Word>),
    }

    /// A parsed word, already having gone through expansion.
    // TODO: How can we expand things like $1 or $? from the lexer?
    // TODO: This needs to handle escapes and all kinds of fun. We first
    //       need to decide on our custom Tokens and lexer.
    #[derive(Debug)]
    pub struct Word(pub String);
}

lalrpop_mod!(lalrpop, "/program/posix.rs");
