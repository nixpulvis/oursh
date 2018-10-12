//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::Write;
use termion::color;

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Prompt {
    pub fn new() -> Self {
        let red = color::Fg(color::Red);
        let reset = color::Fg(color::Reset);
        Prompt(format!("{}oursh{} $ ", red, reset))
    }

    pub fn display(&self, stdout: &mut impl Write) {
        write!(stdout, "{}", String::from(self)).unwrap();
    }
}

impl<'a> From<&'a Prompt> for String {
    fn from(prompt: &'a Prompt) -> Self {
        prompt.0.clone()
    }
}
