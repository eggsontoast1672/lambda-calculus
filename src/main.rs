use std::io::Write;

use lambda_calculus::{ast, eval, lexer::Lexer, parser::Parser};

fn run(source: &str) {
    let tokens = Lexer::tokenize(source).unwrap();
    let tree = match Parser::parse(tokens) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e.message);
            return;
        }
    };

    println!("{}", eval::eval(tree));
}

fn main() {
    loop {
        let mut source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut source).unwrap();
        run(&source);
    }
}
