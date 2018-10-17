#![feature(alloc_system)]

extern crate docopt;
extern crate nix;
extern crate oursh;
extern crate termion;

use std::env;
use std::io::{self, Read};
use std::fs::File;
use docopt::{Docopt, ArgvMap, Value};
use oursh::program::{Result, parse_primary, Program};
use oursh::repl;
use termion::is_tty;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage: oursh [options] [<file>]

Options:
    -h --help     Show this screen.
    -# --ast      Print an AST of each program.
    -v --verbose  Print extra information.
";

// Our shell, for the greater good. Ready and waiting.
fn main() -> Result<()> {
    // Parse argv and exit the program with an error message if it fails.
    let args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    if let Some(Value::Plain(Some(ref filename))) = args.find("<file>") {
        let mut file = File::open(filename)
            .expect(&format!("error opening file: {}", filename));

        // Fill a string buffer from STDIN.
        let mut text= String::new();
        file.read_to_string(&mut text)
            .expect("error reading file");

        // Run the program.
        parse_and_run(&args)(&text)
    } else {
        // Standard input file descriptor (0), used for user input from the
        // user of the shell.
        let stdin = io::stdin();

        // Process text in raw mode style if we're attached to a tty.
        if is_tty(&stdin) {
            // Standard output file descriptor (1), used to display program output
            // to the user of the shell.
            let stdout = io::stdout();

            // Start a program running repl.
            repl::start(stdin, stdout, parse_and_run(&args));
            Ok(())
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            parse_and_run(&args)(&text)
        }
    }
}

fn parse_and_run<'a>(args: &'a ArgvMap) -> impl Fn(&String) -> Result<()> + 'a {
    move |text: &String| {
        // Parse with the primary grammar and run each command in order.
        let program = parse_primary(text.as_bytes())?;

        // Print the program if the flag is given.
        if args.get_bool("-#") {
            println!("{:#?}", program);
        }

        // Run it!
        program.run().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
