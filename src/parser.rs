use std::collections::HashMap;

use crate::{
    ast::{Expression, ExpressionStatement, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub type PrefixParseFn = fn() -> Option<Box<dyn Expression>>;
pub type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

pub type PrefixParseFns = HashMap<TokenType, PrefixParseFn>;
pub type InfixParseFns = HashMap<TokenType, InfixParseFn>;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: Option<PrefixParseFns>,
    infix_parse_fns: Option<InfixParseFns>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),

            prefix_parse_fns: None,
            infix_parse_fns: None,
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn parse_program(&self) -> Program {
        self.prefix_parse_fns = Some(self.register_prefix_fns());
        self.infix_parse_fns = Some(self.register_infix_fns());

        let program = Program::new(vec![]);

        while self.current_token.unwrap().token_type != TokenType::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.advance_tokens();
        }

        return program;
    }

    fn parse_infix_expression(&self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        unimplemented!();
    }

    fn register_infix_fns(&self) -> InfixParseFns {
        let mut infix_parse_fns: HashMap<TokenType, InfixParseFn> = HashMap::new();
        unimplemented!();
        infix_parse_fns.insert(TokenType::PLUS, &self.parse_infix_expression);
        //return infix_parse_fns;
    }

    fn register_prefix_fns(&self) -> PrefixParseFns {
        let mut prefix_parse_fns = HashMap::new();
        prefix_parse_fns.insert(TokenType::PLUS, self.parse_infix_expression);
        unimplemented!();
        //return prefix_parse_fns;
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.unwrap().token_type {
            TokenType::LET => Some(self.parse_let_statement().unwrap()),
            TokenType::RETURN => Some(self.parse_return_statement().unwrap()),
            _ => Some(self.parse_expression_statement().unwrap()),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Box<ExpressionStatement>> {
        let expression_statement = ExpressionStatement {
            token: self.current_token.unwrap().clone(),
            expression: self.parse_expression(),
        };

        if self.peek_token.unwrap().token_type == TokenType::SEMICOLON {
            self.advance_tokens();
        }
        return Some(Box::new(expression_statement));
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        let prefix_parse_fn = self
            .prefix_parse_fns
            .unwrap()
            .get(&self.current_token.unwrap().token_type);
        if prefix_parse_fn.is_none() {
            self.errors.push(format!(
                "no prefix parse function for {} found",
                self.current_token.unwrap().literal
            ));
            return None;
        }

        let prefix_parse_fn = prefix_parse_fn.unwrap();
        let left_exp = prefix_parse_fn();

        if left_exp.is_none() {
            return None;
        }

        let mut left_exp = left_exp.unwrap();

        while self.peek_token.unwrap().token_type != TokenType::SEMICOLON {
            let infix_parse_fn = self
                .infix_parse_fns
                .unwrap()
                .get(&self.peek_token.unwrap().token_type);
            if infix_parse_fn.is_none() {
                break;
            }

            let infix_parse_fn = infix_parse_fn.unwrap();
            self.advance_tokens();

            let right_exp = infix_parse_fn(left_exp.into().clone());
            if right_exp.is_none() {
                return None;
            }

            left_exp = right_exp.unwrap();
        }

        return Some(left_exp);
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        unimplemented!()
    }

    fn parse_return_statement(&mut self) -> Option<Box<ReturnStatement>> {
        unimplemented!()
    }
}
