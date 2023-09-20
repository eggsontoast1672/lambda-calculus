#![allow(unused)]

use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[ \n\r\t]+")]
enum Token {
    #[token(".")]
    Dot,
    #[token("\\")]
    Lambda,
    #[token("[A-Za-z]+")]
    Name,
    #[token("(")]
    ParenLeft,
    #[token(")")]
    ParenRight,
}
