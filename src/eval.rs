use crate::ast::Expr;

/// Replace all occurrences of the name `from` in `expr` with `to`.
///
/// Note that this function does not perform any evaluation. It simply makes the substitutions.
fn beta_reduce<'a>(expr: Expr<'a>, from: &str, to: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Name(name) if name == from => to,
        Expr::Name(_) => expr,

        Expr::Function(name, body) => {
            let body_reduced = beta_reduce(*body, from, to);
            Expr::Function(name, Box::new(body_reduced))
        }

        Expr::Application(func_expr, arg_expr) => {
            let func_reduced = beta_reduce(*func_expr, from, to.clone());
            let arg_reduced = beta_reduce(*arg_expr, from, to);
            Expr::Application(Box::new(func_reduced), Box::new(arg_reduced))
        }
    }
}

pub fn eval(expr: Expr) -> Expr {
    match expr {
        Expr::Name(_) => expr,
        Expr::Function(name, body) => Expr::Function(name, Box::new(eval(*body))),
        Expr::Application(left, right) => eval_application(*left, *right),
    }
}

fn eval_application<'a>(left: Expr<'a>, right: Expr<'a>) -> Expr<'a> {
    match left {
        Expr::Name(_) => Expr::Application(Box::new(left), Box::new(eval(right))),
        Expr::Function(name, body) => eval(beta_reduce(*body, &name, right)),
        Expr::Application(l, r) => {
            // In this case, we evaluate recursively until we encounter an application whose left
            // expression is a name.
            let left_prime = eval_application(*l, *r);
            eval_application(left_prime, right)
        }
    }
}
