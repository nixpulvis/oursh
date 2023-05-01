//! Quick and effective raw mode repl library for ANSI terminals.
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::{Stdin, Stdout};
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use crate::program::Runtime;

#[cfg(feature = "raw")]
use {
    termion::cursor::DetectCursorPos,
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
    crate::repl::action::{Action, ActionContext},
};

#[cfg(not(feature = "raw"))]
use {
    std::io::BufRead,
    crate::program::parse_and_run,
};

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
pub fn start(mut stdin: Stdin, mut stdout: Stdout, runtime: &mut Runtime)
    -> crate::program::Result<WaitStatus>
{
    #[cfg(feature = "raw")]
    raw_loop(stdin, stdout, runtime);
    #[cfg(not(feature = "raw"))]
    buffered_loop(stdin, stdout, runtime);

    Ok(WaitStatus::Exited(Pid::this(), 0))
}

#[cfg(feature = "raw")]
fn raw_loop(stdin: Stdin, stdout: Stdout, runtime: &mut Runtime) {
    // Convert the tty's stdout into raw mode.
    let mut stdout = stdout.into_raw_mode()
        .expect("error opening raw mode");

    // Display the inital prompt.
    prompt::ps1(&mut stdout);

    // XXX: Hack to get the prompt length.
    let prompt_length = stdout.cursor_pos().unwrap().0;

    // TODO #5: We need a better state object for these values.
    let mut text = String::new();

    // Create an context to pass to the actions.
    let mut context = ActionContext {
        runtime,
        prompt_length,
        stdout: &mut stdout,
        text: &mut text,
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
fn buffered_loop(stdin: Stdin, mut stdout: Stdout, runtime: &mut Runtime) {
    // Display the inital prompt.
    prompt::ps1(&mut stdout);

    for line in stdin.lock().lines() {
        let line = line.unwrap();  // TODO: Exit codes
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
        prompt::ps0(&mut stdout);
        if parse_and_run(&line, runtime).is_ok() {
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
pub mod prompt;
#[cfg(feature = "raw")]
pub mod action;
#[cfg(feature = "completion")]
pub mod completion;
#[cfg(feature = "history")]
pub mod history;
