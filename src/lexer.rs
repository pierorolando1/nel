use crate::token::{lookup_token_type, Token, TokenType};

pub struct Lexer {
    source: String,
    position: usize,
    read_position: usize,
    character: String,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        return Lexer {
            character: "".to_owned(),
            source,
            position: 0,
            read_position: 0,
        };
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token;

        match self.character.as_str() {
            "=" => {
                if self.peek_character() == "=" {
                    token = Token::new(TokenType::EQ, "==".to_owned())
                }
                token = Token::new(TokenType::ASSIGN, self.character.clone());
            }
            "+" => {
                token = Token::new(TokenType::PLUS, self.character.clone());
            }
            "" => {
                token = Token::new(TokenType::EOF, self.character.clone());
            }
            "(" => {
                token = Token::new(TokenType::LPAREN, self.character.clone());
            }
            ")" => {
                token = Token::new(TokenType::RPAREN, self.character.clone());
            }
            "{" => {
                token = Token::new(TokenType::LBRACE, self.character.clone());
            }
            "}" => {
                token = Token::new(TokenType::RBRACE, self.character.clone());
            }
            "," => {
                token = Token::new(TokenType::COMMA, self.character.clone());
            }
            ";" => {
                token = Token::new(TokenType::SEMICOLON, self.character.clone());
            }
            "-" => {
                token = Token::new(TokenType::MINUS, self.character.clone());
            }
            "/" => {
                token = Token::new(TokenType::DIVISION, self.character.clone());
            }
            "*" => {
                token = Token::new(TokenType::MULTIPLICATION, self.character.clone());
            }
            "<" => {
                token = Token::new(TokenType::LT, self.character.clone());
            }
            ">" => {
                token = Token::new(TokenType::GT, self.character.clone());
            }
            "!" => {
                if self.peek_character() == "=" {
                    self.read_char();
                    token = Token::new(TokenType::NOTEQ, "!=".to_owned());
                } else {
                    token = Token::new(TokenType::NEGATION, self.character.clone());
                }
            }
            // TODO: is letter, is number, read string
            tkn => {
                if self.is_letter(self.character.as_str()) {
                    let literal = self.read_identifier();
                    return Token::new(lookup_token_type(&literal), literal);
                }

                if self.is_number(self.character.as_str()) {
                    let literal = self.read_number();
                    return Token::new(TokenType::INT, literal);
                }

                if tkn == "\"" {
                    let literal = self.read_string();
                    return Token::new(TokenType::STRING, literal);
                }

                token = Token::new(TokenType::ILLEGAL, self.character.clone());
            }
        }

        self.read_char();
        return token;
    }

    fn is_letter(&self, character: &str) -> bool {
        return character.chars().all(|c| c.is_alphabetic() || c == '_');
    }

    fn is_number(&self, character: &str) -> bool {
        return character.chars().all(|c| c.is_numeric());
    }

    fn read_number(&mut self) -> String {
        let initial_position = self.position;
        while self.is_number(self.character.as_str()) {
            self.read_char();
        }

        return self.source[initial_position..self.position].to_owned();
    }

    fn read_identifier(&mut self) -> String {
        let initial_position = self.position;
        let mut is_first_letter = true;
        while self.is_letter(self.character.as_str())
            || (!is_first_letter && self.is_number(self.character.as_str()))
        {
            self.read_char();
            is_first_letter = false;
        }
        return self.source[initial_position..self.position].to_owned();
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        while self.character != "\"" {
            result.push_str(&self.character);
            self.read_char();
        }
        return result;
    }

    fn peek_character(&self) -> String {
        if self.read_position >= self.source.len() {
            return "".to_owned();
        }
        return self
            .source
            .chars()
            .nth(self.read_position)
            .unwrap()
            .to_string();
    }

    fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.character = "".to_owned();
        } else {
            self.character = self
                .source
                .chars()
                .nth(self.read_position)
                .unwrap()
                .to_string();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.character == " " {
            self.read_char();
        }
    }
}
