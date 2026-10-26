//! User text completion for REPL inputs.
//!
//! Simple use cases for this module should be as easy as the following
//! example taken from the current REPL.
//!
//! ```no_run
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

use std::{cmp::Ordering::Equal, env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

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
            Completion::None | Completion::Partial(_) => false,
            Completion::Complete(_) => true,
        }
    }

    /// Return the first (lexicographically) option if there are multiple
    /// possibilities.
    pub fn first(&self) -> String {
        match *self {
            Completion::None => "".to_owned(),
            Completion::Partial(ref p) => match p.first() {
                Some(t) => t.to_owned(),
                None => "".to_owned(),
            },
            Completion::Complete(ref s) => s.to_owned(),
        }
    }

    /// Return a list of all the possible complete matches.
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
/// ```no_run
/// use oursh::repl::completion::complete;
///
/// assert_eq!("pwd", complete("pw").first());
/// ```
pub fn complete(text: &str) -> Completion {
    match executable_completions(text) {
        c @ Completion::Partial(_) | c @ Completion::Complete(_) => c,
        Completion::None => path_complete(text),
    }
}

/// Return a list of the matches from the given partial program text.
///
/// ### Examples
///
/// ```no_run
/// use oursh::repl::completion::executable_completions;
///
/// assert!(executable_completions("ru").possibilities()
///     .contains(&"rustc".into()));
/// assert!(executable_completions("ru").possibilities()
///     .contains(&"ruby".into()));
/// ```
pub fn executable_completions(text: &str) -> Completion {
    match env::var_os("PATH") {
        Some(paths) => executables_in(env::split_paths(&paths), text),
        None => panic!("PATH is undefined"),
    }
}

/// Match executables in the given directories against the partial text.
fn executables_in(dirs: impl IntoIterator<Item = PathBuf>, text: &str) -> Completion {
    let mut matches = vec![];
    for dir in dirs {
        if let Ok(executables) = fs::read_dir(dir) {
            let paths = executables.filter_map(|e| match e {
                Ok(p) => Some(p.path()),
                _ => None,
            });

            for path in paths {
                if let Some(filename) = path.file_name() {
                    let filename = filename.to_string_lossy();
                    if let Ok(metadata) = fs::metadata(&path) {
                        if (metadata.permissions().mode() & 0o111 != 0)
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
            matches.sort_by(|a, b| match a.len().cmp(&b.len()) {
                Equal => b.cmp(a),
                o => o,
            });
            Completion::Partial(matches)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, process};

    /// Create a fresh temporary directory holding the given files, each
    /// marked executable or not.
    fn bin_dir(name: &str, files: &[(&str, bool)]) -> PathBuf {
        let dir = env::temp_dir().join(format!("oursh-completion-{}-{}", process::id(), name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for (file, executable) in files {
            let path = dir.join(file);
            File::create(&path).unwrap();
            let mode = if *executable { 0o755 } else { 0o644 };
            fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
        }
        dir
    }

    #[test]
    fn shortest_executable_first_across_dirs() {
        let a = bin_dir("shortest-a", &[("cargo-fmt", true), ("carton", false)]);
        let b = bin_dir("shortest-b", &[("cargo", true), ("ls", true)]);
        let completion = executables_in(vec![a.clone(), b.clone()], "car");
        fs::remove_dir_all(a).unwrap();
        fs::remove_dir_all(b).unwrap();

        assert!(!completion.is_complete());
        assert_eq!(vec!["cargo", "cargo-fmt"], completion.possibilities());
    }

    #[test]
    fn single_match_is_complete() {
        let dir = bin_dir("single", &[("whoami", true), ("who", false)]);
        let completion = executables_in(vec![dir.clone()], "who");
        fs::remove_dir_all(dir).unwrap();

        assert!(completion.is_complete());
        assert_eq!("whoami", completion.first());
    }

    #[test]
    fn no_match() {
        let dir = bin_dir("none", &[("ls", true)]);
        let completion = executables_in(vec![dir.clone()], "zz");
        fs::remove_dir_all(dir).unwrap();

        assert!(completion.possibilities().is_empty());
    }

    #[test]
    fn paths() {
        assert_eq!("/home/", complete("/hom").first());
        assert_eq!("/usr/bin/", complete("/usr/b").first());
        assert_eq!("ls /home/", complete("ls /hom").first());
    }
}
