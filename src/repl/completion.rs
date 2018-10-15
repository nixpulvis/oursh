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
//! text = complete(&text);
//!
//! // The user's input is updated to the complete executable.
//! assert_eq!("cargo", &text);
//! ```

use std::cmp::Ordering::Equal;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

/// Return a completed (valid) program text from the partial string
/// given.
///
/// ### Examples
///
/// ```
/// use oursh::repl::completion::complete;
///
/// assert_eq!("ls", complete("l"));
/// assert_eq!("pwd", complete("pw"));
/// ```
pub fn complete(text: &str) -> String {
    let mut matches = executable_completions(text);
    matches.sort_by(|a, b| {
        match a.len().cmp(&b.len()) { Equal => b.cmp(&a), o => o }
    });
    matches.first().unwrap_or(&text.to_string()).clone()
    // path_complete(text)
}

/// Return a list of the matches from the given partial program text.
///
/// ### Examples
///
/// ```
/// use oursh::repl::completion::executable_completions;
///
/// assert!(executable_completions("ru").contains(&"rustc".into()));
/// assert!(executable_completions("ru").contains(&"ruby".into()));
/// ```
pub fn executable_completions(text: &str) -> Vec<String> {
    match env::var_os("PATH") {
        Some(paths) => {
            let mut matches = vec![];
            for dir in env::split_paths(&paths) {
                let executables = fs::read_dir(dir).unwrap();
                for path in executables.map(|e| e.unwrap().path()) {
                    let metadata = fs::metadata(&path);
                    let filename = path.file_name().unwrap().to_string_lossy();
                    if (metadata.unwrap().permissions().mode() & 0o111 != 0) &&
                        filename.starts_with(text)
                    {
                        matches.push(filename.into());
                    }

                }
            }
            matches
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
/// assert_eq!("/usr/bin/", path_complete("/usr/b"));
/// assert_eq!("ls /home/", path_complete("ls /hom"));
/// ```
pub fn path_complete(text: &str) -> String {
    text.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexicographical_order() {
        assert_eq!("cat", complete("ca"));
    }

    #[test]
    fn paths() {
        assert_eq!("/home/", complete("/hom"));
        assert_eq!("/usr/bin/", complete("/usr/b"));
        assert_eq!("ls /home/", complete("ls /hom"));
    }
}
