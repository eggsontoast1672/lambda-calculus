#[derive(Clone, Debug)]
pub enum Expr {
    Application(Box<Expr>, Box<Expr>),
    Function(String, Box<Expr>),
    Name(String),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Application(function, argument) => write!(f, "({} {})", function, argument),
            Expr::Function(name, body) => write!(f, "\\{}.{}", name, body),
            Expr::Name(name) => write!(f, "{}", name),
        }
    }
}

pub fn debug_print(expr: &Expr) {
    match expr {
        Expr::Application(f, a) => {
            println!("Application");
            debug_print(f);
            debug_print(a);
        }

        Expr::Function(n, b) => {
            println!("Function");
            println!("{n}");
            debug_print(b);
        }

        Expr::Name(n) => {
            println!("Name");
            println!("{n}");
        }
    }
}
