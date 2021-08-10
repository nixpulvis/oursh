//! The shell language (often called `sh`) at the heart of the most popular
//! shells.
//!
//! While shells typically implement many extensions to the POSIX standard
//! we'll be implementing only the most basic set of functionality and
//! offloading all extensions to the `modern` language.
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
//! There are more than enough examples of `sh` scripts out there, but here is
//! a collection of examples tested in **this** shell's implementation of the
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
//! Variables are loaded from the environment (often simply called `ENV`) as
//! well. For more information on the enviroment read
//! [section 8.1](http://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap08.html).
//!
//! ```sh
//! echo $PATH
//! echo $TERM
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

use std::{
    ffi::CString,
    io::{Write, BufRead},
    process::{self, Stdio},
    fs::File,
    os::unix::io::IntoRawFd,
    env::{var, set_var}
};
use lalrpop_util::ParseError;
use nix::{
    sys::wait::WaitStatus,
    unistd::Pid,
};
use dirs::home_dir;
use crate::{
    process::{ProcessGroup, Process, Jobs},
    program::{Result, Error, IO},
};
use self::ast::{Assignment, Redirect};

#[cfg(feature = "shebang-block")]
use {
    std::io,
    std::process::ExitStatus,
    std::fs,
    std::os::unix::fs::PermissionsExt,
    self::ast::Interpreter,
};

// Re-exports.
pub use self::ast::Program;
pub use self::ast::Command;
pub use self::builtin::Builtin;

/// The syntax and semantics of a single POSIX command.
///
/// ```
/// use std::io::Read;
/// use oursh::program::Program as ProgramTrait;
/// use oursh::program::posix::ast::Program;
///
/// assert!(Program::parse(b"ls" as &[u8]).is_ok());
/// ```
impl super::Program for Program {
    type Command = Command;

    fn parse<R: BufRead>(mut reader: R) -> Result<Self> {
        let mut string = String::new();
        if reader.read_to_string(&mut string).is_err() {
            return Err(Error::Read);
        }

        let lexer = lex::Lexer::new(&string);
        let parser = parse::ProgramParser::new();
        match parser.parse(&string, lexer) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                match e {
                    ParseError::InvalidToken { location } => {
                        eprintln!("invalid token found at {}", location);
                    },
                    ParseError::UnrecognizedToken { token, expected } => {
                        let (s, t, e) = token;
                        eprintln!("unexpected token {:?} found at {}-{}, expecting one of: {}",
                                  t, s, e, expected.join(", "));
                    },
                    ParseError::UnrecognizedEOF { location, expected }=> {
                        eprintln!("unexpected EOF found at {}, expecting one of: {}",
                                  location, expected.join(", "));
                    }
                    ParseError::ExtraToken { token: (i, t, _) } => {
                        eprintln!("extra token {:?} found at {}", t, i);
                    }
                    ParseError::User { error } => {
                        let lex::Error::UnrecognizedChar(s, c, e) = error;
                        eprintln!("unexpected character {} found at {}-{}", c, s, e);
                    },
                }
                Err(Error::Parse)
            }
        }
    }

    fn commands(&self) -> &[Box<Self::Command>] {
        &self.0[..]
    }
}

// TODO: lazy_static.
// const BUILTINS: HashMap<&'static str, &'static Builtin> = HashMap::new(...);

// The semantics of a single POSIX command.
impl super::Command for Command {}

