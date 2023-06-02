mod evaluator;
mod lexer;
mod parser;

use std::io::Write;

use lexer::Lexer;
use parser::Parser;

fn run(source: &str) {
    let tokens = Lexer::tokenize(source).unwrap();
    for token in &tokens {
        println!("{:?}", token);
    }
    println!();

    let tree = match Parser::parse(tokens) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e.message);
            return;
        }
    };
    println!("{:?}\n", tree);

    let result = evaluator::eval(tree);
    println!("{}", result);
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
