//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use nix::unistd;
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

    pub fn nixpulvis_style(self) -> Self {
        Prompt(format!("{}{}{}@{}{}{}:{}{}{}{}$ ",
            color::Fg(color::Red),
            "nixpulvis",
            color::Fg(color::Reset),
            color::Fg(color::Blue),
            "masva",
            color::Fg(color::Reset),
            color::Fg(color::Green),
            "~/Code/oursh",
            color::Fg(color::Reset),
            ""))
    }

    pub fn long_style(self) -> Self {
        let mut buf = [0u8; 64];
        let hostname_cstr = unistd::gethostname(&mut buf)
            .expect("error getting hostname");
        let hostname = hostname_cstr.to_str()
            .expect("hostname wasn't valid UTF-8");
        Prompt(format!("{}{} {} $ {} {} {} {} {}{} ",
            style::Invert,
            color::Fg(color::Green),
            hostname,
            color::Fg(color::Yellow),
            color::Fg(color::Red),
            color::Fg(color::Magenta),
            color::Fg(color::Cyan),
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