impl super::Run for Command {
    fn run(&self, background: bool, mut io: IO, jobs: &mut Jobs) -> Result<WaitStatus> {
        #[allow(unreachable_patterns)]
        match *self {
            Command::Simple(ref assignments, ref words, ref redirects) => {
                for Assignment(name, value) in assignments {
                    set_var(name, expand_vars(value));
                }

                for r in redirects {
                    match r {
                        Redirect::RW { n, filename, .. } => {
                            let file = File::with_options()
                                            .create(true)
                                            .read(true)
                                            .write(true)
                                            .open(filename).unwrap();
                            let fd = file.into_raw_fd();
                            io.0[*n as usize] = fd;
                        },
                        Redirect::Read { n, filename, .. } => {
                            let file = File::with_options()
                                            .read(true)
                                            .write(false)
                                            .open(filename).unwrap();
                            let fd = file.into_raw_fd();
                            io.0[*n as usize] = fd;
                        },
                        Redirect::Write { n, filename, append, .. } => {
                            // TODO: Clobber
                            let file = File::with_options()
                                            .create(true)
                                            .read(false)
                                            .write(true)
                                            .append(*append)
                                            .open(filename).unwrap();
                            let fd = file.into_raw_fd();
                            io.0[*n as usize] = fd;
                        },
                    };
                }

                // resolve: home and variables
                // sh-5.1$ FOO=~
                // sh-5.1$ echo $FOO
                // /home/nixpulvis
                let argv: Vec<CString> = words.iter().map(|word| {
                    CString::new(&expand_home(&expand_vars(&word.0)) as &str)
                        .expect("error in word UTF-8")
                }).collect();

                if let Some(command) = argv.clone().first() {
                    match command.to_string_lossy().as_ref() {
                        // TODO: IO for builtins.
                        ":"    => builtin::Null::run(argv, jobs),
                        "exit" => builtin::Exit::run(argv, jobs),
                        "export" => builtin::Export::run(argv, jobs),
                        "cd"   => builtin::Cd::run(argv, jobs),
                        "jobs" => builtin::Jobs::run(argv, jobs),
                        _ => {
                            let id = (jobs.borrow().len() + 1).to_string();
                            let mut job = Process::new(argv);
                            if background {
                                let status = job.fork(io).map_err(|_| Error::Runtime);
                                eprintln!("[{}]\t{}", id, job.pid());
                                jobs.borrow_mut().push((id, ProcessGroup(job)));
                                status
                            } else {
                                job.fork_and_wait(io)
                                   .map_err(|_| Error::Runtime)
                            }
                        },
                    }
                } else {
                    Ok(WaitStatus::Exited(Pid::this(), 0))
                }
            },
            // { sleep 3; date; }&
            // { sleep 3; date; }& ls
            Command::Compound(ref commands) => {
                // TODO: Need a way to run a set of commands as one in the background. Kinda like a
                // subshell. For now we just run them both as background as needed.
                let mut last = WaitStatus::Exited(Pid::this(), 0);
                for command in commands.iter() {
                    last = command.run(background, io, jobs)?;
                }
                Ok(last)
            },
            Command::Not(ref command) => {
                match command.run(false, io, jobs) {
                    Ok(WaitStatus::Exited(p, c)) => {
                        Ok(WaitStatus::Exited(p, (c == 0) as i32))
                    }
                    Ok(s) => Ok(s),
                    Err(_) => Err(Error::Runtime),
                }
            },
            Command::And(ref left, ref right) => {
                match left.run(false, io, jobs) {
                    Ok(WaitStatus::Exited(_, c)) if c == 0 => {
                        right.run(false, io, jobs).map_err(|_| Error::Runtime)
                    },
                    Ok(s) => Ok(s),
                    Err(_) => Err(Error::Runtime),
                }
            },
            Command::Or(ref left, ref right) => {
                match left.run(false, io, jobs) {
                    Ok(WaitStatus::Exited(_, c)) if c != 0 => {
                        right.run(false, io, jobs).map_err(|_| Error::Runtime)
                    },
                    Ok(s) => Ok(s),
                    Err(_) => Err(Error::Runtime),
                }
            },
            Command::Subshell(ref program) => {
                // TODO #4: Run in a *subshell* ffs.
                program.run(false, io, jobs)
            },
            Command::Pipeline(ref left, ref right) => {
                // TODO: This is obviously a temporary hack.
                if let box Command::Simple(_assigns, lwords, _redirs) = left {
                    let child = process::Command::new(&lwords[0].0)
                        .args(lwords.iter().skip(1).map(|w| &w.0))
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("error swawning pipeline process");

                    let output = child.wait_with_output()
                        .expect("error reading stdout");

                    if let box Command::Simple(_assigns, rwords, _redirs) = right {
                        let mut child = process::Command::new(&rwords[0].0)
                            .args(rwords.iter().skip(1).map(|w| &w.0))
                            .stdin(Stdio::piped())
                            .spawn()
                            .expect("error swawning pipeline process");

                        {
                            let stdin = child.stdin.as_mut()
                                .expect("error opening stdin");
                            stdin.write_all(&output.stdout)
                                .expect("error writing to stdin");
                        }

                        child.wait()
                            .expect("error waiting for piped command");
                    }
                }
                Ok(WaitStatus::Exited(Pid::this(), 0))
            },
            Command::Background(ref command) => {
                command.run(true, io, jobs)
            },
            #[cfg(feature = "shebang-block")]
            Command::Lang(ref interpreter, ref text) => {
                fn bridge(interpreter: &str, text: &str) -> io::Result<ExitStatus> {
                    // TODO: Even for the Shebang interpretor, we shouldn't
                    // create files like this.
                    // XXX: Length is the worlds worst hash function.
                    let bridgefile = format!("/tmp/.oursh_bridge-{}", text.len());
                    {
                        // TODO: Use our job interface without creating any
                        // fucking files... The shebang isn't even a real
                        // POSIX standard.
                        let mut file = File::create(&bridgefile)?;
                        let mut interpreter = interpreter.chars()
                                                         .map(|c| c as u8)
                                                         .collect::<Vec<u8>>();
                        interpreter.insert(0, '!' as u8);
                        interpreter.insert(0, '#' as u8);
                        file.write_all(&interpreter)?;
                        file.write_all(b"\n")?;
                        let text = text.chars()
                                       .map(|c| c as u8)
                                       .collect::<Vec<u8>>();
                        file.write_all(&text)?;

                        let mut perms = fs::metadata(&bridgefile)?.permissions();
                        perms.set_mode(0o777);
                        fs::set_permissions(&bridgefile, perms)?;
                    }
                    // TODO #4: Suspend and restore raw mode.
                    // TODO: Use our jobs, this will allow WaitStatus' too.
                    process::Command::new(&bridgefile).spawn()?.wait()
                }
                // TODO: Pass text off to another parser.
                match interpreter {
                    Interpreter::Primary => {
                        unimplemented!()
                    }
                    Interpreter::Alternate => {
                        bridge("/bin/sh", text).map_err(|_| Error::Read)?;
                        Ok(WaitStatus::Exited(Pid::this(), 0))
                    },
                    Interpreter::HashLang(ref language) => {
                        let interpreter = match language.as_str() {
                            "ruby"   => "/usr/bin/env ruby",
                            "node"   => "/usr/bin/env node",
                            "python" => "/usr/bin/env python",
                            "racket" => "/usr/bin/env racket",
                            _        => return Err(Error::Read),
                        };
                        bridge(interpreter, text).map_err(|_| Error::Read)?;
                        Ok(WaitStatus::Exited(Pid::this(), 0))
                    },
                    Interpreter::Shebang(ref interpreter) => {
                        bridge(interpreter, text).map_err(|_| Error::Read)?;
                        Ok(WaitStatus::Exited(Pid::this(), 0))
                    },
                }
            },
            #[cfg(not(feature = "shebang-block"))]
            Command::Lang(_,_) => {
                unimplemented!();
            },
        }
    }
}
fn expand_home(word: &str) -> String {
    if word.starts_with("~") {
        if let Some(path) = home_dir() {
            format!("{}{}", &path.to_str().expect("error: home not set"),
                            &word[1..])
        } else {
            "~".into()
        }
    } else {
        word.into()
    }

}

