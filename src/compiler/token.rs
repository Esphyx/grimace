#[derive(Debug, Clone)]
pub enum Keyword {
    Function,
    Set,
    Loop,
}

#[derive(Debug, Clone)]
pub enum Logical {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
pub enum Bitwise {
    And,
    Or,
    Not,
    Shift,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Addition,
    Multiplication,
    Exponentiation,
    Subtraction,
    Division,
    Modulos,
    Bitwise(Bitwise),
    Logical(Logical),
}

#[derive(Debug, Clone)]
pub enum Punctuation {
    Seperator,
    EndLine,
}

#[derive(Debug, Clone)]
pub enum Token {
    Punctuation(Punctuation),
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
    Literal,
}
