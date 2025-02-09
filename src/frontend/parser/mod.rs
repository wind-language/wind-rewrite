use std::{collections::LinkedList, iter::Peekable, slice::Iter};

use crate::frontend::{lexer::Lexer, token::{Token, TokenType}};

use crate::frontend::ast::{ASTNode, Expression, Literal, Variable};


pub struct Parser<'a> {
    token_iterator: Peekable<Iter<'a, Token>>,
    current_token: Option<&'a Token>,
    
    parsed_ast_nodes: LinkedList<ASTNode>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        let mut iterator = lexer.tokens.iter().peekable();
        
        // get first token 
        let current_token = iterator.next();

        Self {
            token_iterator: iterator,
            current_token: current_token,
            parsed_ast_nodes: LinkedList::new()
        }        
    }

    fn eat(&mut self, expected: TokenType) {
        if self.current_token.unwrap().token_type == expected {
            self.current_token = self.token_iterator.next();
        } else {
            panic!("Expected {:?}, but found {:?}", expected, self.current_token);
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        let mut left = self.parse_term();

        while matches!(self.current_token.unwrap().token_type, TokenType::PLUS | TokenType::MINUS | TokenType::ASSIGN | TokenType::ASTERISK | TokenType::PLUS_ASSIGN | TokenType::MINUS_ASSIGN ) {
            let op = self.current_token.unwrap().clone();
            self.eat(op.token_type);
            let right = self.parse_term();
            left = Expression::BINARY{ left: Box::new(left), right: Box::new(right), op: op.literal };
        }

        left
    }

    pub fn parse_expression_enforce_semicolon(&mut self) -> Expression {
        let exp = self.parse_expression();
        self.eat(TokenType::SEMICOLON);

        exp
    }

    pub fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_factor();

        while matches!(self.current_token.expect("Unexpected end of input.").token_type, TokenType::ASTERISK | TokenType::SLASH) {
            let op = self.current_token.clone();
            self.eat(op.unwrap().token_type.clone());
            let right = self.parse_factor();
            left = Expression::BINARY{ left: Box::new(left), right: Box::new(right), op: op.unwrap().literal.clone() };
        }

        left
    }

    pub fn parse_factor(&mut self) -> Expression {
        let unwrapped_current_token = self.current_token.expect("Unexpected end of input");
        match &unwrapped_current_token.token_type {
            TokenType::INTEGER => {
                let parsed_value = unwrapped_current_token.literal.parse::<i64>().expect("Token has wrong type.");
                let expr = Expression::LITERAL(Literal::INTEGER(parsed_value));
                self.eat(unwrapped_current_token.token_type.clone());
                expr
            }
            TokenType::FLOAT => {
                let parsed_value = unwrapped_current_token.literal.parse::<f64>().expect("Token has wrong type.");
                let expr = Expression::LITERAL(Literal::FLOAT(parsed_value));
                self.eat(unwrapped_current_token.token_type.clone());
                expr
            }
            TokenType::IDENTIFIER => {
                let expr = Expression::VARIABLE(Variable { name: unwrapped_current_token.literal.clone() });
                self.eat(unwrapped_current_token.token_type.clone());
                expr
            }
            TokenType::LPAREN => {
                self.eat(TokenType::LPAREN);
                let inner_expression = self.parse_expression();
                self.eat(TokenType::RPAREN);
                inner_expression
            }
            TokenType::MINUS | TokenType::PLUS => {
                self.eat(self.current_token.unwrap().token_type.clone());
                self.parse_expression()
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }


    pub fn parse_all_tokens(&mut self) -> &LinkedList<ASTNode> {
        while self.current_token.is_some() {
            let node = ASTNode::EXPRESSION(self.parse_expression_enforce_semicolon());
            self.parsed_ast_nodes.push_back(node);
        }
    
        &self.parsed_ast_nodes
    }

    pub fn dump_nodes(&self) {
        for node in self.parsed_ast_nodes.iter() {
            println!("{:?}", node)
        }    
    }
    
}

