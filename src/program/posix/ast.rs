//! Abstract Syntax Tree for the POSIX language.
use program::ast::Interpreter;

/// A program is the result of parsing a sequence of commands.
#[derive(Debug, Clone)]
pub struct Program(pub Vec<Box<Command>>);

/// A program's text and the interpreter to be used.
// TODO #8: Include grammar separate from interpreter?
#[derive(Debug, Clone)]
pub struct BridgedProgram(pub Interpreter, pub String);

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
    Simple(Vec<Word>),

    /// A full program embedded in a compound command.
    ///
    /// ```sh
    /// { ls ; }
    /// ```
    // TODO #10: We are currently overpermissive here.
    Compound(Box<Program>),

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
    Background(Box<Program>),

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
    Bridgeshell(Box<BridgedProgram>),
}

/// A parsed word, already having gone through expansion.
// TODO #8: How can we expand things like $1 or $? from the lexer?
// TODO #8: This needs to handle escapes and all kinds of fun. We first
//       need to decide on our custom Tokens and lexer.
#[derive(Debug, Clone)]
pub struct Word(pub String);


impl Program {
    pub(crate) fn push(mut self, command: &Command) -> Self {
        self.0.push(box command.clone());
        self
    }

    pub(crate) fn insert(mut self, command: &Command) -> Self {
        self.0.insert(0, box command.clone());
        self
    }

    pub(crate) fn append(mut self, program: &Program) -> Self {
        self.0.append(&mut program.0.iter().cloned().collect());
        self
    }
}
