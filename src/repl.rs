//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::Write;
use termion::{style, color};

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Prompt {
    pub const DEFAULT_FORMAT: &'static str = "$ ";

    pub fn new() -> Self {
        Prompt(format!("{}", Self::DEFAULT_FORMAT))
    }

    pub fn long_style(self) -> Self {
        Prompt(format!("{}{} oursh $ {} {} {}{} ",
            color::Fg(color::Red),
            style::Invert,
            color::Fg(color::Blue),
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            style::Reset))
    }

    pub fn short_style(self) -> Self {
        Prompt(format!("{}{}${}{} ",
            color::Fg(color::Red),
            style::Invert,
            color::Fg(color::Reset),
            style::Reset))
    }

    pub fn display(&self, stdout: &mut impl Write) {
        write!(stdout, "{}", self.0).unwrap();
        stdout.flush().unwrap();
    }
}
