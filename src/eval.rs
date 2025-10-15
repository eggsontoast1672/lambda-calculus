use crate::ast::Expr;

/// Replace all occurrences of the name `from` in `expr` with `to`.
///
/// Note that this function does not perform any evaluation. It simply makes the substitutions.
fn beta_reduce(expr: Expr, from: &str, to: Expr) -> Expr {
    match expr {
        Expr::Application(func_expr, arg_expr) => {
            let func_reduced = beta_reduce(*func_expr, from, to.clone());
            let arg_reduced = beta_reduce(*arg_expr, from, to);
            Expr::Application(Box::new(func_reduced), Box::new(arg_reduced))
        }

        Expr::Function(name, body) => {
            let body_reduced = beta_reduce(*body, from, to);
            Expr::Function(name, Box::new(body_reduced))
        }

        Expr::Name(name) if name == from => to,
        Expr::Name(_) => expr,
    }
}

pub fn eval(expr: Expr) -> Expr {
    match expr {
        Expr::Application(func_expr, arg_expr) => eval_application(*func_expr, *arg_expr),
        Expr::Function(name, body) => eval_function(name, *body),
        Expr::Name(name) => eval_name(name),
    }
}

fn eval_application(func_expr: Expr, arg_expr: Expr) -> Expr {
    match func_expr {
        Expr::Application(f, a) => {
            // Might need to wrap this in an eval
            Expr::Application(Box::new(eval_application(*f, *a)), Box::new(arg_expr))
        }
        Expr::Function(name, body) => eval(beta_reduce(*body, &name, arg_expr)),
        Expr::Name(_) => Expr::Application(Box::new(func_expr), Box::new(eval(arg_expr))),
    }
}

fn eval_function(name: String, body: Expr) -> Expr {
    Expr::Function(name, Box::new(eval(body)))
}

fn eval_name(name: String) -> Expr {
    Expr::Name(name)
}
