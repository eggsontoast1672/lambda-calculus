use lambda_calculus::lexer::{Lexer, Span, Token, TokenKind};

#[test]
fn test_name() {
    let source = "<some-name-69420>";
    let tokens = Lexer::tokenize(source);
    let expected = vec![
        Token::new(TokenKind::Name("<some-name-69420>"), Span::new(1, 1)),
        Token::new(TokenKind::Eof, Span::new(1, 18)),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_function() {
    let source = "\\x.x";
    let tokens = Lexer::tokenize(source);
    let expected = vec![
        Token::new(TokenKind::Lambda, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Dot, Span::new(1, 3)),
        Token::new(TokenKind::Name("x"), Span::new(1, 4)),
        Token::new(TokenKind::Eof, Span::new(1, 5)),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_application() {
    let source = "(x y)";
    let tokens = Lexer::tokenize(source);
    let expected = vec![
        Token::new(TokenKind::ParenLeft, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Name("y"), Span::new(1, 4)),
        Token::new(TokenKind::ParenRight, Span::new(1, 5)),
        Token::new(TokenKind::Eof, Span::new(1, 6)),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_unfinished_application() {
    let source = "(x y";
    let tokens = Lexer::tokenize(source);
    let expected = vec![
        Token::new(TokenKind::ParenLeft, Span::new(1, 1)),
        Token::new(TokenKind::Name("x"), Span::new(1, 2)),
        Token::new(TokenKind::Name("y"), Span::new(1, 4)),
        Token::new(TokenKind::Eof, Span::new(1, 5)),
    ];

    assert_eq!(tokens, expected);
}