// "$" => "$"
// "$ " => "$ "
// "$USER" => "nixpulvis"
fn expand_vars(string: &str) -> String {
    let mut result = String::new();
    let mut variable = String::new();
    let mut variable_start = -1;
    for (i, c) in string.char_indices() {
        if c == '$' || c == ' ' {
            // It's not possible for these to be vars.
            if variable.is_empty() {
                result.push(c);
            } else {
                result += &var(&variable).unwrap_or("".into());
            }
            variable.clear();
            variable_start = -1;
        }

        if c == '$' {
            variable_start = i as i32;
        } else if c == ' ' {
            variable_start = -1;
        } else if c == '@' || c == ':' {
            result += &var(&variable).unwrap_or("".into());
            variable.clear();
            variable_start = -1;
            result.push(c);
        } else if variable_start > -1 {
            if variable.is_empty() {
                result.pop();  // remove the leading '$'.
            }
            variable.push(c);
        } else {
            result.push(c);
        }
    }
    result += &var(&variable).unwrap_or("".into());
    result
}

// Builtin functions for the POSIX language, like `exit` and `cd`.
pub mod builtin;

// The POSIX AST data structures and helper functions.
pub mod ast;

// The custom LALRPOP lexer.
pub mod lex;

// Following with the skiing analogy, the code inside here is black level.
// Many of the issues in a grammar rule cause conflicts in seemingly unrelated
// rules. Some issues are known to be harder to solve, and while LALRPOP does
// a fantasic job of helping, it's not perfect. Avoid the rocks, trees, and
// enjoy.
//
// The code for this module is located in `src/program/posix/mod.lalrpop`.
lalrpop_mod!(
    /// LALRPOP generated parser module.
    pub parse, "/program/posix.rs");
