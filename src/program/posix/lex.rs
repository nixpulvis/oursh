use std::iter::Peekable;
use std::str::{self, CharIndices};

pub type Spanned<T, E> = Result<(usize, T, usize), E>;
pub type Span<'input> = Spanned<Tok<'input>, Error>;

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
    Backslack,
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
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input: input,
            chars: input.char_indices().peekable(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Span<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        {
            let peek = self.chars.peek();
            println!("top peek: {:?}", peek);
            if let None = peek {
                println!("was none");
                return None;
            }
        }

        loop {
            match self.chars.next() {
                // Some((i, '#'))  => self.start_comment(),
                Some((i, ' ')) => {
                    println!("emit: Space");
                    return Some(Ok((i, Tok::Space, i+1)));
                },
                Some((i, '\t')) => {
                    println!("emit: Tab");
                    return Some(Ok((i, Tok::Tab, i+1)));
                },
                Some((i, '\n')) => {
                    println!("emit: Linefeed");
                    return Some(Ok((i, Tok::Linefeed, i+1)));
                },
                Some((i, c)) => {
                    println!("top: {:?}", c);
                    let mut k = i+1;

                    if let None = self.chars.peek() {
                        let tok = Tok::Word(&self.input[i..k]);
                        println!("emit {:?}", tok);
                        return Some(Ok((i, tok, k)));
                    }

                    loop {
                        match self.chars.next() {
                            // Some((i, '#'))  => self.start_comment(),
                            Some((j, ' '))  |
                            Some((j, '\t')) |
                            Some((j, '\n')) => {
                                println!("word: break whitespace");
                                let tok = Tok::Word(&self.input[i..k]);
                                println!("emit {:?}", tok);
                                return Some(Ok((i, tok, k)));
                            },
                            Some((j, c)) => {
                                println!("word: {:?}", c);
                                k += 1;
                            },
                            None => {
                                println!("word: break EOF");
                                let tok = Tok::Word(&self.input[i..k]);
                                println!("emit {:?}", tok);
                                return Some(Ok((i, tok, k)));
                            }
                        }
                    }
                },
                None => return None,
            }
        }
    }
}

impl<'input> Lexer<'input> {
    fn lex_word<'a: 'input>(&mut self,
                            i: usize,
                            l: char,
                            mut word: &'a mut [u8])
        -> Option<Span<'input>>
    {
        let mut j = i;
        loop {
            let next = self.chars.next();
            println!("{:?}", next);
            match next {
                // Some((i, '#'))  => self.start_comment(),
                Some((i, ' '))  |
                Some((i, '\t')) |
                Some((i, '\n')) => break,
                None => break,
                Some((i, c)) => {
                    println!("{}, {}", i, j);
                    word[j] = c as u8;
                    j = i;
                },
            }
        }
        Some(Ok((i, Tok::Word(str::from_utf8(&word[i..j]).unwrap()), j)))
    }
}
