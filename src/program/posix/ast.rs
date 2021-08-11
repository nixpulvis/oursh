//! Abstract Syntax Tree for the POSIX language.
use std::os::unix::io::RawFd;

/// A program is the result of parsing a sequence of commands.
#[derive(Debug, Clone)]
pub struct Program(pub Vec<Box<Command>>);

/// A command is a *highly* mutually-recursive node with the main features
/// of the POSIX language.
#[derive(Debug, Clone)]
pub enum Command {
    /// Just a single command, with it's arguments.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// date --iso-8601
    /// ```
    // TODO #8: Simple should not just be a vec of words.
    Simple(Vec<Assignment>, Vec<Word>, Vec<Redirect>),

    /// A full program embedded in a compound command.
    ///
    /// ```sh
    /// { ls ; }
    /// ```
    Compound(Vec<Box<Command>>),

    /// Performs boolean negation to the status code of the inner
    /// command.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// ! grep 'password' data.txt
    /// ```
    Not(Box<Command>),

    /// Perform the first command, conditionally running the next
    /// upon success.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// mkdir tmp && cd tmp
    /// ```
    And(Box<Command>, Box<Command>),

    /// Perform the first command, conditionally running the next
    /// upon failure.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// kill $1 || kill -9 $1
    /// ```
    Or(Box<Command>, Box<Command>),

    /// Run the inner **program** in a sub-shell environment.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// DATE=(date)
    /// ```
    Subshell(Box<Program>),

    /// Run a command's output through to the input of another.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// cat $1 | wc -l
    /// ```
    Pipeline(Box<Command>, Box<Command>),

    /// Run a command in the background.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// while true; do
    ///   sleep 1; echo "ping";
    /// done &
    /// ```
    Background(Box<Command>),

    /// Run a program through another parser/interpreter.
    ///
    /// ### Examples
    ///
    /// ```sh
    /// {#ruby puts (Math.sqrt(32**2/57.2))}
    /// ```
    ///
    /// ### Compatibility
    ///
    /// This is **non-POSIX**
    ///
    /// TODO: How bad is it?
    Lang(Interpreter, String),
}

/// A parsed word, already having gone through expansion.
// TODO #8: How can we expand things like $1 or $? from the lexer?
// TODO #8: This needs to handle escapes and all kinds of fun. We first
//       need to decide on our custom Tokens and lexer.
#[derive(Debug, Clone)]
pub enum Word {
    Bare(String),
    Quote(String, i8),
    Variable(String),
}
// BARE WORDS
// 'single quote'
// "double!"
// $FOO
// "this var $foo is expanded, while 'this nested single quote $bar' is not"

#[derive(Debug, Clone)]
pub enum Redirect {
    // Redirecting Input and Output
    // [n]<>word
    RW { n: RawFd, filename: String },
    // Redirecting Input
    // [n]<word  (duplicate = false)
    // [n]<&word (duplicate = true)
    Read {
        n: RawFd,
        filename: String,
        duplicate: bool,
    },
    // Redirecting Output
    // [n]>word  // TODO: clobber flag needed.
    // [n]>|word (clobber = true)
    // [n]>>word (append = true)
    // [n]>&word (duplicate = true)
    Write {
        n: RawFd,
        filename: String,
        duplicate: bool,
        clobber: bool,
        append: bool,
    },
    // // Here-Document
    // // [n]<<word
    // //     here-document
    // // delimiter (above word)
    // Here {
    //     n: RawFd,
    //     leading: bool,
    //     string: String,
    // },
}

