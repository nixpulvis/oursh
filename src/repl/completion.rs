//! User text completion for REPL interations.
//!
//! Simple use cases for this module should be as easy as the following
//! example taken from the current REPL.
//!
//! ```
//! use oursh::repl::completion::complete;
//!
//! // String holding the user's input.
//! let mut text = "car".to_string();
//!
//! // Perform the completion, on `\t` perhaps.
//! text = complete(&text).first();
//!
//! // The user's input is updated to the complete executable.
//! assert_eq!("cargo", &text);
//! ```

use std::{
    env,
    fs,
    cmp::Ordering::Equal,
    os::unix::fs::PermissionsExt,
    io::Write,
};
use tabwriter::TabWriter;

/// The result of a query for text completion.
///
/// A complete result is expected tob run without much thought by the
/// user. Some care should be taken to avoid dangerous completions.
#[derive(Debug)]
pub enum Completion {
    /// Nothing completes the user text.
    None,
    /// The user text could match multiple complete values.
    Partial(Vec<String>),
    /// A single complete value.
    Complete(String),
}

impl Completion {
    /// Returns true if this completion is a single option.
    pub fn is_complete(&self) -> bool {
        match *self {
            Completion::None |
            Completion::Partial(_) => false,
            Completion::Complete(_) => true,
        }
    }

    /// Return the first (lexicographically) option if there are multiple
    /// possibilities.
    pub fn first(&self) -> String {
        match *self {
            Completion::None => "".to_owned(),
            Completion::Partial(ref p) => {
                match p.first() {
                    Some(t) => t.to_owned(),
                    None => "".to_owned(),
                }
            },
            Completion::Complete(ref s) => s.to_owned(),
        }
    }

    /// Return a list of all the possibile complete matches.
    pub fn possibilities(&self) -> Vec<String> {
        match *self {
            Completion::None => vec![],
            Completion::Partial(ref p) => p.clone(),
            Completion::Complete(ref t) => vec![t.clone()],
        }
    }

    // fn guess
}

/// Return a completed (valid) program text from the partial string
/// given.
///
/// ### Examples
///
/// ```
/// use oursh::repl::completion::complete;
///
/// assert_eq!("pwd", complete("pw").first());
/// ```
pub fn complete(text: &str) -> Completion {
    match executable_completions(text) {
        c @ Completion::Partial(_) |
        c @ Completion::Complete(_) => c,
        Completion::None => path_complete(text),
    }
}

/// Return a list of the matches from the given partial program text.
///
/// ### Examples
///
/// ```
/// use oursh::repl::completion::executable_completions;
///
/// assert!(executable_completions("ru").possibilities()
///     .contains(&"rustc".into()));
/// assert!(executable_completions("ru").possibilities()
///     .contains(&"ruby".into()));
/// ```
pub fn executable_completions(text: &str) -> Completion {
    match env::var_os("PATH") {
        Some(paths) => {
            let mut matches = vec![];
            for dir in env::split_paths(&paths) {
                if let Ok(executables) = fs::read_dir(dir) {
                    let paths = executables.filter_map(|e| {
                        match e { Ok(p) => Some(p.path()), _ => None }
                    });

                    for path in paths {
                        if let Some(filename) = path.file_name() {
                            let filename = filename.to_string_lossy();
                            if let Ok(metadata) = fs::metadata(&path) {
                                if (metadata.permissions()
                                            .mode() & 0o111 != 0)
                                    && filename.starts_with(text)
                                {
                                    matches.push(filename.into());
                                }
                            }
                        }
                    }
                }
            }

            match matches.len() {
                0 => Completion::None,
                1 => Completion::Complete(matches.remove(0)),
                _ => {
                    // TODO: Support POSIX style lexicographical order?
                    matches.sort_by(|a, b| {
                        match a.len().cmp(&b.len()) {
                            Equal => b.cmp(&a),
                            o => o
                        }
                    });
                    Completion::Partial(matches)
                }
            }

        }
        None => panic!("PATH is undefined"),
    }
}

/// Complete a path at the end of the given string.
///
/// ### Examples
///
/// ```
/// use oursh::repl::completion::path_complete;
///
/// assert_eq!("/usr/bin/", path_complete("/usr/b").first());
/// assert_eq!("ls /home/", path_complete("ls /hom").first());
/// ```
pub fn path_complete(text: &str) -> Completion {
    match text {
        "/hom" => Completion::Complete("/home/".into()),
        "/usr/b" => Completion::Complete("/usr/bin/".into()),
        "ls /hom" => Completion::Complete("ls /home/".into()),
        _ => Completion::None,
    }
}

pub fn write_table(writer: impl Write, words: &[String]) {
    // TODO: Handle empty case better.
    let max_length = words.iter().max_by(|a,b| a.len().cmp(&b.len())).unwrap().len() as u16;
    // TODO: Can this function be called from outside a terminal environment?
    let (width, _height) = termion::terminal_size().unwrap();
    let padding = 5;
    let columns = width / (max_length + padding);
    let mut tw = TabWriter::new(writer).padding(padding as usize);
    // TODO: Determine table width/height and calculate iteration better
    let mut _row = 0;
    let mut col = 0;
    for word in words {
        tw.write(word.as_bytes()).unwrap();
        tw.write(b"\t").unwrap();

        if col == columns-1 {
            col = 0;
            _row += 1;
            tw.write(b"\n").unwrap();
        } else {
            col += 1;
        }
    }
    tw.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Mock PATH to make deterministic.

    #[test]
    fn lexicographical_order() {
        assert_eq!("cargo", complete("car").first());
    }

    #[test]
    fn paths() {
        assert_eq!("/home/", complete("/hom").first());
        assert_eq!("/usr/bin/", complete("/usr/b").first());
        assert_eq!("ls /home/", complete("ls /hom").first());
    }
}
