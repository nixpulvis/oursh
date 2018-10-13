//! This shell should be both POSIX compatible and yet modern and exciting.
//! Fancy features should not be prevented by POSIX compatibility. This will
//! effect the design of the shell.
//!
//! The name of the shell is `oursh` which is both somewhat unique, and
//! memorable.  It's also a nice name to play with pseudo-satirical themes...
//! right comrade?  It's short (ish) and sneakily fits `rs` in it, which is the
//! extension of Rust programs, the language this will be written in.
//!
//!
//! ## Features
//!
//! - Basic command REPL
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
//! While this project is in early stages, there are no OS packages to use.
//! However, you can compile and run directly from source easily. Just ensure
//! you have [`rustup`][rustup] installed.
//!
//! ```sh
//! cargo run
//! ```
//!
//!
//! ## Previous Work
//!
//! I've been using [`fish`][fish] as my main shell for a few years now. Fish
//! inspires a lot of the modern syntax.
//!
//! POSIX compatibility comes from my desire to use this shell as my `chsh -s
//! ...` shell on [Arch Linux][arch]. See the full POSIX reference for more
//! information.
//!
//! I've built and wrote a few things about shells before:
//!
//! - [`rush`][rush] A glorified homework assignment for computer architecture.
//! - [`myshell.py`][myshell.py] My submission for computer organization a8.
//! - [Building a Shell - Part 1][basp1] Start of this project.
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
//! [documentation]: https://nixpulvis.com/oursh/oursh
//! [rustup]: https://github.com/rust-lang-nursery/rustup.rs
//! [posix]: http://pubs.opengroup.org/onlinepubs/9699919799/
//! [termios]: https://crates.io/crates/termios
//! [libc]: https://crates.io/crates/libc
//! [fish]: https://github.com/fish-shell/fish-shell
//! [arch]: https://www.archlinux.org/
//! [rush]: https://github.com/nixpulvis/rush
//! [myshell.py]: /doc/cs2600-a8-myshell.py
//! [basp1]: https://nixpulvis.com/ramblings/2018-07-11-building-a-shell-part-1
#![feature(box_syntax, box_patterns)]

extern crate nix;
extern crate pwd;
extern crate termion;

#[macro_use]
extern crate lalrpop_util;

pub mod job;
pub mod program;
pub mod repl;


#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
