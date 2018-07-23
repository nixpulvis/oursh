use std::io::{self, Stdout, Write};
use nix::Result;
use nix::sys::signal;
use nix::libc::c_int;
// use termion::raw::IntoRawMode;
use termion::{color, clear};

pub fn prompt(stdout: &Stdout) {
    let red = color::Fg(color::Red);
    let reset = color::Fg(color::Reset);
    print!("{}oursh{} $ ", red, reset);
    stdout.lock().flush().expect("error flushing STDOUT");
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
    let stdout = io::stdout();

    // Clear
    print!("{}\r", clear::CurrentLine);
    prompt(&stdout);
    trap_sigint().expect("error trapping sigint");
}
