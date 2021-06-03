use crate::brain::tokens::{
    Tokens,
    TokenType
};

pub fn tokenizer(input: String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();

    for now in input.chars() {
        match now {
            '<' => {tokens.push(TokenType{token: Tokens::Left, value: now })},
            '>' => {tokens.push(TokenType{token: Tokens::Right, value: now })},
            '+' => {tokens.push(TokenType{token: Tokens::Plus, value: now })},
            '-' => {tokens.push(TokenType{token: Tokens::Minus, value: now })},
            '.' => {tokens.push(TokenType{token: Tokens::Dot, value: now })},
            ',' => {tokens.push(TokenType{token: Tokens::Comma, value: now })},
            '[' => {tokens.push(TokenType{token: Tokens::BracketL, value: now })},
            ']' => {tokens.push(TokenType{token: Tokens::BracketR, value: now })},
            _ => {println!("Unexpected token: {}", now)}
        };
    }

    return tokens;
}