use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    Dot,
    Eof,
    Lambda,
    Name(String),
    ParenLeft,
    ParenRight,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Dot => write!(f, "DOT"),
            Token::Eof => write!(f, "EOF"),
            Token::Lambda => write!(f, "LAMBDA"),
            Token::Name(_) => write!(f, "NAME"),
            Token::ParenLeft => write!(f, "PAREN_LEFT"),
            Token::ParenRight => write!(f, "PAREN_RIGHT"),
        }
    }
}

/// Returns `true` if `c` is a character which can appear in a name, and `false` otherwise.
fn is_name_character(c: char) -> bool {
    c != '\\' && c != '.' && c != '(' && c != ')' && !c.is_whitespace()
}

/// The lambda calculus lexer.
///
/// This type combs through a stream of characters and generates a vector of tokens to be consumed
/// by the parser.
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            tokens: Vec::new(),
        }
    }

    fn get_tokens(mut self) -> Vec<Token> {
        while let Some(b) = self.pop_char() {
            match b {
                '.' => self.tokens.push(Token::Dot),
                '\\' => self.tokens.push(Token::Lambda),
                '(' => self.tokens.push(Token::ParenLeft),
                ')' => self.tokens.push(Token::ParenRight),
                _ => {
                    if is_name_character(b) {
                        self.push_name(b);
                    } else if !b.is_ascii_whitespace() {
                        panic!("unrecognized character");
                    }
                }
            }
        }

        self.tokens.push(Token::Eof);
        self.tokens
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn pop_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn push_name(&mut self, first: char) {
        let mut name2 = first.to_string();
        while let Some(c) = self.peek_char()
            && is_name_character(c)
        {
            self.pop_char();
            name2.push(c);
        }

        self.tokens.push(Token::Name(name2));
    }

    pub fn tokenize(source: &str) -> Vec<Token> {
        Lexer::new(source).get_tokens()
    }
}
