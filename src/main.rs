extern crate nix;
extern crate termion;
extern crate oursh;

use std::io::{self, Read, Write, Stdout};
use nix::Result;
use nix::sys::signal;
use nix::libc::c_int;
// use termion::raw::IntoRawMode;
use oursh::job::Job;
use oursh::program::Program;

// Our shell, for the greater good. Ready and waiting.
fn main() {
    // Standard input file descriptor (0), used for user input from the user
    // of the shell.
    let mut stdin = io::stdin();

    // Standard output file descriptor (1), used to display program output to
    // the user of the shell.
    let stdout = io::stdout();

    // STDIN, input buffer, used to `read` text into for further processing.
    // TODO: Full fledged parser will be neato.
    let mut input: [u8; 24];

    // // Block exits via `SIGINT`, generally triggered with ctrl-c.
    // trap_sigint().expect("error trapping sigint");

    loop {
        // XXX: Blindly drop the contents of input, again this will be better
        // with a real parser.
        input = [0; 24];

        // Print a boring static prompt.
        prompt(&stdout);

        loop {
            // TODO: Enable raw access to STDIN, so we can read as the user
            // types. By default the underlying file is line buffered. This
            // will allow us to process history, syntax, and more!

            // Read what's avalible to us.
            stdin.read(&mut input).expect("error reading STDIN");

            // Once we've read a complete "program" (§2.10.2) we handle it,
            // until then we keep reading. Once we have a proper parser this
            // wont look like a huge hack.
            let vec: Vec<&[u8]> = input.splitn(2, |b| *b == '\n' as u8).collect();
            match &vec[..] {
                [line, _rest] => {
                    let program = Program::parse(*line);
                    Job::new(&program).run();
                    break
                }
                _ => {},
            }
        }
    }
}

fn prompt(stdout: &Stdout) {
    let red = termion::color::Fg(termion::color::Red);
    let reset = termion::color::Fg(termion::color::Reset);
    print!("{}oursh{} $ ", red, reset);
    stdout.lock().flush().expect("error flushing STDOUT");
}

fn trap_sigint() -> Result<signal::SigAction>  {
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
    print!("{}\r", termion::clear::CurrentLine);
    prompt(&stdout);
    trap_sigint().expect("error trapping sigint");
}

