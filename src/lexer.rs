use std::{iter::Peekable, str::CharIndices};

#[derive(PartialEq, Debug)]
pub enum TokenKind<'a> {
    Dot,
    Eof,
    Lambda,
    Name(&'a str),
    ParenLeft,
    ParenRight,
}

#[derive(PartialEq, Debug)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(PartialEq, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    /// Create a new token from kind and location information.
    pub const fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Dot => write!(f, "DOT"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Lambda => write!(f, "LAMBDA"),
            TokenKind::Name(_) => write!(f, "NAME"),
            TokenKind::ParenLeft => write!(f, "PAREN_LEFT"),
            TokenKind::ParenRight => write!(f, "PAREN_RIGHT"),
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
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
    column: usize,
    dispensed_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            column: 1,
            dispensed_eof: false,
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        let p @ (_, c) = self.chars.next()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(p)
    }

    pub fn tokenize(source: &str) -> Vec<Token<'_>> {
        Lexer::new(source).collect()
    }

    /// Skip past all leading whitespace.
    ///
    /// This function sets the state of the lexer to point to the first non-whitespace character in
    /// the stream. It ensures that the line and column information are updated appropriately.
    fn skip_whitespace(&mut self) {
        while let Some((_, c)) = self.chars.peek()
            && c.is_whitespace()
        {
            self.next_char();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let span = Span::new(self.line, self.column);
        let Some((start_byte, c)) = self.next_char() else {
            return if self.dispensed_eof {
                None
            } else {
                self.dispensed_eof = true;
                Some(Token {
                    kind: TokenKind::Eof,
                    span,
                })
            };
        };

        let kind = match c {
            '\\' => TokenKind::Lambda,
            '(' => TokenKind::ParenLeft,
            ')' => TokenKind::ParenRight,
            '.' => TokenKind::Dot,

            _ if is_name_character(c) => {
                let mut end_byte = start_byte + c.len_utf8();
                while let Some(&(i, c2)) = self.chars.peek()
                    && is_name_character(c2)
                {
                    end_byte = i + c2.len_utf8();
                    self.next_char();
                }

                TokenKind::Name(&self.source[start_byte..end_byte])
            }

            // The idea is that any characters other than syntax and whitespace can be treated as
            // part of an identifier, so theoretically there should not be any characters that the
            // lexer won't accept.
            _ => unreachable!(),
        };

        Some(Token { kind, span })
    }
}
