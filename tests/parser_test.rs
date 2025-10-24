use lambda_calculus::{
    ast::Expr,
    lexer::{Span, Token, TokenKind},
    parser::{ParseError, Parser},
};

#[test]
fn test_one_name() {
    let tokens = [
        Token::new(TokenKind::Name("hello"), Span::new(1, 1)),
        Token::new(TokenKind::Eof, Span::new(1, 6)),
    ];

    let ast = Parser::parse(tokens).unwrap();
    let expected = Expr::Name("hello");

    assert_eq!(ast, expected);
}

#[test]
fn test_one_function() {
    let tokens = [
        Token::new(TokenKind::Lambda, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Dot, Span::new(1, 3)),
        Token::new(TokenKind::Name("x"), Span::new(1, 4)),
        Token::new(TokenKind::Eof, Span::new(1, 5)),
    ];

    let ast = Parser::parse(tokens).unwrap();
    let expected = Expr::Function("x", Box::new(Expr::Name("x")));

    assert_eq!(ast, expected);
}

#[test]
fn test_nested_functions() {
    let tokens = [
        Token::new(TokenKind::Lambda, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Dot, Span::new(1, 3)),
        Token::new(TokenKind::Lambda, Span::new(1, 4)),
        Token::new(TokenKind::Name("y"), Span::new(1, 5)),
        Token::new(TokenKind::Dot, Span::new(1, 6)),
        Token::new(TokenKind::Name("x"), Span::new(1, 7)),
        Token::new(TokenKind::Eof, Span::new(1, 8)),
    ];

    let ast = Parser::parse(tokens).unwrap();
    let expected = Expr::Function(
        "x",
        Box::new(Expr::Function("y", Box::new(Expr::Name("x")))),
    );

    assert_eq!(ast, expected);
}

#[test]
fn test_application() {
    let tokens = [
        Token::new(TokenKind::ParenLeft, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Name("y"), Span::new(1, 4)),
        Token::new(TokenKind::ParenRight, Span::new(1, 5)),
        Token::new(TokenKind::Eof, Span::new(1, 6)),
    ];

    let ast = Parser::parse(tokens).unwrap();
    let expected = Expr::Application(Box::new(Expr::Name("x")), Box::new(Expr::Name("y")));

    assert_eq!(ast, expected);
}

#[test]
fn test_unfinished_application() {
    let tokens = [
        Token::new(TokenKind::ParenLeft, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Name("y"), Span::new(1, 4)),
        Token::new(TokenKind::Eof, Span::new(1, 5)),
    ];

    let err = Parser::parse(tokens).unwrap_err();
    match err {
        ParseError::UnexpectedToken(t) => {
            assert_eq!(t.kind, TokenKind::Eof);
            assert_eq!(t.span, Span::new(1, 5));
        }
    }
}
