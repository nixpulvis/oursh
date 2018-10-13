//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use std::io::{Write, Stdin};
use std::process::exit;
use nix::unistd;
use pwd::Passwd;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::{style, color};

pub fn start<W, F>(mut stdin: Stdin, mut stdout: RawTerminal<W>, runner: F)
    where W: Write,
          F: Fn(&String),
{
    // Load history from file in $HOME.
    let mut history = History::load();

    // A styled static (for now) prompt.
    let prompt = Prompt::new().nixpulvis_style();

    prompt.display(&mut stdout);

    let mut text = String::new();
    let mut cursor = 0usize;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
                // Load history from file in $HOME.
                history.save();
                exit(0)
            },
            Key::Char('\n') => {
                print!("\n\r");
                stdout.flush().unwrap();

                stdout.suspend_raw_mode().unwrap();
                history.add(&text);
                runner(&text);
                history.reset_index();
                stdout.activate_raw_mode().unwrap();

                // Reset the text for the next program.
                text.clear();

                // Reset the cursor.
                cursor = 0;

                // Print a boring static prompt.
                prompt.display(&mut stdout);
            },
            Key::Up => {
                print!("{}{}",
                       termion::clear::CurrentLine,
                       termion::cursor::Left(prompt.len() as u16));
                prompt.display(&mut stdout);
                if let Some(history_text) = history.get_up() {
                    cursor = history_text.len();
                    text = history_text;
                    print!("{}", text);
                }
                stdout.flush().unwrap();
            },
            Key::Down => {
                print!("{}{}",
                       termion::clear::CurrentLine,
                       termion::cursor::Left(prompt.len() as u16));
                prompt.display(&mut stdout);

                match history.get_down() {
                    Some(history_text) => {
                        cursor = history_text.len();
                        text = history_text;
                        print!("{}", text);
                    },
                    None => text = String::new(),
                }
                stdout.flush().unwrap();
            },
            Key::Left => {
                cursor = cursor.saturating_sub(1);
                print!("{}", termion::cursor::Left(1));
                stdout.flush().unwrap();
            },
            Key::Right => {
                cursor = cursor.saturating_add(1);
                print!("{}", termion::cursor::Right(1));
                stdout.flush().unwrap();
            },
            Key::Char(c) => {
                cursor = cursor.saturating_add(1);
                text.push(c);
                print!("{}", c);
                stdout.flush().unwrap();
            },
            Key::Backspace => {
                if !text.is_empty() {
                    cursor = cursor.saturating_sub(1);
                    print!("{}{}",
                           termion::cursor::Left(1),
                           termion::clear::UntilNewline);
                    text.remove(cursor);
                    print!("{}", &text[cursor..]);
                    print!("{}", termion::cursor::Left((text.len() - cursor) as u16));
                    stdout.flush().unwrap();
                }
            }
            Key::Ctrl('c') => {
                text.clear();
                print!("\n\r");
                prompt.display(&mut stdout);
            },
            _ => {}
        }
    }
}

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Prompt {
    pub const DEFAULT_FORMAT: &'static str = "$ ";

    pub fn new() -> Self {
        Prompt(format!("{}", Self::DEFAULT_FORMAT))
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
        Prompt(format!("{}{}${}{} ",
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

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct History(Option<usize>, Vec<(String, usize)>);

impl History {
    pub fn reset_index(&mut self) {
        self.0 = None;
    }

    pub fn add(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        // HACK: There's got to be a cleaner way.
        let mut index = 0;
        if self.1.iter().enumerate().find(|(i, (t, c))| {
            index = *i;
            text == t
        }).is_some() {
            self.1[index].1 += 1;
        } else {
            self.1.insert(0, (text.to_owned(), 0));
        }
    }

    pub fn get_up(&mut self) -> Option<String> {
        let text_len = self.1.len();
        if text_len > 0 {
            match self.0 {
                Some(i) => {
                    self.0 = Some(i.saturating_add(1)
                                   .min(text_len - 1));
                },
                None => self.0 = Some(0),
            }
        } else {
            self.0 = None;
        }

        match self.0 {
            Some(i) => Some(self.1[i].0.clone()),
            None => None,
        }
    }

    pub fn get_down(&mut self) -> Option<String> {
        match self.0 {
            Some(i) if i == 0 => self.0 = None,
            Some(i) => self.0 = Some(i.saturating_sub(1)),
            None => {},
        };

        match self.0 {
            Some(i) => Some(self.1[i].0.clone()),
            None => None,
        }
    }

    pub fn load() -> Self {
        let mut f = File::open("/home/nixpulvis/.oursh_history")
            .expect("error cannot find history");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("error reading history");

        let mut history = History(None, vec![]);
        for text in contents.split("\n").map(|s| String::from(s)) {
            history.add(&text);
        }
        history
    }

    pub fn save(&self) {
        let mut f = File::open("/home/nixpulvis/.oursh_history")
            .expect("error cannot find history");
        f.write(self.1.iter().map(|(t, c)| t.to_owned()).collect::<Vec<String>>().join("\n").as_bytes())
            .expect("error reading history");
    }
}
