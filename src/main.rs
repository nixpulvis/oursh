extern crate oursh;

use std::io::{self, Read};
use std::process::exit;
use oursh::job::Job;
use oursh::program::Program;
use oursh::repl;

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

    // Block exits via `SIGINT`, generally triggered with ctrl-c.
    repl::trap_sigint().expect("error trapping sigint");

    loop {
        // XXX: Blindly drop the contents of input, again this will be better
        // with a real parser.
        input = [0; 24];

        // Print a boring static prompt.
        repl::prompt(&stdout);

        loop {
            // TODO: Enable raw access to STDIN, so we can read as the user
            // types. By default the underlying file is line buffered. This
            // will allow us to process history, syntax, and more!

            // Read what's avalible to us.
            if stdin.read(&mut input).expect("error reading STDIN") == 0 {
                exit(0);
            }

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
