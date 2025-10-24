#[derive(Clone, PartialEq, Debug)]
pub enum Expr<'a> {
    Name(&'a str),
    Function(&'a str, Box<Expr<'a>>),
    Application(Box<Expr<'a>>, Box<Expr<'a>>),
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Name(name) => write!(f, "{}", name),
            Expr::Function(name, body) => write!(f, "\\{}.{}", name, body),
            Expr::Application(function, argument) => write!(f, "({} {})", function, argument),
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
