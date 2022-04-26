// crate::boron::tokenizer

use std::fmt;

pub enum Token {
    Name (String),
    Int (i32),
    Float (f32),
    Bool (bool),
    Char (char),    
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenAngle,
    CloseAngle,
    TernaryIf,
    TernaryElse,
    Keyword (String),
    FnArgs,
    FnReturnType,
    Assignment,
    Operator (String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

#[allow(unused_variables)]
pub fn tokenize(tokens: &mut String) -> Vec<Token> {
    Vec::new()
}