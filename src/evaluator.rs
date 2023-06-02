use crate::parser::Expr;

pub struct Evaluator;

impl Evaluator {
    fn beta_reduce(expr: Expr, from: &str, to: Expr) -> Expr {
        match expr {
            Expr::Application(func_expr, arg_expr) => {
                Self::beta_reduce_application(*func_expr, *arg_expr, from, to)
            }
            Expr::Function(name, body) => Self::beta_reduce_function(name, *body, from, to),
            Expr::Name(name) => Self::beta_reduce_name(name, from, to),
        }
    }

    fn beta_reduce_application(func_expr: Expr, arg_expr: Expr, from: &str, to: Expr) -> Expr {
        Expr::Application(
            Box::new(Self::beta_reduce(func_expr, from, to.clone())),
            Box::new(Self::beta_reduce(arg_expr, from, to)),
        )
    }

    fn beta_reduce_function(name: String, body: Expr, from: &str, to: Expr) -> Expr {
        Expr::Function(name, Box::new(Self::beta_reduce(body, from, to)))
    }

    fn beta_reduce_name(name: String, from: &str, to: Expr) -> Expr {
        if name == from {
            return to;
        }
        Expr::Name(name)
    }

    pub fn eval(expr: Expr) -> Expr {
        match expr {
            Expr::Application(func_expr, arg_expr) => Self::eval_application(*func_expr, *arg_expr),
            Expr::Function(name, body) => Self::eval_function(name, *body),
            Expr::Name(name) => Self::eval_name(name),
        }
    }

    fn eval_application(func_expr: Expr, arg_expr: Expr) -> Expr {
        match func_expr {
            Expr::Application(f, a) => {
                // Might need to wrap this in an eval
                Expr::Application(Box::new(Self::eval_application(*f, *a)), Box::new(arg_expr))
            }
            Expr::Function(name, body) => Self::eval(Self::beta_reduce(*body, &name, arg_expr)),
            Expr::Name(_) => Expr::Application(Box::new(func_expr), Box::new(Self::eval(arg_expr))),
        }
    }

    fn eval_function(name: String, body: Expr) -> Expr {
        Expr::Function(name, Box::new(Self::eval(body)))
    }

    fn eval_name(name: String) -> Expr {
        Expr::Name(name)
    }
}
