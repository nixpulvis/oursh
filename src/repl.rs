//! Quick and effective raw mode repl library for ANSI terminals.
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::{Stdin, Stdout};
use crate::program::Result;
pub use self::prompt::Prompt;

#[cfg(feature = "raw")]
use {
    termion::cursor::DetectCursorPos,
    termion::event::Key,
    termion::input::TermRead,
    termion::raw::IntoRawMode,
};
#[cfg(feature = "raw")]
use self::action::{Action, ActionContext};

#[cfg(not(feature = "raw"))]
use std::io::BufRead;

#[cfg(feature = "history")]
use self::history::History;

/// Start a REPL over the strings the user provides.
///
/// ## Examples
///
/// ```
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
pub fn start<F>(mut prompt: Prompt, mut stdin: Stdin, mut stdout: Stdout, runner: F)
    where F: Fn(&String) -> Result<()>
{
    // Load history from file in $HOME.
    #[cfg(feature = "history")]
    let mut history = History::load();

    // Convert the tty's stdout into raw mode.
    #[cfg(feature = "raw")]
    let mut stdout = stdout.into_raw_mode()
        .expect("error opening raw mode");

    // Display the inital prompt.
    prompt.display(&mut stdout);

    // XXX: Hack to get the prompt length.
    #[cfg(feature = "raw")]
    let prompt_length = stdout.cursor_pos().unwrap().0;

    // TODO #5: We need a better state object for these values.
    #[cfg(feature = "raw")]
    let mut text = String::new();

    #[cfg(feature = "raw")]
    {
        // Create an context to pass to the actions.
        let mut context = ActionContext {
            stdout: &mut stdout,
            runner: &runner,
            prompt: &mut prompt,
            #[cfg(feature = "raw")]
            prompt_length: prompt_length,
            #[cfg(feature = "raw")]
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
    for line in stdin.lock().lines() {
        // XXX: Blindly read a full line.
        let text = line.unwrap();

        // XXX: Blindly run the text.
        if runner(&text).is_ok() {
            #[cfg(feature = "history")]
            {
                history.add(&text, 1);
                history.reset_index();
            }
        }

        // Display a brand spanking new prompt.
        prompt.display(&mut stdout);
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
