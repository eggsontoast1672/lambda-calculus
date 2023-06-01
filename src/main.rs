#![allow(unused)]

mod lexer;
mod parser;

fn main() {
    let program = r#"\x.(x (x x))"#;
    // let program = r#"(\f.(f \x.x) \s.(s s))"#;

    let tokens = match lexer::tokenize(program) {
        Ok(t) => t,
        Err(s) => {
            println!("{}", s);
            std::process::exit(1);
        }
    };
    for token in &tokens {
        println!("{:?}", token);
    }

    println!("-------------------------------");

    let expr = parser::parse(tokens);
    println!("{:?}", expr);
}
