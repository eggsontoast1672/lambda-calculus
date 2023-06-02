use crate::parser::Expr;

fn beta_reduce(expr: Expr, bound: &str, what: Expr) -> Expr {
    match expr {
        Expr::Application(function, argument) => eval(Expr::Application(
            Box::new(beta_reduce(*function, bound, what.clone())),
            Box::new(beta_reduce(*argument, bound, what)),
        )),
        Expr::Function(name, body) => {
            Expr::Function(name, Box::new(beta_reduce(*body, bound, what)))
        }
        Expr::Name(name) => {
            if name == bound {
                what
            } else {
                Expr::Name(name)
            }
        }
    }
}

pub fn eval(expr: Expr) -> Expr {
    match expr {
        Expr::Application(func_expr, arg_expr) => match *func_expr {
            Expr::Application(_, _) => {
                eval(Expr::Application(Box::new(eval(*func_expr)), arg_expr))
            }
            Expr::Function(name, body) => beta_reduce(*body, &name, *arg_expr),
            Expr::Name(s) => Expr::Application(Box::new(Expr::Name(s)), Box::new(eval(*arg_expr))),
        },
        Expr::Function(name, body) => Expr::Function(name, body),
        Expr::Name(name) => Expr::Name(name),
    }
}
