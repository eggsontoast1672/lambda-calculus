mod evaluator;
mod lexer;
mod new_lexer;
mod parser;

use std::io::Write;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn run(source: &str) {
    let tokens = Lexer::tokenize(source).unwrap();
    let tree = match Parser::parse(tokens) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e.message);
            return;
        }
    };
    println!("{}", Evaluator::eval(tree));
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
