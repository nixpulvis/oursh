use std::env;
use std::io::{Write, Stdout};
use pwd::Passwd;
use nix::unistd;
use termion::{style, color};

pub const NAME: &str = "oursh";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn ps1(stdout: &mut Stdout) {
    let prompt = expand_prompt(env::var("PS1").unwrap_or_else(|_| "\\s-\\v\\$ ".into()));
    write!(stdout, "{}", prompt).unwrap();
    stdout.flush().unwrap();
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
                    let cstr = unistd::gethostname(&mut buf).expect("error getting hostname");
                    cstr.to_str().expect("error invalid UTF-8").into()
                }
                'e' => (0x1b as char).into(),
                'u' => env::var("USER").unwrap_or_else(|_| "".into()),
                'w' => env::var("PWD").unwrap_or_else(|_| "".into()),
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

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Default for Prompt {
    fn default() -> Self {
        Prompt("$ ".into())
    }
}

impl Prompt {
    pub fn new(prompt: &str) -> Self {
        Prompt(format!("{}", prompt))
    }

    pub fn sh_style() -> Self {
        const NAME: &'static str = "oursh";
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        let version = &VERSION[0..(VERSION.len() - 2)];
        // TODO: Add a flag for pretending to be `sh`.
        // let name = "sh";
        // let version = "4.4";
        Prompt(format!("{}-{}$ ", NAME, version))
    }

    pub fn nixpulvis_style() -> Self {
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

    pub fn long_style() -> Self {
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

    pub fn short_style() -> Self {
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
