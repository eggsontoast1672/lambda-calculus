use crate::lexer::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Application(Box<Expr>, Box<Expr>),
    Function(String, Box<Expr>),
    Name(String),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Application(function, argument) => write!(f, "({} {})", function, argument),
            Expr::Function(name, body) => write!(f, "\\{}.{}", name, body),
            Expr::Name(name) => write!(f, "{}", name),
        }
    }
}

pub struct ParserError {
    pub message: String,
}

impl ParserError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn get_token_and_advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParserError> {
        if tokens.len() == 0 {
            return Err(ParserError::new("expected expression, got nothing"));
        }
        Parser::new(tokens).parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        let name = match self.get_token_and_advance() {
            Token::Lambda => return self.parse_expr_function(),
            Token::Name(n) => n.clone(),
            Token::ParenLeft => return self.parse_expr_application(),
            token => {
                return Err(ParserError::new(&format!(
                    "expected one of LAMBDA, NAME, PAREN_LEFT, got {}",
                    token
                )))
            }
        };
        self.parse_expr_name(name)
    }

    fn parse_expr_application(&mut self) -> Result<Expr, ParserError> {
        let func_expr = self.parse_expr()?;
        let arg_expr = self.parse_expr()?;
        let token = self.get_token_and_advance();
        let Token::ParenRight = token else {
            return Err(ParserError::new(&format!("expected PAREN_RIGHT, got {}", token)));
        };
        Ok(Expr::Application(Box::new(func_expr), Box::new(arg_expr)))
    }

    fn parse_expr_function(&mut self) -> Result<Expr, ParserError> {
        let name = match self.get_token_and_advance() {
            Token::Name(n) => n.clone(),
            t => return Err(ParserError::new(&format!("expected NAME, got {}", t))),
        };
        let token = self.get_token_and_advance();
        let Token::Dot = token else {
            return Err(ParserError::new(&format!("expected DOT, got {}", token)));
        };
        let body = self.parse_expr()?;
        Ok(Expr::Function(name, Box::new(body)))
    }

    fn parse_expr_name(&self, name: String) -> Result<Expr, ParserError> {
        Ok(Expr::Name(name))
    }
}
