use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenType {
    ASSIGN,
    COMMA,
    DIVISION,
    ELSE,
    EOF,
    EQ,
    FALSE,
    FUNCTION,
    GT,
    IDENT,
    IF,
    ILLEGAL,
    INT,
    LBRACE,
    LET,
    LPAREN,
    LT,
    MINUS,
    MULTIPLICATION,
    NEGATION,
    NOTEQ,
    PLUS,
    RETURN,
    RPAREN,
    RBRACE,
    SEMICOLON,
    STRING,
    TRUE,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type,
            literal,
        };
    }
}

pub fn lookup_token_type(literal: &String) -> TokenType {
    let mut keywords = HashMap::new();
    keywords.insert("false", TokenType::FALSE);
    keywords.insert("proc", TokenType::FUNCTION);
    keywords.insert("return", TokenType::RETURN);
    keywords.insert("if", TokenType::IF);
    keywords.insert("else", TokenType::ELSE);
    keywords.insert("declare", TokenType::LET);
    keywords.insert("true", TokenType::TRUE);

    match keywords.get(literal.as_str()) {
        Some(token_type) => token_type.clone(),
        None => TokenType::IDENT,
    }
}
