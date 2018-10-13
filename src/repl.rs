//! ...
//!
//! There will be *absolutely no* blocking STDIN/OUT/ERR on things like tab
//! completion or other potentially slow, or user defined behavior.

use nix::unistd;
use pwd::Passwd;
use std::io::Write;
use termion::{style, color};

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