impl Redirect {
    pub fn fd(&mut self) -> &mut RawFd {
        match self {
            Redirect::RW { ref mut n, .. } => n,
            Redirect::Read { ref mut n, .. } => n,
            Redirect::Write { ref mut n, .. } => n,
            // Redirect::Here { ref mut n, .. } => n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assignment(pub String, pub String);

impl Command {
    pub fn push(mut self, command: &Command) -> Self {
        match self {
            Command::Compound(ref mut c) => {
                c.push(box command.clone());
            },
            c @ _ => {
                return Command::Compound(vec![box c, box command.clone()]);
            },
        }

        self
    }

    pub fn insert(mut self, command: &Command) -> Self {
        match self {
            Command::Compound(ref mut c) => {
                c.insert(0, box command.clone());
            },
            c @ _ => {
                return Command::Compound(vec![box command.clone(), box c]);
            },
        }

        self
    }
}

/// Either explicit or implicit declaration of the interperator for
/// a bridged program.
///
/// ### Examples
///
/// ```sh
/// {# ...}
/// {#ruby ...}
/// ```
#[derive(Debug, Clone)]
pub enum Interpreter {
    Primary,
    Alternate,
    HashLang(String),
    Shebang(String),
}

impl Program {
    pub(crate) fn insert(mut self, command: &Command) -> Self {
        self.0.insert(0, box command.clone());
        self
    }

    pub(crate) fn append(mut self, program: &Program) -> Self {
        self.0.append(&mut program.0.iter().cloned().collect());
        self
    }
}


#[cfg(test)]
mod tests {
    use lalrpop_util::ParseError;
    use crate::program::posix::{
        parse::{ProgramParser, CommandParser},
        lex::{Lexer, Token, Error},
    };
    use super::*;

    fn parse_program<'a>(text: &'a str)
        -> Result<Program, ParseError<usize, Token<'a>, Error>>
    {
        let lexer = Lexer::new(text);
        let parser = ProgramParser::new();
        parser.parse(text, lexer)
    }

    #[test]
    fn program() {
        assert!(parse_program("").is_err());
        // TODO: Should this be Ok? len 0.
        assert_eq!(1, parse_program("cat README.md").unwrap().0.len());
        assert_eq!(1, parse_program("ls;").unwrap().0.len());
        assert_eq!(2, parse_program("ls; date").unwrap().0.len());
        assert_eq!(3, parse_program("git s; ls -la; true;").unwrap().0.len());
    }

    fn parse_command<'a>(text: &'a str)
        -> Result<Command, ParseError<usize, Token<'a>, Error>>
    {
        let lexer = Lexer::new(text);
        let parser = CommandParser::new();
        parser.parse(text, lexer)
    }

    #[test]
    fn simple_command() {
        // TODO: Just parse a command.
        assert!(parse_command("ls").is_ok());
        assert!(parse_command("git s").is_ok());
        assert!(parse_command("ls -la").is_ok());
    }

    #[test]
    fn compound_command() {
        assert!(parse_command("{ls}").is_err());
        assert!(parse_command("{ls; date}").is_err());

        let text = "{ls;}";
        let command = parse_command(text).unwrap();
        assert_matches!(&command, Command::Compound(c) if c.len() == 1);

        let text = "{ls; date;}";
        let command = parse_command(text).unwrap();
        assert_matches!(&command, Command::Compound(c) if c.len() == 2);

        let text = "{git s; ls -la; true;}";
        let command = parse_command(text).unwrap();
        assert_matches!(&command, Command::Compound(c) if c.len() == 3);
    }

    #[test]
    fn not_command() {
        let command = parse_command("! true").unwrap();
        assert_matches!(command, Command::Not(_));
        let command = parse_command("! true || false").unwrap();
        assert_matches!(command, Command::Or(box Command::Not(_),_));
    }

    #[test]
    fn and_command() {
        let command = parse_command("true && false").unwrap();
        assert_matches!(command, Command::And(_,_));
        let command = parse_command("true || false && true").unwrap();
        assert_matches!(command, Command::And(_,_));
    }

    #[test]
    fn or_command() {
        let command = parse_command("true || false").unwrap();
        assert_matches!(command, Command::Or(_,_));
        let command = parse_command("true && false || true").unwrap();
        assert_matches!(command, Command::Or(_,_));
    }

    #[test]
    fn subshell_command() {
        assert!(parse_command("()").is_err());

        let command = parse_command("(ls)").unwrap();
        assert_matches!(command, Command::Subshell(_));

        let command = parse_command("(date;)").unwrap();
        assert_matches!(command, Command::Subshell(_));

        let command = parse_command("(date; ls)").unwrap();
        assert_matches!(command, Command::Subshell(_));

        let command = parse_command("(date; ls -la;)").unwrap();
        assert_matches!(command, Command::Subshell(_));
    }
}
