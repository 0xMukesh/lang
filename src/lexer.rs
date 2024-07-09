// lowkey x = 45 + (foo * bar)
// lowkey y = nada
// [LetToken, IdentifierToken(x), EqualToken, NumberToken(45), PlusToken, OpenParen, IdentifierToken(foo), MultiplyToken, IdentifierToken(bar)]

use crate::utils::{is_alpha, is_num, is_skippable, parse_num_literals, parse_str_literals};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    Null,

    Identifier,
    Number,

    Equal,
    BinaryOpr(BinOpTks),

    OpenParen,
    CloseParen,

    Invalid,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOpTks {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl BinOpTks {
    pub fn from_string(operator: &String) -> Option<Self> {
        match operator.as_str() {
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            "%" => Some(Self::Modulo),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub struct Keyword {
    identifier: String,
    token_type: TokenType,
}

impl Keyword {
    pub fn list() -> Vec<Keyword> {
        vec![
            Keyword {
                identifier: String::from("lowkey"),
                token_type: TokenType::Let,
            },
            Keyword {
                identifier: String::from("nada"),
                token_type: TokenType::Null,
            },
        ]
    }

    pub fn is_reserved(str: &String) -> (bool, Option<Keyword>) {
        let keywords = Self::list();

        for keyword in keywords {
            if &keyword.identifier == str {
                return (true, Some(keyword));
            }
        }

        return (false, None);
    }
}

impl Token {
    pub fn new(chars: &mut Vec<char>, token_type: TokenType) -> Self {
        Self {
            value: chars.remove(0).to_string(),
            token_type,
        }
    }

    pub fn tokenize(content: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut chars: Vec<char> = content.chars().collect();

        while chars.len() > 0 {
            let char = chars[0];
            let str = String::from(char);

            let token = match str.as_str() {
                "(" => Self::new(&mut chars, TokenType::OpenParen),
                ")" => Self::new(&mut chars, TokenType::CloseParen),
                "+" => Self::new(&mut chars, TokenType::BinaryOpr(BinOpTks::Add)),
                "-" => Self::new(&mut chars, TokenType::BinaryOpr(BinOpTks::Subtract)),
                "*" => Self::new(&mut chars, TokenType::BinaryOpr(BinOpTks::Multiply)),
                "/" => Self::new(&mut chars, TokenType::BinaryOpr(BinOpTks::Divide)),
                "%" => Self::new(&mut chars, TokenType::BinaryOpr(BinOpTks::Modulo)),
                "=" => Self::new(&mut chars, TokenType::Equal),
                _ => {
                    let is_alpha = is_alpha(&str);
                    let is_num = is_num(&str);
                    let is_skippable = is_skippable(&str);

                    if is_skippable {
                        chars.remove(0);
                        continue;
                    } else if is_num {
                        let num_literal = parse_num_literals(&mut chars);
                        Self {
                            value: num_literal,
                            token_type: TokenType::Number,
                        }
                    } else if is_alpha {
                        let str_literal = parse_str_literals(&mut chars);
                        let (is_reserved, keyword) = Keyword::is_reserved(&str_literal);

                        if is_reserved && keyword.is_some() {
                            Self {
                                value: str_literal,
                                token_type: keyword.unwrap().token_type,
                            }
                        } else {
                            Self {
                                value: str_literal,
                                token_type: TokenType::Identifier,
                            }
                        }
                    } else {
                        Self::new(&mut chars, TokenType::Invalid)
                    }
                }
            };

            tokens.push(token);
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            value: String::new(),
        });

        tokens
    }
}
