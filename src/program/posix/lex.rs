use std::str::{self, CharIndices};

pub type Spanned<T, E> = Result<(usize, T, usize), E>;
pub type Span<'input> = Spanned<Tok<'input>, Error>;

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub enum Tok<'input> {
    Space,
    Tab,
    Linefeed,
    Amper,
    RBrace,
    LBrace,
    RParen,
    LParen,
    Backtick,
    Bang,
    Pipe,
    Dollar,
    Equals,
    Slash,
    Backslash,
    DoubleQuote,
    SingleQuote,
    RCaret,
    LCaret,
    And,
    Or,
    Word(&'input str),
    Shebang,
}

pub struct Lexer<'input> {
    /// The original text.
    input: &'input str,

    /// An iterator over all the characters of the input.
    chars: CharIndices<'input>,

    /// Always have the next (or first in the initial case) character
    /// of the input, allows for EOF detection, amongst other things.
    lookahead: Option<(usize, char)>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let lookahead = chars.next();
        Lexer { input, chars, lookahead }
    }
}

impl<'input> Lexer<'input> {
    pub fn advance(&mut self) -> Option<(usize, char)> {
        match self.lookahead {
            Some((i, t)) => {
                self.lookahead = self.chars.next();
                Some((i, t))
            },
            None => None,
        }
    }

    pub fn take_until<F>(&mut self, start: usize, mut terminate: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        let mut end = start + 1;
        while let Some((e, c)) = self.lookahead {
            if terminate(c) {
                return (e, &self.input[start..end]);
            } else {
                end = e + 1;
                self.advance();
            }
        }
        (end, &self.input[start..end])
    }

    fn take_while<F>(&mut self, start: usize, mut keep_going: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(start, |c| !keep_going(c))
    }

    pub fn word(&mut self, start: usize) -> Result<(usize, Tok<'input>, usize), Error> {
        let (mut end, mut ident) = self.take_while(start, is_ident_continue);
        Ok((start, Tok::Word(&self.input[start..end]), end))
    }

    pub fn shebang_block(&mut self, start: usize) -> Option<(usize, Tok<'input>, usize)> {
        let (end, line) = self.take_until(start, |ch| ch == '\n');

        if line.starts_with("{#!") {
            Some((start, Tok::Shebang, end))
        } else {
            None
        }
    }
}

fn is_ident_start(ch: char) -> bool {
    // TODO: Unicode?
    match ch {
        '-' | '_' | '.' |
        'a'...'z' | 'A'...'Z' |
        '0'...'9' => true,
        _ => false,
    }
}

fn is_ident_continue(ch: char) -> bool {
    // TODO: Unicode?
    match ch {
        '\'' => true,
        ch => is_ident_start(ch),
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.advance() {
            let tok = match c {
                '\n' => Some(Ok((i, Tok::Linefeed, i+1))),
                '}'  => Some(Ok((i, Tok::RBrace, i+1))),
                '{'  => Some(Ok((i, Tok::LBrace, i+1))),
                ')'  => Some(Ok((i, Tok::RParen, i+1))),
                '('  => Some(Ok((i, Tok::LParen, i+1))),
                '`'  => Some(Ok((i, Tok::Backtick, i+1))),
                '!'  => Some(Ok((i, Tok::Bang, i+1))),
                '$'  => Some(Ok((i, Tok::Dollar, i+1))),
                '='  => Some(Ok((i, Tok::Equals, i+1))),
                '/'  => Some(Ok((i, Tok::Slash, i+1))),
                '\\' => Some(Ok((i, Tok::Backslash, i+1))),
                '"'  => Some(Ok((i, Tok::DoubleQuote, i+1))),
                '\'' => Some(Ok((i, Tok::DoubleQuote, i+1))),
                '>'  => Some(Ok((i, Tok::RCaret, i+1))),
                '<'  => Some(Ok((i, Tok::LCaret, i+1))),
                '&' => {
                    if let Some((_, '&')) = self.lookahead {
                        self.advance();
                        Some(Ok((i, Tok::And, i+2)))
                    } else {
                        Some(Ok((i, Tok::Amper, i+1)))
                    }
                },
                '|' => {
                    if let Some((_, '|')) = self.lookahead {
                        self.advance();
                        Some(Ok((i, Tok::Or, i+2)))
                    } else {
                        Some(Ok((i, Tok::Pipe, i+1)))
                    }
                },
                // TODO: Compund syntax.
                '{' => {
                    match self.shebang_block(i) {
                        Some(token) => Some(Ok(token)),
                        None => continue,
                    }
                },
                // XXX: ident isn't even the correct name...
                c if is_ident_start(c) => {
                    let tok = self.word(i);
                    Some(tok)
                },
                c if is_whitespace(c) => {
                    continue;
                },
                c => panic!("unexpected char {:?}", c),
            };
            debug!("emit: {:?}", tok);
            return tok;
        }
        None
    }
}
