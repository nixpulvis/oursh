# oursh
[![Build Status](https://travis-ci.org/nixpulvis/oursh.svg?branch=master)](https://travis-ci.org/nixpulvis/oursh)

This shell should be both POSIX compatible and yet modern and exciting. Fancy
features should not be prevented by POSIX compatibility. This will effect the
design of the shell.

The name of the shell is `oursh` which is both somewhat unique, and memorable.
It's also a nice name to play with pseudo-satirical themes... right comrade?
It's short (ish) and sneakily fits `rs` in it, which is the extension of Rust
programs, the language this will be written in.

- [Documentation][documentation]

## Features

- POSIX compatibility w/ non-posix blocks (`{@lang ... }`)
- bash/zsh autocomplete compatibility
- `man` / `-h` / `--help` parsing
- Multi-line input
- Modern scripting language (types, higher-order functions, threading?, etc)
- obfuscated strings (`!'password'!`)
- mosh like remote session support
- Smart history, sync'd across devices
- Package manager
- Sane defaults
- Fast
- Pipe old commands without rerunning

## Setup

```sh
rustup update
```

## Usage

While this project is in early stages, there are no OS packages to use. However,
you can compile and run directly from source easily.

```sh
cargo run
```


## Building

```sh
cargo build
cargo build --release
```


## Testing

TODO

```sh
cargo test
```


## [POSIX Reference][posix]

See the following sections for building the POSIX `sh` compliant program
language, and interactive terminal based REPL.

- 3§2 Shell Command Language
    - 10.2 Shell Grammar Rules
- 2§2.5 Standard I/O Streams
- 3§1.6 Built-In Utilities
- 3§1.4 Utility Description Defaults
- 2§2.3 Error Numbers
- 1§11 General Terminal Interface
- 2§2.4 Signal Concepts


## Implementation

This shell will be written in Rust with minimal dependencies. Notably
[termios][termios] and [libc][libc] will likely be used. A crate for parsing
should also be decided on, but there are a few options and it's unclear at this
point what to use.

We will want to create a few internal modules for the shell.

**This design is highly subject to change.**

- `job` - sub-process execution management.
- `program` - parser and interpreter for the syntax of the shell.
    - `posix` - POSIX (`sh`-like) syntax.
    - `modern` - Modified syntax for supporting "modern" features, like lambdas.
- `repl` - syntax aware, read eval print loop for an underlying terminal.
    - `history` - records previous execution to a shared DB.
    - `completion` - searches for autocompletions based on partial syntax.
        - `bash` - bash completion support.
        - `zsh` - zsh completion support.
        - `parse` - dynamic completion generation, from `man` for example.
    - `sync` - remote session and DB synchronization.
- `config` - loading for `.ourshrc` and others.
- `package` - simplistic package manager support (builtin function).


[documentation]: https://nixpulvis.com/oursh/oursh
[posix]: http://pubs.opengroup.org/onlinepubs/9699919799/
[termios]: https://crates.io/crates/termios
[libc]: https://crates.io/crates/libc
