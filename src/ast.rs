use crate::lexer::BinOp;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Invalid,
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Expr>,
}
