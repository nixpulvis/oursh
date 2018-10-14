#[macro_use]
extern crate oursh;
extern crate termion;
extern crate docopt;

use std::env;
use std::io::{self, Read};
use std::fs::File;
use oursh::program::{parse_primary, Program};
use oursh::repl;
use termion::is_tty;
use docopt::{Docopt, ArgvMap, Value};

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage: oursh [-#] [<file>]

Options:
    -#, --ast  Print an AST of each program.
";

// Our shell, for the greater good. Ready and waiting.
fn main() {
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
        parse_and_run(&args)(&text);
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
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            parse_and_run(&args)(&text);
        }
    }
}

enum RunMode {
    Repl,
    Stdin,
    File,
}

// fn ::

fn parse_and_run<'a>(args: &'a ArgvMap) -> impl Fn(&String) + 'a {
    move |text: &String| {
        // Parse with the primary grammar and run each command in order.
        match parse_primary(text.as_bytes()) {
            Ok(program) => {
                if args.get_bool("-#") { debug!(program); }

                program.run()
                    .expect(&format!("error running program: {:?}", program));
            },
            Err(()) => {
                println!("error parsing text: {}", text);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
