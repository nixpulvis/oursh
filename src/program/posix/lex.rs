//! Custom LALRPOP lexer for tokenizing the input stream.
//!
//! ```
//! use oursh::program::posix::lex::Lexer;
//!
//! for span in Lexer::new("ls -la | wc") {
//!     let (start, token, end) = span.unwrap();
//!     println!("{:?}", token);
//! }
//! ```

use std::str::{self, CharIndices};

/// A result type wrapping a token with start and end locations.
pub type Span<T, E> = Result<(usize, T, usize), E>;

/// A lexer error.
#[derive(Debug)]
pub enum Error {
    UnrecognizedChar(usize, char, usize),
}

/// Every token in the langauge, these are the terminals of the grammar.
#[derive(Eq, PartialEq, Clone, Debug)]
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
    Great,
    DGreat,
    GreatAnd,
    Clobber,
    Less,
    DLess,
    DLessDash,
    LessAnd,
    LessGreat,
    And,
    Or,
    If,
    Then,
    Else,
    Elif,
    Fi,
    Do,
    Done,
    Case,
    Esac,
    While,
    Until,
    For,
    Word(&'input str),
    IoNumber(usize),
    Shebang(&'input str),
    Text(&'input str),
}

/// A lexer to feed the parser gernerated by LALRPOP.
pub struct Lexer<'input> {
    /// The original text.
    input: &'input str,

    /// An iterator over all the characters of the input.
    chars: CharIndices<'input>,

    /// Always have the next (or first in the initial case) character
    /// of the input, allows for EOF detection, amongst other things.
    lookahead: Option<(usize, char, usize)>,

    #[cfg(feature = "shebang-block")]
    /// A boolean indicating we're currently lexing inside a shebang block,
    /// and should therefor output TEXT.
    in_shebang: bool,
}

impl<'input> Lexer<'input> {
    /// Create a new lexer from an input &str.
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let next = chars.next();
        let lookahead = next.map(|n| (n.0, n.1, n.0 + n.1.len_utf8()));
        Lexer {
            input,
            chars,
            lookahead,
            #[cfg(feature = "shebang-block")]
            in_shebang: false,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "shebang-block")]
        {
            // If we're inside a shebang, parse a full TEXT block.
            if self.in_shebang {
                let tok = if let Some((start, _, end)) = self.lookahead {
                    Some(self.text(start, end))
                } else {
                    None
                };

                debug!("emit<end>: {:?}", tok);
                return tok;
            }
        }

        // Consume characters until we've got a token.
        while let Some((s, c, e)) = self.advance() {
            let tok = match c {
                '\n' => Some(Ok((s, Token::Linefeed, e))),
                ';'  => Some(Ok((s, Token::Semi, e))),
                ')'  => Some(Ok((s, Token::RParen, e))),
                '('  => Some(Ok((s, Token::LParen, e))),
                '`'  => Some(Ok((s, Token::Backtick, e))),
                '!'  => Some(Ok((s, Token::Bang, e))),
                '$'  => Some(Ok((s, Token::Dollar, e))),
                '='  => Some(Ok((s, Token::Equals, e))),
                '\\' => Some(Ok((s, Token::Backslash, e))),
                '\'' => Some(self.single_quote(s, e)),
                '"'  => Some(self.double_quote(s, e)),
                '>'  => {
                    match self.lookahead {
                        Some((_, '>', e)) => {
                            self.advance();
                            Some(Ok((s, Token::DGreat, e)))
                        },
                        Some((_, '&', e)) => {
                            self.advance();
                            Some(Ok((s, Token::GreatAnd, e)))
                        },
                        Some((_, '|', e)) => {
                            self.advance();
                            Some(Ok((s, Token::Clobber, e)))
                        },
                        _ => Some(Ok((s, Token::Great, e))),
                    }
                },
                '<'  => {
                    match self.lookahead {
                        Some((_, '&', e)) => {
                            self.advance();
                            Some(Ok((s, Token::LessAnd, e)))
                        },
                        Some((_, '<', e)) => {
                            self.advance();
                            if let Some((_, '-', e)) = self.lookahead {
                                self.advance();
                                Some(Ok((s, Token::DLessDash, e)))
                            } else {
                                Some(Ok((s, Token::DLess, e)))
                            }
                        },
                        Some((_, '>', e)) => {
                            self.advance();
                            Some(Ok((s, Token::LessGreat, e)))
                        },
                        _ => Some(Ok((s, Token::Less, e))),
                    }
                },
                '&' => {
                    if let Some((_, '&', e)) = self.lookahead {
                        self.advance();
                        Some(Ok((s, Token::And, e)))
                    } else {
                        Some(Ok((s, Token::Amper, e)))
                    }
                },
                '|' => {
                    if let Some((_, '|', e)) = self.lookahead {
                        self.advance();
                        Some(Ok((s, Token::Or, e)))
                    } else {
                        Some(Ok((s, Token::Pipe, e)))
                    }
                },
                '{' => Some(self.block(s, e)),
                '}' => Some(Ok((s, Token::RBrace, e))),
                c if is_word_start(c) => Some(self.word(s, e)),
                c if c.is_whitespace() => continue,
                c => return Some(Err(Error::UnrecognizedChar(s, c, e))),
            };
            debug!("emit<end>: {:?}", tok);
            return tok;
        }

        // Otherwise, return the EOF none.
        let tok = None;
        debug!("emit<end>: {:?}", tok);
        tok
    }
}

