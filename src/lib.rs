//! [![CV](https://github.com/nixpulvis/oursh/actions/workflows/cv.yml/badge.svg?branch=master&event=push)](https://github.com/nixpulvis/oursh/actions/workflows/cv.yml)
//! [![CI](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml)
//! [![Documentation](https://docs.rs/oursh/badge.svg)](https://docs.rs/oursh)
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
//! - POSIX compatibility
//! - Shebang block programs
//! - bash/zsh autocomplete compatibility
//! - `man` / `-h` / `--help` parsing
//! - Multi-line input
//! - Modern scripting language
//! - Obfuscated strings (`!'password'!`)
//! - mosh like remote session support
//! - Smart history, sync'd across devices
//! - Pipe old commands without rerunning
//! - Package manager
//! -  Sane defaults
//! - Fast
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
//! Some of the shebang language interoperation was inspired by my time with
//! the Northeastern University PL group, and generally from writing
//! [Racket][racket].  The beauty of of merging the UNIX style `#!...` with
//! Racket's `#lang ...` here is very exciting to me. I might just _have_ to
//! make a `{#lang ...}` shortcut for Racket!
//!
//! I've built and wrote a few things about shells before:
//!
//! - [`rush`][rush] A glorified homework assignment for computer architecture
//! - [`shell.py`][shell.py] My submission for computer organization a8
//! - [Building a Shell - Part 1][basp1] Start of this project
//! - [Building a Shell - Part 2][basp2] `program` module intro
//!
//!
//! ## [POSIX Reference][posix]
//!
//! See the following sections for building the POSIX `sh` compliant program
//! language, and interactive terminal based REPL. While this mainly defines
//! the `posix` module, there are a lot of common concepts to all shells here.
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
//! [lalrpop]: https://github.com/lalrpop/lalrpop
//! [fish]: https://github.com/fish-shell/fish-shell
//! [arch]: https://www.archlinux.org/
//! [racket]: https://racket-lang.org/
//! [rush]: https://github.com/nixpulvis/rush
//! [shell.py]: /doc/shell.py
//! [basp1]: https://nixpulvis.com/ramblings/2018-07-11-building-a-shell-part-1
//! [basp2]: https://nixpulvis.com/ramblings/2018-10-15-building-a-shell-part-2
#![feature(box_syntax, box_patterns, with_options)]
#![forbid(unsafe_code)]

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

pub mod process;
pub mod program;
// pub mod repl;


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
