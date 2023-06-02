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
            return Err(String::from("utf8 code is not supported"));
        }
        Ok(Self {
            source: source.as_bytes().to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
        })
    }

    fn get_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let current = self.source[self.current];
            self.start = self.current;
            self.current += 1;
            match current {
                b'.' => self.tokens.push(Token::Dot),
                b'\\' => self.tokens.push(Token::Lambda),
                b'(' => self.tokens.push(Token::ParenLeft),
                b')' => self.tokens.push(Token::ParenRight),
                _ => {
                    if Self::is_name_character(current) {
                        self.push_name();
                    } else if !current.is_ascii_whitespace() {
                        panic!("unrecognized character");
                    }
                }
            };
        }
        self.tokens.push(Token::Eof);
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_name_character(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    fn push_name(&mut self) {
        while !self.is_at_end() && Self::is_name_character(self.source[self.current]) {
            self.current += 1;
        }

        // By this point the name is guaranteed to be ascii, and is
        // therefore valid UTF-8
        self.tokens.push(Token::Name(
            String::from_utf8(self.source[self.start..self.current].as_ref().to_owned()).unwrap(),
        ));
    }

    pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
        Ok(Lexer::new(source)?.get_tokens())
    }
}
