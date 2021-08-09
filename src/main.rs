#![feature(termination_trait_lib)]

extern crate docopt;
extern crate nix;
extern crate oursh;
extern crate termion;
extern crate dirs;

use std::{
    env,
    process::Termination,
    fs::File,
    io::{self, Read},
    cell::RefCell,
    rc::Rc,
};
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use docopt::{Docopt, Value};
use termion::is_tty;
use dirs::home_dir;
use oursh::{
    repl::{
        self,
        Prompt,
    },
    program::{parse_and_run, Result, Error},
    process::{Jobs, IO},
};

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage:
    oursh    [options] [<file> [<arguments>...]]
    oursh -c [options] <command_string> [<command_name> [<arguments>...]]
    oursh -s [options] [<arguments>...]

Options:
    -h --help       Show this screen.
    -v --verbose    Print extra information.
    -a --ast        Print program ASTs.
    -# --alternate  Use alternate program syntax.
    --noprofile     Don't load and profile code on launch.
";

// Our shell, for the greater good. Ready and waiting.
// TODO: Replace program::Result
//
fn main() -> MainResult {
    // Parse argv and exit the program with an error message if it fails.
    let mut args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    // Elementary job management.
    let mut jobs: Jobs = Rc::new(RefCell::new(vec![]));

    // Default inputs and outputs.
    let mut io = IO::default();

    // Run the profile before anything else.
    // TODO:
    // - ourshrc
    // - oursh_logout
    // - Others?
    if !args.get_bool("--noprofile") {
        if let Some(mut path) = home_dir() {
            path.push(".oursh_profile");
            if let Ok(mut file) = File::open(path) {
                let mut contents = String::new();
                if let Ok(_) = file.read_to_string(&mut contents) {
                    if let Err(e) = parse_and_run(&contents, io, &mut jobs, &args) {
                        eprintln!("failed to source profile: {:?}", e);
                    }
                }
            }
        }
    }

    if let Some(Value::Plain(Some(ref c))) = args.find("<command_string>") {
        MainResult(parse_and_run(c, io, &mut jobs, &args))
    } else if let Some(Value::Plain(Some(ref filename))) = args.find("<file>") {
        let mut file = File::open(filename)
            .expect(&format!("error opening file: {}", filename));

        // Fill a string buffer from the file.
        let mut text = String::new();
        file.read_to_string(&mut text)
            .expect("error reading file");

        // Run the program.
        MainResult(parse_and_run(&text, io, &mut jobs, &args))
    } else {
        // Standard input file descriptor (0), used for user input from the
        // user of the shell.
        let stdin = io::stdin();

        // TODO: Verify we don't actually need to do anything with this flag
        // since we process STDIN from the repl regardless.
        //
        // args.get_bool("-s")

        // Process text in raw mode style if we're attached to a tty.
        if is_tty(&stdin) {
            // Standard output file descriptor (1), used to display program output
            // to the user of the shell.
            let stdout = io::stdout();

            // Trap SIGINT.
            ctrlc::set_handler(move || {
                // noop for now.
            }).unwrap();

            // Start a program running repl.
            // A styled static (for now) prompt.
            let prompt = Prompt::sh_style();
            repl::start(prompt, stdin, stdout, &mut io, &mut jobs, &mut args);
            MainResult(Ok(WaitStatus::Exited(Pid::this(), 0)))
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            MainResult(parse_and_run(&text, io, &mut jobs, &args))
        }
    }
}

#[derive(Debug)]
struct MainResult(Result<WaitStatus>);
impl Termination for MainResult {
    fn report(self) -> i32 {
        match self.0 {
            Ok(WaitStatus::Exited(_pid, exit_code)) => exit_code,
            Ok(WaitStatus::Signaled(_pid, _signal, _coredump)) => 128,
            Err(Error::Read) => 1,
            Err(Error::Parse) => 2,
            Err(Error::Runtime) => 127,
            _ => unreachable!(),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_has_a_test() {}
}
