//! Actions to be bound to input methods.
//!
//! - `Insert`
//! - `Backspace`
//! - `Delete`
//! - `Submit` (or `Enter`, `Execute`)
//! - `Move`, args: `home`, `end`, `word:left`, `right`, ...
//! - `Interrupt`
//! - `Exit`
//! - `Clear` (TODO: needed?)
//!
//! - `Complete`
//! - `History`, args: `next`, `prev`
use std::io::Write;
use super::ActionContext;

#[cfg(feature = "raw")]
use {
    std::process::exit,
    termion::cursor::DetectCursorPos,
};

#[cfg(feature = "completion")]
use super::completion::*;


pub enum Action {
    /// Insert a single character into the REPL.
    Insert(char),
    Backspace,
    Delete,
    Enter,
    Move(Movement),
    Interrupt,
    Exit,
}

pub enum Movement {
    Home,
    End,
    Word(Direction),
    Char(Direction),
}

pub enum Direction {
    Left,
    Right,
}

impl Action {
    // pub fn execute(self, context: &mut ActionContext) {
    //     match self {
    //         Action::Insert(c) => Action::insert(context, c),
    //         Action::Enter => Action::enter(context),
    //         _ => {}
    //     }
    // }

    pub fn insert(context: &mut ActionContext, c: char) {
        if let Ok((x, y)) = context.stdout.cursor_pos() {
            let i = (x - context.prompt_length) as usize;
            context.text.insert(i, c);
            print!("{}{}",
                   &context.text[i..],
                   termion::cursor::Goto(x + 1, y));
        } else {
            context.text.push(c);
            print!("{}", c);
        }
        context.stdout.flush().unwrap();
    }

    pub fn backspace(context: &mut ActionContext) {
        if let Ok((x, y)) = context.stdout.cursor_pos() {
            if x > context.prompt_length {
                let i = x - context.prompt_length;
                context.text.remove((i - 1) as usize);
                print!("{}{}{}{}",
                       termion::cursor::Goto(context.prompt_length, y),
                       termion::clear::UntilNewline,
                       context.text,
                       termion::cursor::Goto(x - 1, y));
                context.stdout.flush().unwrap();
            }
        }
    }

    pub fn enter(context: &mut ActionContext) {
        // Perform a raw mode line break.
        print!("\n\r");
        context.stdout.flush().unwrap();

        // Run the command.
        context.stdout.suspend_raw_mode().unwrap();
        if (context.runner)(&context.text).is_ok() {
            #[cfg(feature = "history")]
            context.history.add(&context.text, 1);
        }
        context.stdout.activate_raw_mode().unwrap();

        // Reset for the next program.
        context.text.clear();
        #[cfg(feature = "history")]
        context.history.reset_index();

        // Print a boring static prompt.
        context.prompt.display(&mut context.stdout);
    }

    pub fn interrupt(context: &mut ActionContext) {
        // TODO: Send signal if we're running a program.
        context.text.clear();
        print!("^C\n\r");
        context.prompt.display(&mut context.stdout);
    }

    pub fn eof(context: &mut ActionContext) {
        if context.text.is_empty() {
            print!("exit\n\r");
            context.stdout.flush().unwrap();

            // Save history to file in $HOME.
            #[cfg(feature = "history")]
            context.history.save();

            // Manually drop the raw terminal.
            // TODO: Needed?
            // drop(context.stdout);

            // Exit this wonderful world.
            exit(0)
        }
    }

    pub fn left(context: &mut ActionContext) {
        if let Ok((x, _y)) = context.stdout.cursor_pos() {
            if x > context.prompt_length {
                print!("{}", termion::cursor::Left(1));
                context.stdout.flush().unwrap();
            }
        }
    }

    pub fn right(context: &mut ActionContext) {
        if let Ok((x, _y)) = context.stdout.cursor_pos() {
            if x < context.prompt_length + context.text.len() as u16 {
                print!("{}", termion::cursor::Right(1));
                context.stdout.flush().unwrap();
            }
        }
    }

    pub fn home(context: &mut ActionContext) {
        if let Ok((_x, y)) = context.stdout.cursor_pos() {
            print!("{}", termion::cursor::Goto(context.prompt_length, y));
            context.stdout.flush().unwrap();
        }
    }

    pub fn end(context: &mut ActionContext) {
        if let Ok((_x, y)) = context.stdout.cursor_pos() {
            let end = context.prompt_length + context.text.len() as u16;
            print!("{}", termion::cursor::Goto(end, y));
            context.stdout.flush().unwrap();
        }
    }

    pub fn clear(context: &mut ActionContext) {
        print!("{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1));
        context.prompt.display(&mut context.stdout);
    }

    #[cfg(feature = "history")]
    pub fn history_up(context: &mut ActionContext) {
        print!("{}{}",
               termion::cursor::Left(1000),  // XXX
               termion::clear::CurrentLine);
        context.prompt.display(&mut context.stdout);

        if let Some(history_text) = context.history.get_up() {
            *context.text = history_text;
            print!("{}", context.text);
        }
        context.stdout.flush().unwrap();
    }

    #[cfg(feature = "history")]
    pub fn history_down(context: &mut ActionContext) {
        print!("{}{}",
               termion::cursor::Left(1000),  // XXX
               termion::clear::CurrentLine);
        context.prompt.display(&mut context.stdout);

        if let Some(history_text) = context.history.get_down() {
            *context.text = history_text;
            print!("{}", context.text);
            context.stdout.flush().unwrap();
        } else {
            context.text.clear();
        }
    }

    #[cfg(feature = "completion")]
    pub fn complete(context: &mut ActionContext) {
        match complete(&context.text) {
            Completion::Partial(possibilities) => {
                if possibilities.len() > 25 {
                    print!("\n\r");
                    for possibility in possibilities {
                        print!("{}\n\r", possibility);
                    }
                    print!("\n\r");
                } else {
                    print!("\n\r{}\n\r", possibilities.join("\t"));
                }
                context.prompt.display(&mut context.stdout);
                print!("{}", context.text);
                context.stdout.flush().unwrap();
            },
            Completion::Complete(t) => {
                *context.text = t;
                print!("{}{}",
                       termion::cursor::Left(1000),  // XXX
                       termion::clear::CurrentLine);
                context.stdout.flush().unwrap();
                context.prompt.display(&mut context.stdout);
                print!("{}", context.text);
                context.stdout.flush().unwrap();
            },
            Completion::None => {},
        }
    }
}
