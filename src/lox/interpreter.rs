use super::{expr::*, token_type::TokenType, token::*, expr::Literal as Lit, token::Literal as TokenLiteral};

pub fn interpret(expr: Expr) -> Result<(), (Token, String)> {
    let value = evaluate(expr);
    match value {
        Ok(object) => {
            println!("{}", stringify(object));
            return Ok(());
        },
        Err(err) => return Err(err),
    }
}

fn evaluate(expr: Expr) -> Result<Object, (Token, String)> {
    match expr {
        Expr::Binary(expr) => visit_binary(expr),
        Expr::Grouping(expr) => visit_grouping(expr),
        Expr::Literal(expr) => Ok(visit_literal(expr)),
        Expr::Unary(expr) => visit_unary(expr),
        Expr::Ternary(expr) => visit_ternary(expr),
    }
}

fn visit_binary(expr: Binary) -> Result<Object, (Token, String)> {
    let left = match evaluate(*expr.left) {
        Ok(object) => object,
        Err(err) => return Err(err),
    };
    let right = match evaluate(*expr.right) {
        Ok(object) => object,
        Err(err) => return Err(err),
    };
    match expr.operator.token_type {
        TokenType::Minus => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Number(left - right))
        },
        TokenType::Slash => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Number(left / right))
        },
        TokenType::Star => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Number(left * right))
        },
        TokenType::Plus => {
            if let Object::String(mut left_value) = left {
                if let Object::String(right_value) = right {
                    left_value.push_str(&right_value);
                    return Ok(Object::String(left_value))
                }
            } else if let Object::Number(left_value) = left {
                if let Object::Number(right_value) = right {
                    return Ok(Object::Number(left_value + right_value))
                }
            }
            // throw new run_time_error
        },
        TokenType::Greater => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Boolean(left > right))
        },
        TokenType::GreaterEqual => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Boolean(left >= right))
        },
        TokenType::Less => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Boolean(left < right))
        },
        TokenType::LessEqual => {
            if let Err((token, string)) = check_number_binary(expr.operator, &left, &right) {
                return Err((token, string))
            }
            let left = Object::number(left);
            let right = Object::number(right);
            return Ok(Object::Boolean(left <= right))
        },
        TokenType::BangEqual => {return Ok(Object::Boolean(!is_equal(left, right)))},
        TokenType::EqualEqual => {return Ok(Object::Boolean(is_equal(left, right)))},
        _ => return Ok(Object::Nil),
    }
    // Unreachable
    return Err((Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None), String::from("")));
}

fn is_equal(a: Object, b: Object) -> bool {
    if a == Object::Nil && b == Object::Nil {
        return true
    } else if a == Object::Nil {
        return false
    }
    // TODO: confirm that this is working fine
    return a == b;
}

fn stringify(object: Object) -> String {
    if let Object::Nil = object {
        return String::from("nil")
    }

    if let Object::Number(number) = object {
        let text: String = number.to_string();
        if text.ends_with(".0") {
            return String::from(&text[0..text.len() - 2]);
        }
        return text;
    }
    return object.to_string();
}

fn visit_ternary(expr: Ternary) -> Result<Object, (Token, String)> {
    let expression = match evaluate(*expr.expression) {
        Ok(expr) => expr,
        Err(err) => return Err(err),
    };

    match expression {
        Object::Boolean(bool) => {
            if bool {
                let true_part = evaluate(*expr.true_part);
                return true_part
            } else {
                let false_part = evaluate(*expr.false_part);
                return false_part
            }
        }
        _ => (),
    }
    // Unreachable
    return Err((Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None), String::from("")));
}

fn visit_grouping(expr: Grouping) -> Result<Object, (Token, String)> {
    return evaluate(*expr.expression);
}

fn visit_literal(expr: Lit) -> Object {
    return expr.value;
}

fn visit_unary(expr: Unary) -> Result<Object, (Token, String)> {
    let mut right = match evaluate(*expr.right) {
        Ok(expr) => expr,
        Err(err) => return Err(err),
    };
    match expr.operator.token_type {
        TokenType::Minus => {
            check_number_operand(expr.operator, &right);
            if let Object::Number(number) = right {
                return Ok(Object::Number(-number));
            }
        },
        TokenType::Bang => {
            return Ok(Object::Boolean(!is_truthy(right)));
        }
        _ => (),
    }
    // Unreachable
    return Err((Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None), String::from("")));
}

fn check_number_operand(operator: Token, operand: &Object) -> Result<(), (Token, String)>{
    if let Object::Number(number) = operand {
        return Ok(());
    }
    return Err((operator, String::from("Operand must be a number.")))
}

fn check_number_binary(operator: Token, left: &Object, right: &Object) -> Result<(), (Token, String)>{
    if let Object::Number(_number) = left {
        if let Object::Number(_number) = right {
            return Ok(())
        }
    }
    return Err((operator, String::from("Operand must be numbers.")))
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Boolean(bool) => return bool,
        Object::Nil => return false,
        _ => return true,
    }
}