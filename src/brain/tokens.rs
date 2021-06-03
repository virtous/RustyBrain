#[derive(Debug)]
pub enum Tokens {
    Left,
    Right,
    Plus,
    Minus,
    Dot,
    Comma,
    BracketL,
    BracketR
}

#[derive(Debug)]
pub struct TokenType {
    pub token: Tokens,
    pub value: char
}