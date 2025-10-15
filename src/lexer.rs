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

pub struct Lexer {
    current: usize,
    source: Vec<u8>,
    start: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(source: &str) -> Result<Self, String> {
        if !source.is_ascii() {
            Err(String::from("utf8 code is not supported"))
        } else {
            Ok(Self {
                source: source.as_bytes().to_owned(),
                tokens: Vec::new(),
                start: 0,
                current: 0,
            })
        }
    }

    fn get_tokens(mut self) -> Vec<Token> {
        while let Some(b) = self.pop_byte() {
            self.start = self.current - 1;

            match b {
                b'.' => self.tokens.push(Token::Dot),
                b'\\' => self.tokens.push(Token::Lambda),
                b'(' => self.tokens.push(Token::ParenLeft),
                b')' => self.tokens.push(Token::ParenRight),
                _ => {
                    if Self::is_name_character(b) {
                        self.push_name();
                    } else if !b.is_ascii_whitespace() {
                        panic!("unrecognized character");
                    }
                }
            }
        }

        self.tokens.push(Token::Eof);
        self.tokens
    }

    fn peek_byte(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    fn pop_byte(&mut self) -> Option<u8> {
        if let Some(b) = self.peek_byte() {
            self.current += 1;
            Some(b)
        } else {
            None
        }
    }

    fn is_name_character(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    fn push_name(&mut self) {
        while let Some(b) = self.peek_byte()
            && Self::is_name_character(b)
        {
            self.pop_byte();
        }

        // SAFETY: The constructor for the lexer guarantees that `self.source` contains ascii data,
        // so any subsequence of bytes will also be valid ascii and therefore valid UTF8.
        let name_bytes = self.source[self.start..self.current].to_owned();
        let name = unsafe { String::from_utf8_unchecked(name_bytes) };
        self.tokens.push(Token::Name(name));
    }

    pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
        Lexer::new(source).map(|lexer| lexer.get_tokens())
    }
}
