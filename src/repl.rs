//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::{self, Read, Write};
use nix::Result;
use nix::sys::signal;
use nix::libc::c_int;
use termion::raw::IntoRawMode;
use termion::{color, clear};

/// For now, we simple export the function from termion to allow main to
/// decide when to use it without linking termion directly itself ;)
pub use termion::is_tty;

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

pub fn trap_sigint() -> Result<signal::SigAction>  {
    let action = signal::SigAction::new(signal::SigHandler::Handler(handle_ctrl_c),
                                        signal::SaFlags::all(),
                                        signal::SigSet::all());
    unsafe {
        signal::sigaction(signal::SIGINT, &action)
    }
}

extern fn handle_ctrl_c(_: c_int) {
    use std::process::exit;
    exit(1);
    let mut stdout = io::stdout();

    // Clear
    print!("{}\r", clear::CurrentLine);
    Prompt::new().display(&mut stdout);
    trap_sigint().expect("error trapping sigint");
}
