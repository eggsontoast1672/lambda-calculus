use std::{
    io::{self, Write},
    process,
};

use lambda_calculus::{eval, lexer::Lexer, parser::Parser};

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().lines().next().unwrap_or_else(|| {
        println!();
        process::exit(0);
    })
}

fn run(source: &str) {
    let tokens = Lexer::tokenize(source);
    match Parser::parse(tokens) {
        Ok(expr) => println!("{}", eval::eval(expr)),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    loop {
        let Ok(source) = read_line("> ") else {
            println!("An I/O error occurred");
            continue;
        };

        run(&source);
    }
}
