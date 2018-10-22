use std::str::{self, CharIndices};

/// A result type wrapping a token with start and end locations.
pub type Span<T, E> = Result<(usize, T, usize), E>;

/// A lexer error.
// TODO: Expand this.
#[derive(Debug)]
pub struct Error;

/// Every token in the langauge, these are the terminals of the grammar.
#[derive(Debug)]
pub enum Token<'input> {
    Space,
    Tab,
    Linefeed,
    Semi,
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

/// A lexer to feed the parser gernerated by LALRPOP.
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
    /// Create a new lexer from an input &str.
    // TODO: Try taking/using `utf8::BufReadDecoder`.
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let lookahead = chars.next();
        Lexer { input, chars, lookahead }
    }
}

impl<'input> Lexer<'input> {
    fn advance(&mut self) -> Option<(usize, char)> {
        match self.lookahead {
            Some((i, t)) => {
                self.lookahead = self.chars.next();
                Some((i, t))
            },
            None => None,
        }
    }

    fn take_until<F>(&mut self, start: usize, mut terminate: F) -> (usize, &'input str)
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

    fn word(&mut self, start: usize) -> Result<(usize, Token<'input>, usize), Error> {
        let (end, _) = self.take_while(start, is_word_continue);
        Ok((start, Token::Word(&self.input[start..end]), end))
    }

    fn block(&mut self, start: usize) -> Result<(usize, Token<'input>, usize), Error> {
        if let Some((_, '#')) = self.lookahead {
            self.advance();  // Move past the matched '#'.
            // // TODO: Matching '}' detection.
            // let (end, interp) = self.take_until(start, |c| c == '!' || c == '\n');
            // debug!("{:?}, {:?}", end, interp);

            // debug!("before {:?}", self.lookahead);
            // self.advance();  // Move past the delim.
            // debug!("after {:?}", self.lookahead);

            // TODO: Distinguish kinds of Shebang.
            if let Some((_, '!')) = self.lookahead {
                self.advance();  // Move past the matched '!'.
                Ok((start, Token::Shebang, start+3))
            } else {
                Ok((start, Token::Shebang, start+2))
            }
        } else {
            Ok((start, Token::LBrace, start+1))
        }
    }
}

fn is_word_start(ch: char) -> bool {
    // TODO: Unicode?
    match ch {
        '-' | '_' | '.' | ':' | '/' |
        'a'...'z' | 'A'...'Z' |
        '0'...'9' => true,
        _ => false,
    }
}

fn is_word_continue(ch: char) -> bool {
    // TODO: Unicode?
    match ch {
        '\'' => true,
        ch => is_word_start(ch),
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("starting at: {:?}", self.lookahead);
        while let Some((i, c)) = self.advance() {
            let tok = match c {
                '\n' => Some(Ok((i, Token::Linefeed, i+1))),
                ';'  => Some(Ok((i, Token::Semi, i+1))),
                ')'  => Some(Ok((i, Token::RParen, i+1))),
                '('  => Some(Ok((i, Token::LParen, i+1))),
                '`'  => Some(Ok((i, Token::Backtick, i+1))),
                '!'  => Some(Ok((i, Token::Bang, i+1))),
                '$'  => Some(Ok((i, Token::Dollar, i+1))),
                '='  => Some(Ok((i, Token::Equals, i+1))),
                '\\' => Some(Ok((i, Token::Backslash, i+1))),
                '"'  => Some(Ok((i, Token::DoubleQuote, i+1))),
                '\'' => Some(Ok((i, Token::DoubleQuote, i+1))),
                '>'  => Some(Ok((i, Token::RCaret, i+1))),
                '<'  => Some(Ok((i, Token::LCaret, i+1))),
                '&' => {
                    if let Some((_, '&')) = self.lookahead {
                        self.advance();
                        Some(Ok((i, Token::And, i+2)))
                    } else {
                        Some(Ok((i, Token::Amper, i+1)))
                    }
                },
                '|' => {
                    if let Some((_, '|')) = self.lookahead {
                        self.advance();
                        Some(Ok((i, Token::Or, i+2)))
                    } else {
                        Some(Ok((i, Token::Pipe, i+1)))
                    }
                },
                '{' => Some(self.block(i)),
                '}' => Some(Ok((i, Token::RBrace, i+1))),
                c if is_word_start(c) => {
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
