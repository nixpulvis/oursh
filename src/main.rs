#![feature(exclusive_range_pattern, termination_trait_lib)]

extern crate docopt;
extern crate nix;
extern crate oursh;
extern crate termion;
extern crate dirs;

use std::{
    env::{self, var},
    process::Termination,
    fs::File,
    io::{self, Read},
    cell::RefCell,
    rc::Rc,
};
use nix::sys::wait::WaitStatus;
use nix::unistd::{gethostname, Pid};
use docopt::{Docopt, Value};
use termion::is_tty;
use rustyline::{
    Editor,
    error::ReadlineError,
};
use oursh::{
    config::source_profile,
    program::{parse_and_run, Runtime, Result, Error},
    process::{Jobs, IO},
};

pub const NAME: &'static str = "oursh";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Write the Docopt usage string.
const USAGE: &'static str = "
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
    let mut args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    // Elementary job management.
    let mut jobs: Jobs = Rc::new(RefCell::new(vec![]));

    // Default inputs and outputs.
    let io = IO::default();

    let home = dirs::home_dir().expect("HOME variable not set.");

    let mut runtime = Runtime { io,
        jobs: &mut jobs,
        args: &mut args,
        background: false,
        rl: None,
        history_path: home.join(".oursh_history"),
    };

    // Run the profile before anything else.
    // TODO:
    // - ourshrc
    // - oursh_logout
    // - Others?
    if !runtime.args.get_bool("--noprofile") {
        source_profile(&mut runtime);
    }

    let args = runtime.args.clone();
    if let Some(Value::Plain(Some(ref c))) = args.find("<command_string>") {
        MainResult(parse_and_run(c, &mut runtime))
    } else if let Some(Value::Plain(Some(ref filename))) = runtime.args.find("<file>") {
        let mut file = File::open(filename)
            .expect(&format!("error opening file: {}", filename));

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
            // // Standard output file descriptor (1), used to display program output
            // // to the user of the shell.
            // let stdout = io::stdout();

            let mut rl = Editor::<()>::new();
            runtime.rl = Some(&mut rl);
            if runtime.rl.as_mut().unwrap().load_history(&runtime.history_path).is_err() {
                println!("No previous history.");
            }

            // Trap SIGINT.
            ctrlc::set_handler(move || println!()).unwrap();

            let code;
            loop {
                let prompt = expand_prompt(env::var("PS1").unwrap_or("\\s-\\v\\$ ".into()));
                let readline = runtime.rl.as_mut().unwrap().readline(&prompt);
                match readline {
                    Ok(line) => {
                        match parse_and_run(&line, &mut runtime) {
                            Ok(status) => {
                                match status {
                                    WaitStatus::Exited(_pid, _code) =>
                                        runtime.rl.as_mut().unwrap().save_history(&runtime.history_path).unwrap(),
                                    WaitStatus::Signaled(_pid, _signal, _coredump) =>
                                        runtime.rl.as_mut().unwrap().save_history(&runtime.history_path).unwrap(),
                                    _ => {},
                                }
                            }
                            Err(e) => {
                                dbg!(e);
                            }
                        }
                    },
                    Err(ReadlineError::Interrupted) => {
                        println!("^C");
                        continue;
                    },
                    Err(ReadlineError::Eof) => {
                        println!("exit");
                        code = 0;
                        break;
                    },
                    Err(err) => {
                        println!("error: {:?}", err);
                        code = 130;
                        break;
                    }
                }
            }

            runtime.rl.unwrap().save_history(&runtime.history_path).unwrap();
            MainResult(Ok(WaitStatus::Exited(Pid::this(), code)))
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            MainResult(parse_and_run(&text, &mut runtime))
        }
    }
}

fn expand_prompt(prompt: String) -> String {
    let mut result = String::new();
    let mut command = false;
    let mut octal = vec![];
    for c in prompt.chars() {
        let o = octal.iter().map(|c: &char| c.to_string())
                     .collect::<Vec<_>>()
                     .join("");
        if !octal.is_empty() && octal.len() < 3 {
            if ('0'..'8').contains(&c) {
                octal.push(c);
            } else {
                result += &o;
                octal.clear();
            }
        } else if octal.len() == 3 {
            if let Ok(n) = u8::from_str_radix(&o, 8) {
                result.push(n as char);
            }
            octal.clear();
        }

        if command {
            // TODO: https://ss64.com/bash/syntax-prompt.html
            result += &match c {
                'h' => {
                    let mut buf = [0u8; 64];
                    let cstr = gethostname(&mut buf).expect("error getting hostname");
                    cstr.to_str().expect("error invalid UTF-8").into()
                }
                'e' => (0x1b as char).into(),
                'u' => var("USER").unwrap_or("".into()),
                'w' => var("PWD").unwrap_or("".into()),
                's' => NAME.into(),
                'v' => VERSION[0..(VERSION.len() - 2)].into(),
                '0' => { octal.push(c); "".into() },
                '\\' => "".into(),
                c => c.into(),
            };
            command = false;
        } else if c == '\\' {
            command = true;
        } else if octal.is_empty() {
            result.push(c);
        }
    }
    result
}

#[derive(Debug)]
struct MainResult(Result<WaitStatus>);
impl Termination for MainResult {
    fn report(self) -> i32 {
        match self.0 {
            Ok(WaitStatus::Exited(_pid, exit_code)) => exit_code,
            Ok(WaitStatus::Signaled(_pid, _signal, _coredump)) => 128,
            Ok(_) => 0,  // TODO: Is this even remotely correct?
            Err(Error::Read) => 1,
            Err(Error::Parse) => 2,
            Err(Error::Runtime) => 127,
        }
    }
}
