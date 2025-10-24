use crate::{ast::Expr, lexer::TokenKind};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "unexpected token"),
        }
    }
}

/// The Lambda Calculus parser.
///
/// This type wraps a stream of tokens in an interface for extracting valid syntactic expressions.
pub struct Parser<I> {
    tokens: I,
}

impl<I> Parser<I> {
    /// Create a new parser.
    fn new(tokens: I) -> Self {
        Self { tokens }
    }
}

impl<'a, I> Parser<I>
where
    I: Iterator<Item = TokenKind<'a>>,
{
    /// Pop a token from the input stack and return it.
    ///
    /// # Panics
    ///
    /// If the end of the token stream has been reached (past EOF), this function will panic. Since
    /// EOF should always be processed before the stream terminates, the end user should never
    /// observe this outcome.
    fn pop_token(&mut self) -> TokenKind<'a> {
        self.tokens.next().unwrap()
    }

    pub fn parse(tokens: I) -> Result<Expr<'a>, ParseError> {
        let mut parser = Self::new(tokens);
        let expr = parser.parse_expr();
        match parser.pop_token() {
            TokenKind::Eof => expr,
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    /// Parse and return the next expression in the stream.
    fn parse_expr(&mut self) -> Result<Expr<'a>, ParseError> {
        match self.pop_token() {
            TokenKind::Lambda => self.parse_expr_function(),
            TokenKind::Name(n) => Ok(Expr::Name(n)),
            TokenKind::ParenLeft => self.parse_expr_application(),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_expr_function(&mut self) -> Result<Expr<'a>, ParseError> {
        let name = match self.pop_token() {
            TokenKind::Name(n) => (*n).to_owned(),
            _ => return Err(ParseError::UnexpectedToken),
        };

        let token = self.pop_token();
        let TokenKind::Dot = token else {
            return Err(ParseError::UnexpectedToken);
        };

        let body = self.parse_expr()?;
        Ok(Expr::Function(name.into(), Box::new(body)))
    }
    
    fn parse_expr_application(&mut self) -> Result<Expr<'a>, ParseError> {
        let func_expr = self.parse_expr()?;
        let arg_expr = self.parse_expr()?;
        let token = self.pop_token();
        let TokenKind::ParenRight = token else {
            return Err(ParseError::UnexpectedToken);
        };
        Ok(Expr::Application(Box::new(func_expr), Box::new(arg_expr)))
    }
}
