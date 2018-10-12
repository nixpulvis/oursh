extern crate oursh;
extern crate termion;

use std::process::exit;
use std::io::{self, BufRead, Read, Write};
use oursh::job::Job;
use oursh::program::{parse_default, Program};
use oursh::repl;
use termion::event::Key;
use termion::input::TermRead;

// Our shell, for the greater good. Ready and waiting.
fn main() {
    // Block exits via `SIGINT`, generally triggered with ctrl-c.
    if repl::is_tty(&io::stdin()) {
        repl::trap_sigint()
            .expect("error trapping sigint");

        let (mut stdin, mut stdout) = repl::raw_io();

        // TODO: Move all this gross logic into a clean repl API.
        // Print a boring static prompt.
        if repl::is_tty(&io::stdin()) {
            repl::Prompt::new().display(&mut stdout);
        }
        stdout.flush().unwrap();
        let mut text = String::new();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => exit(0),
                Key::Char('\n') => {
                    print!("\n\r");
                    stdout.flush().unwrap();
                    parse_and_run(&text);
                    print!("\r");
                    // Print a boring static prompt.
                    if repl::is_tty(&io::stdin()) {
                        repl::Prompt::new().display(&mut stdout);
                    }
                    stdout.flush().unwrap();

                    // Reset the text for the next program.
                    text.clear();
                },
                Key::Char(c) => {
                    print!("{}", c);
                    text.push(c);
                },
                Key::Alt(c)  => print!("Alt-{}", c),
                Key::Ctrl(c) => print!("Ctrl-{}", c),
                Key::Left    => print!("<left>"),
                Key::Right   => print!("<right>"),
                Key::Up      => print!("<up>"),
                Key::Down    => print!("<down>"),
                k            => print!("{:?}", k),
            }
            stdout.flush().unwrap();
        }
    } else {
        let (mut stdin, mut stdout) = (io::stdin(), io::stdout());
        let mut text = String::new();
        stdin.lock().read_to_string(&mut text).unwrap();
        parse_and_run(&text);
    }
}

fn parse_and_run(text: &String) {
    // Parse with the default grammar and run each command in order.
    let program = parse_default(text.as_bytes());
    for command in program.commands().iter() {
        // TODO: Can we disable raw mode for the program being
        // run?
        Job::new(&**command).run();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