impl<'input> Lexer<'input> {
    fn advance(&mut self) -> Option<(usize, char, usize)> {
        match self.lookahead {
            Some((s, c, e)) => {
                debug!("emit<advance>: c: {} at {}-{}", c, s, e);
                let next = self.chars.next();
                self.lookahead = next.map(|n| (n.0, n.1, n.0 + n.1.len_utf8()));
                Some((s, c, e))
            },
            None => None,
        }
    }

    fn take_until<F>(&mut self, start: usize, mut end: usize,  mut terminate: F)
        -> (&'input str, usize)
        where F: FnMut(char) -> bool
    {
        while let Some((_, c, _)) = self.lookahead {
            if terminate(c) {
                return (&self.input[start..end], end);
            } else {
                if let Some((_, _, e)) = self.advance() {
                    end = e;
                }
            }
        }
        (&self.input[start..end], end)
    }

    fn take_while<F>(&mut self, start: usize, end: usize, mut keep_going: F)
        -> (&'input str, usize)
        where F: FnMut(char) -> bool,
    {
        self.take_until(start, end, |c| !keep_going(c))
    }

    fn single_quote(&mut self, start: usize, end: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        // TODO: This quitely stops at EOF.
        let (_, end) = self.take_while(start, end, |c| c != '\'');
        self.advance();  // Consume the ending single quote.
        Ok((start, Token::Word(&self.input[start+1..end]), end))
    }

    // TODO: Escapes
    // TODO: Should we expand $ variables here?
    fn double_quote(&mut self, start: usize, end: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        // TODO: This quitely stops at EOF.
        let (_, end) = self.take_while(start, end, |c| c != '"');
        self.advance();  // Consume the ending double quote.
        Ok((start, Token::Word(&self.input[start+1..end]), end))
    }

    fn word(&mut self, start: usize, end: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        let (word, end) = self.take_while(start, end, is_word_continue);
        let tok = match word {
            "if"    => Token::If,
            "then"  => Token::Then,
            "else"  => Token::Else,
            "elif"  => Token::Elif,
            "fi"    => Token::Fi,
            "do"    => Token::Do,
            "done"  => Token::Done,
            "case"  => Token::Case,
            "esac"  => Token::Esac,
            "while" => Token::While,
            "until" => Token::Until,
            "for"   => Token::For,
            word    => self.io_number(word),
        };

        Ok((start, tok, end))
    }

    fn io_number<'a>(&mut self, word: &'a str) -> Token<'a> {
        if let Some((_, c, _)) = self.lookahead {
            if c == '<' || c == '>' {
                if let Ok(n) = word.parse::<usize>() {
                    return Token::IoNumber(n);
                }
            }
        }

        Token::Word(word)
    }

