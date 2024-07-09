use crate::lexer::BinOpTks;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    Binary(Box<Expr>, BinOpTks, Box<Expr>),
    Null,
    Invalid,
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Expr>,
}
