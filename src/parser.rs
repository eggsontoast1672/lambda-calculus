use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Application(Box<Expr>, Box<Expr>),
    Function(String, Box<Expr>),
    Name(String),
}

struct Parser {
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

    fn parse_expr(&mut self) -> Expr {
        match self.get_token_and_advance() {
            Token::Lambda => Expr::Function(
                match self.get_token_and_advance() {
                    Token::Name(s) => {
                        let name = s.clone();
                        let Token::Dot = self.get_token_and_advance() else {
                            panic!("malformed function expression");
                        };
                        name
                    }
                    _ => panic!("malformed function expression"),
                },
                Box::new(self.parse_expr()),
            ),
            Token::Name(s) => Expr::Name(s.clone()),
            Token::ParenLeft => {
                let function = self.parse_expr();
                let argument = self.parse_expr();
                let Token::ParenRight = self.get_token_and_advance() else {
                    panic!("malformed application expression");
                };
                Expr::Application(Box::new(function), Box::new(argument))
            }
            _ => panic!("malformed expression"),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    Parser::new(tokens).parse_expr()
}
