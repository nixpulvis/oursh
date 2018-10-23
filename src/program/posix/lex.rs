use std::str::{self, CharIndices};

/// A result type wrapping a token with start and end locations.
pub type Span<T, E> = Result<(usize, T, usize), E>;

/// A lexer error.
#[derive(Debug)]
pub enum Error {
    UnrecognizedChar(usize, char),
}

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
    lookahead: Option<(usize, char)>,

    #[cfg(feature = "bridge")]
    /// A boolean indicating we're currently lexing inside a shebang block,
    /// and should therefor output TEXT.
    in_shebang: bool,
}

impl<'input> Lexer<'input> {
    /// Create a new lexer from an input &str.
    // TODO: Try taking/using `utf8::BufReadDecoder`.
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let lookahead = chars.next();
        Lexer {
            input,
            chars,
            lookahead,
            #[cfg(feature = "bridge")]
            in_shebang: false,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<Token<'input>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "bridge")]
        {
            // If we're inside a shebang, parse a full TEXT block.
            if self.in_shebang {
                if let Some((start, _)) = self.lookahead {
                    let tok = Some(self.text(start));
                    debug!("emit<end>:   {:?}", tok);
                    return tok;
                } else {
                    return None
                }
            }
        }

        // Consume characters until we've got a token.
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
                '\'' => Some(self.single_quote(i)),
                '"'  => Some(self.double_quote(i)),
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
                c if is_word_start(c) => Some(self.word(i)),
                c if is_whitespace(c) => continue,
                c => return Some(Err(Error::UnrecognizedChar(i, c))),
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
    fn advance(&mut self) -> Option<(usize, char)> {
        match self.lookahead {
            Some((i, c)) => {
                debug!("emit<advance>: {}: {}", i, c);
                self.lookahead = self.chars.next();
                Some((i, c))
            },
            None => None,
        }
    }

    fn take_until<F>(&mut self, start: usize, mut terminate: F)
        -> (usize, &'input str)
        where F: FnMut(char) -> bool
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

    fn take_while<F>(&mut self, start: usize, mut keep_going: F)
        -> (usize, &'input str)
        where F: FnMut(char) -> bool,
    {
        self.take_until(start, |c| !keep_going(c))
    }

    // TODO: Escapes
    fn single_quote(&mut self, start: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        let (end, string) = self.take_while(start, |c| c != '\'');
        self.advance();  // Consume the ending single quote.
        Ok((start, Token::Word(&self.input[start+1..end]), end))
    }

    // TODO: Escapes
    // TODO: Should we expand $ variables here?
    fn double_quote(&mut self, start: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        let (end, string) = self.take_while(start, |c| c != '"');
        self.advance();  // Consume the ending double quote.
        Ok((start, Token::Word(&self.input[start+1..end]), end))
    }

    fn word(&mut self, start: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        let (end, word) = self.take_while(start, is_word_continue);
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
            w       => Token::Word(w),
        };

        Ok((start, tok, end))
    }

    fn block(&mut self, start: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        #[cfg(feature = "bridge")]
        {
            if let Some((_, '#')) = self.lookahead {
                let (end, _) = self.take_until(start, |c| c == ';');
                self.advance();  // Consume the ';' delimeter.
                self.take_while(end, is_whitespace);
                self.in_shebang = true;

                // TODO: Distinguish kinds of Shebang.
                if let Some((_, '!')) = self.lookahead {
                    let tok = Token::Shebang(&self.input[(start+4)..end]);
                    return Ok((start, tok, end));
                } else {
                    let tok = Token::Shebang(&self.input[(start+3)..end]);
                    return Ok((start, tok, end));
                }
            }
        }

        Ok((start, Token::LBrace, start+1))
    }

    #[cfg(feature = "bridge")]
    fn text(&mut self, start: usize)
        -> Result<(usize, Token<'input>, usize), Error>
    {
        // TODO: Count matching braces in TEXT.
        let (end, _) = self.take_until(start, |d| d == '}');
        self.in_shebang = false;
        Ok((start, Token::Text(&self.input[start..end]), end))
    }
}

// TODO: Unicode?
fn is_word_start(ch: char) -> bool {
    match ch {
        '-' | '_' | '.' | ':' | '/' |
        'a'...'z' | 'A'...'Z' |
        '0'...'9' => true,
        _ => false,
    }
}

// TODO: Unicode?
fn is_word_continue(ch: char) -> bool {
    match ch {
        '\'' => true,
        ch => is_word_start(ch),
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}
