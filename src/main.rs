use std::{
    io::{self, Write},
    process,
};

use lambda_calculus::lexer::Lexer;

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    match io::stdin().lines().next() {
        Some(line) => line,
        None => process::exit(0),
    }
}

fn run(source: &str) {
    let _tokens = Lexer::tokenize(source);
    // let tree = match Parser::parse(tokens) {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("{}", e);
    //         return;
    //     }
    // };

    // println!("{}", eval::eval(tree));
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
