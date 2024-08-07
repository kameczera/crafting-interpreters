use super::expr::*;

pub fn print(expr: Expr) -> String {
    match expr {
        Expr::Binary(expr) => visit_binary(expr),
        Expr::Grouping(expr) => visit_grouping(expr),
        Expr::Literal(expr) => visit_literal(expr),
        Expr::Unary(expr) => visit_unary(expr),
        Expr::Ternary(expr) => visit_ternary(expr),
    }
}

fn visit_binary(expr: Binary) -> String {
    return parenthesize(expr.operator.to_string_lexeme(), vec![expr.left, expr.right]);
}

fn visit_ternary(expr: Ternary) -> String {
    return write_ternary(expr.expression, expr.true_part, expr.false_part);
}

fn visit_grouping(expr: Grouping) -> String {
    return parenthesize(String::from("group"), vec![expr.expression]);
}

fn visit_literal(expr: Literal) -> String {
    if let Object::Nil = expr.value {
        return String::from("nil");
    }
    return expr.value.to_string();
}

fn visit_unary(expr: Unary) -> String {
    return parenthesize(expr.operator.to_string_lexeme(), vec![expr.right]);
}

fn parenthesize(name: String, exprs: Vec<Box<Expr>>) -> String {
    let mut builder: String = String::from("(");
    builder.push_str(&name);
    for expr in exprs {
        builder.push_str(" ");
        builder.push_str(&print(*expr));
    }
    builder.push_str(")");
    return builder;
}

fn write_ternary(expression: Box<Expr>, true_part: Box<Expr>, false_part: Box<Expr>) -> String {
    let mut builder: String = String::from("(");
    builder.push_str(&print(*expression));
    builder.push_str(" ? ");
    builder.push_str(&print(*true_part));
    builder.push_str(" : ");
    builder.push_str(&print(*false_part));
    builder.push_str(")");
    return builder;
}