use std::process::exit;

use crate::ast::*;
use crate::lexer::*;

pub struct Parser {}

impl Parser {
    // order of precedence
    // additive
    // multiplicative
    // primary

    pub fn parse(content: &String) -> Program {
        let mut tokens = Token::tokenize(content);
        let mut program = Program { body: vec![] };

        while tokens.len() > 0 {
            let token = tokens.get(0).unwrap();

            if token.token_type == TokenType::EOF {
                break;
            }

            let expr = Parser::parse_additive_expr(&mut tokens);

            program.body.push(expr);
        }

        program
    }

    pub fn parse_additive_expr(tokens: &mut Vec<Token>) -> Expr {
        let mut left = Parser::parse_multiplicative_expr(tokens);

        while tokens[0].value.as_str() == "+" || tokens[0].value.as_str() == "-" {
            let operator = &tokens.remove(0);
            let right = Parser::parse_multiplicative_expr(tokens);
            let operator = BinOp::from_string(&operator.value);

            match operator {
                Some(value) => left = Expr::Binary(Box::new(left), value, Box::new(right)),
                None => {}
            }
        }

        left
    }

    pub fn parse_multiplicative_expr(tokens: &mut Vec<Token>) -> Expr {
        let mut left = Parser::parse_primary_expr(tokens);

        while tokens[0].value.as_str() == "*"
            || tokens[0].value.as_str() == "/"
            || tokens[0].value.as_str() == "%"
        {
            let operator = &tokens.remove(0);
            let right = Parser::parse_primary_expr(tokens);
            let operator = BinOp::from_string(&operator.value);

            match operator {
                Some(value) => left = Expr::Binary(Box::new(left), value, Box::new(right)),
                None => {}
            }
        }

        left
    }

    pub fn parse_primary_expr(tokens: &mut Vec<Token>) -> Expr {
        let token = tokens.remove(0);

        let expr = match token.token_type {
            TokenType::Identifier => Expr::Identifier(token.value.clone()),
            TokenType::Number => Expr::Number(token.value.parse::<i64>().unwrap()),
            TokenType::OpenParen => {
                let value = Parser::parse_additive_expr(tokens);
                let prev = tokens.remove(0);

                if prev.token_type != TokenType::CloseParen {
                    eprintln!("missing closing parentheses");
                    return Expr::Invalid;
                }

                value
            }
            _ => Expr::Invalid,
        };

        if expr == Expr::Invalid {
            eprintln!("invalid character");
            exit(1);
        }

        expr
    }
}
