extern crate oursh;

use std::io::{self, BufRead};
use std::process::exit;
use oursh::job::Job;
use oursh::program::{parse_default, Program};
use oursh::repl;

// Our shell, for the greater good. Ready and waiting.
fn main() {
    // Standard input file descriptor (0), used for user input from the user
    // of the shell.
    let mut stdin = io::stdin();

    // Standard output file descriptor (1), used to display program output to
    // the user of the shell.
    let stdout = io::stdout();

    // Block exits via `SIGINT`, generally triggered with ctrl-c.
    if repl::is_tty(&stdin) {
        repl::trap_sigint()
            .expect("error trapping sigint");
    }

    loop {
        // Print a boring static prompt.
        if repl::is_tty(&stdin) {
            repl::Prompt::new().display(&stdout);
        }

        // TODO: Enable raw access to STDIN, so we can read as the user
        // types. By default the underlying file is line buffered. This
        // will allow us to process history, syntax, and more!

        // Parse with the default grammar and run each command in order.
        let program = parse_default(stdin.lock());
        for command in program.commands().iter() {
            Job::new(&**command).run();
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
