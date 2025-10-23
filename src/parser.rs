use std::{
    error,
    fmt::{self, Display, Formatter},
};

use crate::{ast::Expr, lexer::TokenKind};

#[derive(Debug)]
pub enum PE {
    UnexpectedToken,
}

impl error::Error for PE {}

impl Display for PE {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PE::UnexpectedToken => write!(f, "unexpected token")
        }
    }
}

// pub struct ParserError {
//     pub message: String,
// }
// 
// impl ParserError {
//     fn new(message: &str) -> Self {
//         Self {
//             message: message.to_owned(),
//         }
//     }
// }

pub struct Parser<'a> {
    current: usize,
    tokens: Vec<TokenKind<'a>>,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    ///
    /// The internal token stream must have an end-of-file token at the end, so this function will
    /// return [`None`] if that token is not present.
    fn _new(tokens: Vec<TokenKind<'a>>) -> Option<Self> {
        match tokens.last() {
            // SAFETY: By taking this branch, we have already asserted that there is a
            // [`Token::Eof`] at the end of the stream.
            Some(TokenKind::Eof) => unsafe { Some(Self::new_unchecked(tokens)) },
            _ => None,
        }
    }

    /// Create a new parser without checking for the terminating [`Token::Eof`].
    ///
    /// # Safety
    ///
    /// This function requires that the last token of the `tokens` vector is a [`Token::Eof`]
    /// variant. If this is not the case, then many of the parsing functions will panic.
    unsafe fn new_unchecked(tokens: Vec<TokenKind<'a>>) -> Self {
        Self { current: 0, tokens }
    }

    /// Pop a token from the input stack and return it.
    ///
    /// # Panics
    ///
    /// If the end of the token stream has been reached (past EOF), this function will panic. Since
    /// EOF should always be processed before the stream terminates, the end user should never
    /// observe this outcome.
    fn pop_token(&mut self) -> &TokenKind<'_> {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    pub fn parse(mut tokens: Vec<TokenKind<'a>>) -> Result<Expr, PE> {
        if !matches!(tokens.last(), Some(TokenKind::Eof)) {
            tokens.push(TokenKind::Eof);
        }

        // SAFETY: The if-statement above ensures that the token stream ends with an end of file
        // token.
        let mut parser = unsafe { Self::new_unchecked(tokens) };
        let expr = parser.parse_expr();
        match parser.pop_token() {
            TokenKind::Eof => expr,
            _ => Err(PE::UnexpectedToken),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, PE> {
        match self.pop_token() {
            TokenKind::Lambda => self.parse_expr_function(),
            TokenKind::Name(n) => {
                let name = (*n).to_owned();
                self.parse_expr_name(name)
            }
            TokenKind::ParenLeft => self.parse_expr_application(),
            _ => Err(PE::UnexpectedToken),
        }
    }

    fn parse_expr_application(&mut self) -> Result<Expr, PE> {
        let func_expr = self.parse_expr()?;
        let arg_expr = self.parse_expr()?;
        let token = self.pop_token();
        let TokenKind::ParenRight = token else {
            return Err(PE::UnexpectedToken);
        };
        Ok(Expr::Application(Box::new(func_expr), Box::new(arg_expr)))
    }

    fn parse_expr_function(&mut self) -> Result<Expr, PE> {
        let name = match self.pop_token() {
            TokenKind::Name(n) => (*n).to_owned(),
            _ => return Err(PE::UnexpectedToken),
        };

        let token = self.pop_token();
        let TokenKind::Dot = token else {
            return Err(PE::UnexpectedToken);
        };

        let body = self.parse_expr()?;
        Ok(Expr::Function(name.into(), Box::new(body)))
    }

    fn parse_expr_name(&self, name: String) -> Result<Expr, PE> {
        Ok(Expr::Name(name))
    }
}
