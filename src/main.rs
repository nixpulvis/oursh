#![feature(termination_trait_lib)]

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
use dirs::home_dir;
use docopt::{Docopt, Value};
use termion::is_tty;
use rustyline::{
    Editor,
    error::ReadlineError,
};
use oursh::{
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
    let args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    // Elementary job management.
    let mut jobs: Jobs = Rc::new(RefCell::new(vec![]));

    // Default inputs and outputs.
    let io = IO::default();

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
                    if let Err(e) = parse_and_run(&contents, io, &mut jobs, &args, None) {
                        eprintln!("failed to source profile: {:?}", e);
                    }
                }
            }
        }
    }

    if let Some(Value::Plain(Some(ref c))) = args.find("<command_string>") {
        MainResult(parse_and_run(c, io, &mut jobs, &args, None))
    } else if let Some(Value::Plain(Some(ref filename))) = args.find("<file>") {
        let mut file = File::open(filename)
            .expect(&format!("error opening file: {}", filename));

        // Fill a string buffer from the file.
        let mut text = String::new();
        file.read_to_string(&mut text)
            .expect("error reading file");

        // Run the program.
        MainResult(parse_and_run(&text, io, &mut jobs, &args, None))
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

            // Trap SIGINT.
            ctrlc::set_handler(move || {
                dbg!("HIT! help me!");
                // noop for now.
            }).unwrap();

            let home = env::var("HOME").expect("HOME variable not set.");
            let history_path = format!("{}/.oursh_history", home);

            let mut rl = Editor::<()>::new();
            if rl.load_history(&history_path).is_err() {
                println!("No previous history.");
            }

            let code;
            loop {
                let prompt = expand_prompt(env::var("PS1").unwrap_or(sh_style_prompt()));
                let readline = rl.readline(&prompt);
                match readline {
                    Ok(line) => {
                        parse_and_run(&line, io, &mut jobs, &args, Some(&mut rl)).unwrap();
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

            rl.save_history(&history_path).unwrap();
            MainResult(Ok(WaitStatus::Exited(Pid::this(), code)))
        } else {
            // Fill a string buffer from STDIN.
            let mut text = String::new();
            stdin.lock().read_to_string(&mut text).unwrap();

            // Run the program.
            MainResult(parse_and_run(&text, io, &mut jobs, &args, None))
        }
    }
}

fn expand_prompt(prompt: String) -> String {
    let mut result = String::new();
    let mut command = false;
    for c in prompt.chars() {
        if command {
            // TODO: https://ss64.com/bash/syntax-prompt.html
            result += &match c {
                'h' => {
                    let mut buf = [0u8; 64];
                    let cstr = gethostname(&mut buf).expect("error getting hostname");
                    cstr.to_str().expect("error invalid UTF-8").into()
                }
                'u' => var("USER").unwrap_or("".into()),
                'w' => var("PWD").unwrap_or("".into()),
                '\\' => "".into(),
                c => c.into(),
            };
        } else if c == '\\' {
            command = true;
        } else {
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
            Err(Error::Read) => 1,
            Err(Error::Parse) => 2,
            Err(Error::Runtime) => 127,
            _ => unreachable!(),
        }
    }
}

pub fn sh_style_prompt() -> String {
    const NAME: &'static str = "oursh";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let version = &VERSION[0..(VERSION.len() - 2)];
    // TODO: Add a flag for pretending to be `sh`.
    // let name = "sh";
    // let version = "4.4";
    format!("{}-{}$ ", NAME, version)
}
