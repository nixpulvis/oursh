//! Quick and effective raw mode repl library for ANSI terminals.
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use crate::process::{Jobs, IO};
use docopt::ArgvMap;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use std::io::{Stdin, Stdout};

#[cfg(feature = "raw")]
use {
    crate::repl::action::{Action, ActionContext},
    termion::cursor::DetectCursorPos,
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
};

#[cfg(not(feature = "raw"))]
use {
    crate::program::{parse_and_run, Runtime},
    std::io::BufRead,
};

#[cfg(feature = "history")]
use self::history::History;

/// Start a REPL over the strings the user provides.
///
/// ## Examples
///
/// ```ignore
/// use std::io;
/// use oursh::program::Result;
/// use oursh::repl;
///
/// fn echo(text: &String) -> Result<()> {
///     eprintln!("{}", text);
///     Ok(())
/// }
///
/// repl::start(io::stdin(), io::stdout(), echo);
/// ```
// TODO: Partial syntax, completion.
#[allow(unused_mut)]
pub fn start(
    mut stdin: Stdin,
    mut stdout: Stdout,
    io: &mut IO,
    jobs: &mut Jobs,
    args: &mut ArgvMap,
) -> crate::program::Result<WaitStatus> {
    // Load history from file in $HOME.
    #[cfg(feature = "history")]
    let mut history = History::load();

    #[cfg(feature = "raw")]
    raw_loop(stdin, stdout, io, jobs, args);
    #[cfg(not(feature = "raw"))]
    buffered_loop(stdin, stdout, io, jobs, args);

    Ok(WaitStatus::Exited(Pid::this(), 0))
}

#[cfg(feature = "raw")]
fn raw_loop(stdin: Stdin, stdout: Stdout, io: &mut IO, jobs: &mut Jobs, args: &mut ArgvMap) {
    // Convert the tty's stdout into raw mode.
    let mut stdout = stdout.into_raw_mode().expect("error opening raw mode");

    // Display the initial prompt.
    prompt::ps1(&mut stdout);

    // XXX: Hack to get the prompt length.
    let prompt_length = stdout.cursor_pos().unwrap().0;

    // TODO #5: We need a better state object for these values.
    let mut text = String::new();

    // Create an context to pass to the actions.
    let mut context = ActionContext {
        stdout: &mut stdout,
        io: io,
        jobs: jobs,
        args: args,
        prompt_length: prompt_length,
        text: &mut text,
        #[cfg(feature = "history")]
        history: &mut history,
    };
    // Iterate the keys as a user presses them.
    // TODO #5: Mouse?
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => Action::enter(&mut context),
            #[cfg(feature = "completion")]
            Key::Char('\t') => Action::complete(&mut context),
            Key::Char(c) => Action::insert(&mut context, c),
            Key::Left => Action::left(&mut context),
            Key::Right => Action::right(&mut context),
            Key::Backspace => Action::backspace(&mut context),
            Key::Ctrl('a') => Action::home(&mut context),
            Key::Ctrl('e') => Action::end(&mut context),
            Key::Ctrl('c') => Action::interrupt(&mut context),
            Key::Ctrl('d') => Action::eof(&mut context),
            Key::Ctrl('l') => Action::clear(&mut context),
            #[cfg(feature = "history")]
            Key::Up => Action::history_up(&mut context),
            #[cfg(feature = "history")]
            Key::Down => Action::history_down(&mut context),
            _ => {}
        }
    }
}

#[cfg(not(feature = "raw"))]
fn buffered_loop(
    stdin: Stdin,
    mut stdout: Stdout,
    io: &mut IO,
    jobs: &mut Jobs,
    args: &mut ArgvMap,
) {
    // Display the initial prompt.
    prompt::ps1(&mut stdout);

    for line in stdin.lock().lines() {
        let line = line.unwrap(); // TODO: Exit codes
                                  //     let readline = runtime.rl.as_mut().unwrap().readline(&prompt);
                                  //     match readline {
                                  //         Ok(line) => {
                                  //         },
                                  //         // Err(ReadlineError::Interrupted) => {
                                  //         //     println!("^C");
                                  //         //     continue;
                                  //         // },
                                  //         // Err(ReadlineError::Eof) => {
                                  //         //     println!("exit");
                                  //         //     code = 0;
                                  //         //     break;
                                  //         // },
                                  //         Err(err) => {
                                  //             println!("error: {:?}", err);
                                  //             code = 130;
                                  //             break;
                                  //         }
        let mut runtime = Runtime {
            background: false,
            io: io.clone(),
            jobs: jobs,
            args: args,
            #[cfg(feature = "history")]
            history: history,
        };
        if parse_and_run(&line, &mut runtime).is_ok() {
            #[cfg(feature = "history")]
            history.add(&line, 1);
        }
        #[cfg(feature = "history")]
        history.add(&line, 1);
        #[cfg(feature = "history")]
        history.reset_index();

        prompt::ps1(&mut stdout);
    }
}

// pub mod display;
#[cfg(feature = "raw")]
pub mod action;
#[cfg(feature = "completion")]
pub mod completion;
#[cfg(feature = "history")]
pub mod history;
pub mod prompt;
