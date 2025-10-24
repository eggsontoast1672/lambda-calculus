use lambda_calculus::{ast::Expr, eval};

#[test]
fn test_one_name() {
    let expr = Expr::Name("x");
    let result = eval::eval(expr);
    let expected = Expr::Name("x");

    assert_eq!(result, expected);
}

#[test]
fn test_nested_application() {
    let expr = Expr::Application(
        Box::new(Expr::Application(
            Box::new(Expr::Function(
                "x",
                Box::new(Expr::Function("y", Box::new(Expr::Name("x")))),
            )),
            Box::new(Expr::Name("one")),
        )),
        Box::new(Expr::Name("two")),
    );

    let result = eval::eval(expr);
    let expected = Expr::Name("one");

    assert_eq!(result, expected);
}
