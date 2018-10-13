extern crate oursh;
extern crate termion;

use std::env;
use std::io::{self, Read, Write};
use oursh::program::{parse_primary, Program};
use oursh::repl;
use termion::is_tty;
use termion::raw::IntoRawMode;

// Our shell, for the greater good. Ready and waiting.
fn main() {
    // Standard input file descriptor (0), used for user input from the
    // user of the shell.
    let stdin = io::stdin();

    // Process text in raw mode style if we're attached to a tty.
    if is_tty(&stdin) {
        // Standard output file descriptor (1), used to display program output
        // to the user of the shell.
        let mut stdout = io::stdout().into_raw_mode()
            .expect("error opening raw mode");

        // Start a program running repl.
        repl::start(stdin, stdout, parse_and_run);
    } else {
        // Fill a string buffer from STDIN.
        let mut text = String::new();
        stdin.lock().read_to_string(&mut text).unwrap();

        // Run the program.
        parse_and_run(&text);
    }
}

fn parse_and_run(text: &String) {
    // Parse with the primary grammar and run each command in order.
    match parse_primary(text.as_bytes()) {
        Ok(program) => {
            // TODO: Proper arg parsing.
            if let Some(arg1) = env::args().nth(1) {
                if arg1 == "-v" || arg1 == "--verbose" {
                    println!("{:#?}", program);
                }
            }

            program.run()
                .expect(&format!("error running program: {:?}", program));
        },
        Err(()) => {
            println!("error parsing text: {}", text);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
