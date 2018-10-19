//! Quick and effective raw mode repl library for ANSI terminals.
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::{Write, Stdin, Stdout};
use nix::unistd;
use pwd::Passwd;
use termion::{style, color};
use program::Result;
#[cfg(not(feature = "raw"))]
use std::io::BufRead;
#[cfg(feature = "raw")]
use std::process::exit;
#[cfg(feature = "raw")]
use termion::cursor::DetectCursorPos;
#[cfg(feature = "raw")]
use termion::event::Key;
#[cfg(feature = "raw")]
use termion::input::TermRead;
#[cfg(feature = "raw")]
use termion::raw::IntoRawMode;
#[cfg(feature = "completion")]
use repl::completion::*;
#[cfg(feature = "history")]
use repl::history::History;

/// Start a REPL over the strings the user provides.
// TODO: Partial syntax, completion.
#[allow(unused_mut)]
pub fn start<F>(mut stdin: Stdin, mut stdout: Stdout, runner: F)
    where F: Fn(&String) -> Result<()>
{
    // Load history from file in $HOME.
    #[cfg(feature = "history")]
    let mut history = History::load();

    // A styled static (for now) prompt.
    let prompt = Prompt::new().sh_style();

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

    // Iterate the keys as a user presses them.
    // TODO #5: Mouse?
    #[cfg(feature = "raw")]
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
            },
            Key::Char('\n') => {
                // Perform a raw mode line break.
                print!("\n\r");
                stdout.flush().unwrap();

                // Run the command.
                stdout.suspend_raw_mode().unwrap();
                if runner(&text).is_ok() {
                    #[cfg(feature = "history")]
                    history.add(&text, 1);
                }
                stdout.activate_raw_mode().unwrap();

                // Reset for the next program.
                text.clear();
                #[cfg(feature = "history")]
                history.reset_index();

                // Print a boring static prompt.
                prompt.display(&mut stdout);
            },
            #[cfg(feature = "completion")]
            Key::Char('\t') => {
                match complete(&text) {
                    Completion::None => continue,
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
                        prompt.display(&mut stdout);
                        print!("{}", text);
                        stdout.flush().unwrap();
                    },
                    Completion::Complete(t) => {
                        text = t;
                        print!("{}{}",
                               termion::cursor::Left(1000),  // XXX
                               termion::clear::CurrentLine);
                        stdout.flush().unwrap();
                        prompt.display(&mut stdout);
                        print!("{}", text);
                        stdout.flush().unwrap();
                    }
                }
            },
            Key::Char(c) => {
                if let Ok((x, y)) = stdout.cursor_pos() {
                    let i = (x - prompt_length) as usize;
                    text.insert(i, c);
                    print!("{}{}",
                           &text[i..],
                           termion::cursor::Goto(x + 1, y));
                } else {
                    text.push(c);
                    print!("{}", c);
                }
                stdout.flush().unwrap();
            },
            #[cfg(feature = "history")]
            Key::Up => {
                print!("{}{}",
                       termion::cursor::Left(1000),  // XXX
                       termion::clear::CurrentLine);
                prompt.display(&mut stdout);

                if let Some(history_text) = history.get_up() {
                    text = history_text;
                    print!("{}", text);
                }
                stdout.flush().unwrap();
            },
            #[cfg(feature = "history")]
            Key::Down => {
                print!("{}{}",
                       termion::cursor::Left(1000),  // XXX
                       termion::clear::CurrentLine);
                prompt.display(&mut stdout);

                if let Some(history_text) = history.get_down() {
                    text = history_text;
                    print!("{}", text);
                    stdout.flush().unwrap();
                } else {
                    text.clear();
                }
            },
            Key::Left => {
                if let Ok((x, _y)) = stdout.cursor_pos() {
                    if x > prompt_length {
                        print!("{}", termion::cursor::Left(1));
                        stdout.flush().unwrap();
                    }
                }
            },
            Key::Right => {
                if let Ok((x, _y)) = stdout.cursor_pos() {
                    if x < prompt_length + text.len() as u16 {
                        print!("{}", termion::cursor::Right(1));
                        stdout.flush().unwrap();
                    }
                }
            },
            Key::Backspace => {
                if let Ok((x, y)) = stdout.cursor_pos() {
                    if x > prompt_length {
                        let i = x - prompt_length;
                        text.remove((i - 1) as usize);
                        print!("{}{}{}{}",
                               termion::cursor::Goto(prompt_length, y),
                               termion::clear::UntilNewline,
                               text,
                               termion::cursor::Goto(x - 1, y));
                        stdout.flush().unwrap();
                    }
                }
            },
            Key::Ctrl('a') => {
                if let Ok((_x, y)) = stdout.cursor_pos() {
                    print!("{}", termion::cursor::Goto(prompt_length, y));
                    stdout.flush().unwrap();
                }
            },
            Key::Ctrl('e') => {
                if let Ok((_x, y)) = stdout.cursor_pos() {
                    let end = prompt_length + text.len() as u16;
                    print!("{}", termion::cursor::Goto(end, y));
                    stdout.flush().unwrap();
                }
            },
            Key::Ctrl('c') => {
                // TODO: Send signal if we're running a program.
                text.clear();
                print!("^C\n\r");
                prompt.display(&mut stdout);
            },
            Key::Ctrl('d') => {
                if text.is_empty() {
                    print!("exit\n\r");
                    stdout.flush().unwrap();

                    // Save history to file in $HOME.
                    #[cfg(feature = "history")]
                    history.save();

                    // Exit this wonderful world.
                    exit(0)
                }
            },
            Key::Ctrl('l') => {
                print!("{}{}",
                       termion::clear::All,
                       termion::cursor::Goto(1, 1));
                prompt.display(&mut stdout);
            },
            _ => {}
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

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Prompt {
    /// The most basic possible prompt.
    pub const DEFAULT_FORMAT: &'static str = "$ ";

    pub fn new() -> Self {
        Prompt(format!("{}", Self::DEFAULT_FORMAT))
    }

    pub fn sh_style(self) -> Self {
        const NAME: &'static str = "oursh";
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        let version = &VERSION[0..(VERSION.len() - 2)];
        // TODO: Add a flag for pretending to be `sh`.
        // let name = "sh";
        // let version = "4.4";
        Prompt(format!("{}-{}$ ", NAME, version))
    }

    pub fn nixpulvis_style(self) -> Self {
        let mut buf = [0u8; 64];
        let hostname_cstr = unistd::gethostname(&mut buf)
            .expect("error getting hostname");
        let hostname = hostname_cstr.to_str()
            .expect("hostname wasn't valid UTF-8");
        let passwd = Passwd::current_user()
            .expect("error i don't exist, passwd validation failed!");
        let whoami = passwd.name;
        let cwd = unistd::getcwd()
            .expect("error reading cwd");
        Prompt(format!("{}{}{}@{}{}{}:{}{}{}{}$ ",
            color::Fg(color::Red),
            whoami,
            color::Fg(color::Reset),
            color::Fg(color::Blue),
            hostname,
            color::Fg(color::Reset),
            color::Fg(color::Green),
            cwd.display(),
            color::Fg(color::Reset),
            " "))
    }

    pub fn long_style(self) -> Self {
        let mut buf = [0u8; 64];
        let hostname_cstr = unistd::gethostname(&mut buf)
            .expect("error getting hostname");
        let hostname = hostname_cstr.to_str()
            .expect("hostname wasn't valid UTF-8");
        Prompt(format!("{}{} {} $ {} {} {} {} {}{} ",
            style::Invert,
            color::Fg(color::Green),
            hostname,
            color::Fg(color::Yellow),
            color::Fg(color::Red),
            color::Fg(color::Magenta),
            color::Fg(color::Cyan),
            color::Fg(color::Reset),
            style::Reset))
    }

    pub fn short_style(self) -> Self {
        Prompt(format!("{}{}our$h{}{} ",
            color::Fg(color::Red),
            style::Invert,
            color::Fg(color::Reset),
            style::Reset))
    }

    pub fn display(&self, stdout: &mut impl Write) {
        write!(stdout, "{}", self.0).unwrap();
        stdout.flush().unwrap();
    }
}


#[cfg(feature = "completion")]
pub mod completion;
#[cfg(feature = "history")]
pub mod history;
