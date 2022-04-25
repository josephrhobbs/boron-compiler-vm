// crate::boron::tokenizer

pub enum Token {
    Name (String),
    Int (i32),
    Float (f32),
    Bool (bool),
    Char (char),    
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
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

#[allow(unused_variables)]
pub fn tokenize(tokens: &mut String) -> Vec<Vec<Token>> {
    vec![Vec::new()]
}