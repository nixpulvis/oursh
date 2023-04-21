//! [![Documentation](https://docs.rs/oursh/badge.svg)](https://docs.rs/oursh)
//! [![CI](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml)
//! [![Dependencies](https://deps.rs/repo/github/nixpulvis/oursh/status.svg)](https://deps.rs/repo/github/nixpulvis/oursh)
//!
//!
//! This shell should be both POSIX compatible and yet modern and exciting.
//! Fancy features should not be prevented by POSIX compatibility. This will
//! effect the design of the shell.
//!
//! The name of the shell is `oursh` which is both somewhat unique, and
//! memorable.  It's also a nice name to play with pseudo-satirical themes...
//! right comrade?  It's short (ish) and sneakily fits `rs` in it, which is the
//! extension of Rust programs, the language this will be written in.
//!
//! ## Features
//!
//! - [ ] [POSIX compatibility](https://github.com/nixpulvis/oursh/milestone/1)
//!     - [x] Simple commands `ls`
//!     - [ ] Quotes (#28) `echo "foo"; echo 'bar'`
//!     - [x] Assignment `LOG=trace cargo run`
//!     - [x] Variables `echo $foo`
//!     - [ ] Special variables ($54) `echo $?; echo $1`
//!     - [x] Boolean status syntax `! true && false || true`
//!     - [x] Conditionals `if ; then ; elif ; then ; else ; fi`
//!     - [x] Compound commands `{ ls; date; }`
//!     - [ ] Subshells `\$(sleep 1; date)`
//!     - [x] Background jobs `{ sleep 1; date; }& date`
//!     - [x] Redirection `date > now.txt`
//!     - [ ] Pipes `ls | wc -l`
//! - [ ] Shebang block programs
//!     - [ ] Alternate syntax `{# ...}`
//!     - [ ] Hashlang syntax `{#lang; ...}`, i.e. `{#posix ls}`
//!     - [x] Shebang syntax `{#!/usr/bin/env ruby; puts :sym}`
//! - [ ] bash/zsh autocomplete compatibility
//!     - [ ] Command completion
//!     - [ ] Path completion
//!     - [ ] Variable completion
//!     - [ ] Job completion
//!     - [ ] Syntax completion
//!     - [ ] `man` / `-h` / `--help` parsing
//! - [ ] Multi-line input
//! - [ ] Modern scripting language
//!     - [ ] Macros
//!     - [ ] Types
//!     - [ ] Higher-order functions
//!     - [ ] Threading?
//! - [ ] Obfuscated strings (`!'password'!`)
//! - [ ] mosh like remote session support
//! - [ ] Smart history, sync'd across devices
//! - [ ] Pipe old commands without rerunning
//! - [ ] Package manager
//! - Sane defaults
//! - Fast
//!
//!
//! ## [POSIX Reference][posix-ref]
//!
//! See the following sections for building the POSIX `sh` compliant program
//! language, and interactive terminal based REPL. While this mainly defines the
//! [`posix`][program::posix] module, there are a lot of common concepts to all
//! shells here.
//!
//! - 3§2 Shell Command Language
//!     - 10.2 Shell Grammar Rules
//! - 2§2.5 Standard I/O Streams
//! - 3§1.6 Built-In Utilities
//! - 3§1.4 Utility Description Defaults
//! - 2§2.3 Error Numbers
//! - 1§11 General Terminal Interface
//! - 2§2.4 Signal Concepts
//!
//!
//! ## Implementation
//!
//! This shell will be written in Rust with minimal dependencies. Notably
//! `termios` and `libc` will likely be used. The parsing library will be
//! `lalrpop`, which should support the syntax we want somewhat easily, though
//! grammar's in general can be a tricky beast.
//!
//! We will want to create a few internal modules for the shell.
//!
//! **This design is subject to change.**
//!
//! - `process` - sub-process execution management.
//! - `program` - parser and interpreter for the syntax of the shell.
//!     - `posix` - POSIX (`sh`-like) syntax.
//!     - `modern` - Modified syntax for supporting "modern" features, like lambdas.
//! - `repl` - syntax aware, read eval print loop for an underlying terminal.
//!     - `history` - records previous execution to a shared DB.
//!     - `completion` - searches for autocompletions based on partial syntax.
//!         - `bash` - bash completion support.
//!         - `zsh` - zsh completion support.
//!         - `parse` - dynamic completion generation, from `man` for example.
//!     - `sync` - remote session and DB synchronization.
//! - `invocation` - loading for `.ourshrc` and others.
//! - `package` - simplistic package manager support (builtin function).
//!
//!
//! [documentation]: https://nixpulvis.com/oursh/oursh
//! [rustup]: https://github.com/rust-lang-nursery/rustup.rs
//! [posix-ref]: http://pubs.opengroup.org/onlinepubs/9699919799/
#![feature(box_patterns)]

extern crate nix;
extern crate pwd;
extern crate termion;

#[macro_use]
extern crate lalrpop_util;

/// Print debug information to stderr.
///
/// ### Examples
///
/// ```
/// use oursh::debug;
///
/// debug!(1 + 2);
///
/// let msg = "because.";
/// debug!("why!? {}", msg);
/// ```
#[macro_export]
macro_rules! debug {
    ($e:expr) => {
        if let Ok(_level) = ::std::env::var("LOG") {
            eprintln!("{:#?}", $e);
        }
    };
    ($format:expr, $($e:expr),*) => {
        if let Ok(_level) = ::std::env::var("LOG") {
            eprintln!($format, $($e),*);
        }
    };
}

pub mod invocation;
pub mod process;
pub mod program;

pub mod repl;


#[macro_use]
#[cfg(test)]
extern crate assert_matches;

#[cfg(test)]
mod tests {
    // This just tests syntax.
    #[test]
    fn debug_macro() {
        debug!(1);
        debug!(1 + 2);
        debug!("addition: {}", 1 + 2);
        debug!("{}", vec![1,2,3,4][2]);
        debug!("{} = {} * {}", 15, 3, 5);
    }
}
