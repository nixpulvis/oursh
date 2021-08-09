use std::io::Write;
use pwd::Passwd;
use nix::unistd;
use termion::{style, color};

/// A status prompt to be displayed in interactive sessions before each
/// program.
pub struct Prompt(String);

impl Default for Prompt {
    fn default() -> Self {
        Prompt("$ ".into())
    }
}

impl Into<String> for Prompt {
    fn into(self) -> String {
        self.0
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
