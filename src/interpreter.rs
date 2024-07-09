use crate::{ast::Expr, lexer::BinOpTks};

#[derive(Debug, PartialEq)]
pub enum RuntimeValues {
    Null,
    Number(i64),
    Error(String),
}

impl RuntimeValues {
    pub fn is_number(self: &Self) -> bool {
        if let RuntimeValues::Number(_value) = self {
            true
        } else {
            false
        }
    }

    pub fn get_number(self: &Self) -> Option<i64> {
        match self {
            RuntimeValues::Number(value) => Some(*value),
            _ => None,
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(expr: Expr) -> RuntimeValues {
        match expr {
            Expr::Number(value) => RuntimeValues::Number(value),
            Expr::Null => RuntimeValues::Null,
            Expr::Binary(lhs, operator, rhs) => Interpreter::eval_binary_expr(lhs, operator, rhs),
            _ => RuntimeValues::Error(
                "this ast node type hasn't been implemented in the interpreter".to_string(),
            ),
        }
    }

    pub fn eval_binary_expr(lhs: Box<Expr>, operator: BinOpTks, rhs: Box<Expr>) -> RuntimeValues {
        let lhs = Interpreter::evaluate(*lhs);
        let rhs = Interpreter::evaluate(*rhs);

        if lhs.is_number() && rhs.is_number() {
            Interpreter::eval_numeric_binary_expr(lhs, operator, rhs)
        } else {
            RuntimeValues::Error("both lhs and rhs must be of type number".to_string())
        }
    }

    pub fn eval_numeric_binary_expr(
        lhs: RuntimeValues,
        operator: BinOpTks,
        rhs: RuntimeValues,
    ) -> RuntimeValues {
        if !(lhs.is_number() && rhs.is_number()) {
            return RuntimeValues::Error("both lhs and rhs must be of type number".to_string());
        } else {
            let result;
            let lhs = lhs.get_number().unwrap();
            let rhs = rhs.get_number().unwrap();

            match operator {
                BinOpTks::Add => result = lhs.checked_add(rhs).unwrap(),
                BinOpTks::Subtract => result = lhs.checked_sub(rhs).unwrap(),
                BinOpTks::Multiply => result = lhs.checked_mul(rhs).unwrap(),
                BinOpTks::Divide => result = lhs.checked_div(rhs).unwrap(),
                BinOpTks::Modulo => result = lhs.checked_rem(rhs).unwrap(),
            };

            return RuntimeValues::Number(result);
        }
    }
}
