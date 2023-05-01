#![feature(exclusive_range_pattern)]

extern crate docopt;
extern crate nix;
extern crate oursh;
extern crate termion;
extern crate dirs;

use std::{
    env,
    process::{Termination, ExitCode},
    fs::File,
    io::{self, Read},
    cell::RefCell,
    rc::Rc,
};
use nix::sys::wait::WaitStatus;
use docopt::{Docopt, Value};
use termion::is_tty;
use oursh::{
    VERSION,
    repl,
    invocation::source_profile,
    program::{parse_and_run, Runtime, Result, Error},
    process::{Jobs, IO},
};

#[cfg(feature = "history")]
use oursh::repl::history::History;

// Write the Docopt usage string.
const USAGE: &str = "
The oursh utility is a command language interpreter that shall execute commands
read from a command line string, the standard input, or a specified file.

Usage:
    oursh    [options] [<command_file> [<arguments>...]]
    oursh -s [options] [<arguments>...]
    oursh -c [options] <command_string> [<command_name> [<arguments>...]]

By default our will read commands from the command_file operand. If there are no
operands and the -c option is not specified, the -s option shall be assumed.

Options:
    -c              Read commands from the command_string operand.
    -s              Read commands from the standard input.
    -i              Specify that the shell is interactive.
    --login         Act as if invoked as a login shell.
    -h --help       Show this screen.
    -v --verbose    Print extra information.
    -a --ast        Print program ASTs.
    -# --alternate  Use alternate program syntax.
    --posix         Run using the (strict) POSIX language by default.
    --init-file     Override the default profile.
    --rcfile        and RC file locations for startup.
    --noprofile     Don't load and profile code on launch.
    --norc

    --debug
    --debugger
    --dump-po-strings
    --dump-strings
    --noediting
    --restricted
    --version

TODO: Read set [-abCefhmnuvx] [-o option] for a complete list of arguments.
";

// Our shell, for the greater good. Ready and waiting.
// TODO: Replace program::Result
//
fn main() -> MainResult {
    // Parse argv and exit the program with an error message if it fails.
    //
    // TODO: From sh docs:
    //     "with an extension for support of a
    //      leading  <plus-sign> ('+') as noted below."
    let args = Docopt::new(USAGE).and_then(|d|
        d.version(Some(VERSION.into()))
            .argv(env::args())
            .parse())
        .unwrap_or_else(|e| e.exit());

    // Default inputs and outputs for the processes.
    let io = IO::default();
    // Elementary job management.
    let mut jobs: Jobs = Rc::new(RefCell::new(vec![]));

    #[cfg(feature = "history")]
    let mut history = History::load();

    let mut runtime = Runtime {
        io,
        jobs: &mut jobs,
        args: &args,
        background: false,
        #[cfg(feature = "history")]
        history: &mut history,
    };

    // Run the profile before anything else.
    // TODO:
    // - ourshrc
    // - oursh_logout
    // - Others?
    if !args.get_bool("--noprofile") {
        source_profile(&mut runtime);
    }

    if let Some(Value::Plain(Some(ref c))) = args.find("<command_string>") {
        MainResult(parse_and_run(c, &mut runtime))
    } else if let Some(Value::Plain(Some(ref filename))) = args.find("<command_file>") {
        let mut file = File::open(filename)
            .unwrap_or_else(|_| panic!("error opening file: {}", filename));

        // Fill a string buffer from the file.
        let mut text = String::new();
        file.read_to_string(&mut text)
            .expect("error reading file");

        // Run the program.
        MainResult(parse_and_run(&text, &mut runtime))
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
            ctrlc::set_handler(move || println!()).unwrap();

            let result = repl::start(stdin, stdout, &mut runtime);
            MainResult(result)
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            MainResult(parse_and_run(&text, &mut runtime))
        }
    }
}

#[derive(Debug)]
struct MainResult(Result<WaitStatus>);
impl Termination for MainResult {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(WaitStatus::Exited(_pid, code)) => ExitCode::from(code as u8),
            Ok(WaitStatus::Signaled(_pid, _signal, _coredump)) => ExitCode::from(128),
            Ok(_) => ExitCode::from(0),  // TODO: Is this even remotely correct?
            Err(Error::Read) => ExitCode::from(1),
            Err(Error::Parse) => ExitCode::from(2),
            Err(Error::Runtime) => ExitCode::from(127),
        }
    }
}
