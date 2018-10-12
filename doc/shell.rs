use std::io::{self, Read, Write};
use std::process::{Command, Output};
use std::str;

// Our shell, for the greater good. Ready and waiting.
fn main() {
    // Standard input file descriptor (0), used for user input from the user
    // of the shell.
    let mut stdin = io::stdin();

    // Standard output file descriptor (1), used to display program output to
    // the user of the shell.
    let mut stdout = io::stdout();

    // STDIN, input buffer, used to `read` text into for further processing.
    // TODO: Full fledged parser will be neato.
    let mut input = [0; 24];

    loop {
        // XXX: Blindly drop the contents of input, again this will be better
        // with a real parser.
        input = [0; 24];

        // Print a boring static prompt.
        print!("oursh> ");
        stdout.lock().flush();

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
                [line, rest] => {
                    let program = str::from_utf8(&line).expect("error reading utf8");
                    let mut output = handle_program(program);
                    stdout.write(&output.stdout).expect("error writing to STDOUT");
                    break
                }
                _ => {},
            }
        }
    }
}

// Obviously very wrong. Most notably this blocks until the command completes.
fn handle_program(program: &str) -> Output {
    Command::new(program)
        .output()
        .expect("error executing program")
}