    fn block(&mut self, start: usize, end: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        #[cfg(feature = "shebang-block")]
        {
            if let Some((_, '#', s)) = self.lookahead {
                self.advance();  // Consume the '#'.
                // TODO: Distinguish kinds of Shebang.
                if let Some((_, '!', s)) = self.lookahead {
                    let (_, end) = self.take_until(s, end, |c| c == ';');
                    self.advance();  // Consume the ';' delimeter.
                    self.take_while(end, end, |c| c.is_whitespace());
                    self.in_shebang = true;

                    let tok = Token::Shebang(&self.input[s..end]);
                    return Ok((start, tok, end));
                } else {
                    let (_, end) = self.take_until(s, end, char::is_whitespace);
                    self.in_shebang = true;

                    let tok = Token::Shebang(&self.input[s..end]);
                    return Ok((start, tok, end));
                }
            }
        }

        Ok((start, Token::LBrace, end))
    }

    #[cfg(feature = "shebang-block")]
    fn text(&mut self, start: usize, end: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        // TODO: Count matching braces in TEXT.
        let (_, end) = self.take_until(start, end, |d| d == '}');
        self.in_shebang = false;
        Ok((start, Token::Text(&self.input[start..end]), end))
    }
}

fn is_word_start(ch: char) -> bool {
    match ch {
        // Ignore C0 and C1 control character words.
        // TODO: Test
        '\u{007F}' |
        '\u{0000}'..='\u{001F}' |
        '\u{0080}'..='\u{009F}' => false,
        _ => is_word_continue(ch),
    }
}

fn is_word_continue(ch: char) -> bool {
    match ch {
        // List of syntax from above.
        // TODO: Make this list generated.
        ';' | ')' | '(' | '`' | '!' |
        '$' | '=' | '\\' | '\'' | '"' |
        '>' | '<' | '&' | '|' | '{' | '}' |
        '*' => false,

        _ => !ch.is_whitespace()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut lexer = Lexer::new("");
        assert!(lexer.next().is_none());
    }

    #[test]
    fn error() {
        let mut lexer = Lexer::new("*");
        assert_matches!(lexer.next(),
                        Some(Err(Error::UnrecognizedChar(_, '*', _))));
    }

    #[test]
    fn linefeed() {
        let mut lexer = Lexer::new("\n");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Linefeed, _))));
    }

    #[test]
    fn words() {
        let mut lexer = Lexer::new("ls -la");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("ls"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("-la"), _))));
        // Numbers are still words on their own.
        let mut lexer = Lexer::new("123");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("123"), _))));
    }

    #[test]
    fn redirects() {
        let mut lexer = Lexer::new(">");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Great, _))));
        let mut lexer = Lexer::new(">>");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::DGreat, _))));
        let mut lexer = Lexer::new(">&");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::GreatAnd, _))));
        let mut lexer = Lexer::new(">|");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Clobber, _))));

        let mut lexer = Lexer::new("<");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Less, _))));
        let mut lexer = Lexer::new("<<");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::DLess, _))));
        let mut lexer = Lexer::new("<<-");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::DLessDash, _))));
        let mut lexer = Lexer::new("<&");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::LessAnd, _))));
        let mut lexer = Lexer::new("<>");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::LessGreat, _))));
    }

    #[test]
    fn io_number() {
        let mut lexer = Lexer::new("ls -la 1> /dev/null");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("ls"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("-la"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::IoNumber(1), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Great, _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("/dev/null"), _))));
    }

    #[test]
    fn whitespace_words() {
        let mut lexer = Lexer::new("ls\n");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("ls"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Linefeed, _))));
    }

    #[test]
    fn unicode_words() {
        let mut lexer = Lexer::new("😀 -🧪");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("😀"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("-🧪"), _))));

        let mut lexer = Lexer::new("😀 -🧪💀");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("😀"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("-🧪💀"), _))));
    }

    #[test]
    fn keywords() {
        let mut lexer = Lexer::new("if ls done");
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::If, _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Word("ls"), _))));
        assert_matches!(lexer.next(),
                        Some(Ok((_, Token::Done, _))));
    }
}
