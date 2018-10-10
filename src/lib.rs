//! This shell should be both POSIX compatible and yet modern and exciting. Fancy
//! features should not be prevented by POSIX compatibility. This will effect the
//! design of the shell.
//!
//! The name of the shell is `oursh` which is both somewhat unique, and memorable.
//! It's also a nice name to play with pseudo-satirical themes... right comrade?
//! It's short (ish) and sneakily fits `rs` in it, which is the extension of Rust
//! programs, the language this will be written in.
//!
//!
//! ## Features
//!
//! - POSIX compatibility w/ non-posix blocks (`{@lang ... }`)
//! - bash/zsh autocomplete compatibility
//! - `man` / `-h` / `--help` parsing
//! - Multi-line input
//! - Modern scripting language (types, higher-order functions, threading?, etc)
//! - obfuscated strings (`!'password'!`)
//! - mosh like remote session support
//! - Smart history, sync'd across devices
//! - Package manager
//! - Sane defaults
//! - Fast
//!
//! ## Usage
//!
//! While this project is in early stages, there are no OS packages to use. However,
//! you can compile and run directly from source easily.
//!
//! ```sh
//! cargo run
//! ```
//!
//!
//! ## [POSIX Reference][posix]
//!
//! See the following sections for building the POSIX `sh` compliant program
//! language, and interactive terminal based REPL.
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
//! [posix]: http://pubs.opengroup.org/onlinepubs/9699919799/

extern crate nix;
extern crate termion;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod job;
pub mod program;
pub mod repl;
