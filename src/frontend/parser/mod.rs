use std::{collections::LinkedList, iter::Peekable, slice::Iter};

use crate::frontend::{lexer::Lexer, token::{Token, TokenType}};

use crate::frontend::ast::{ASTNode, Expression, Literal, Variable};

use super::token::BINARY_OPERATORS;


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

    /// The highest, the most preceding.
    const fn get_operator_precedence(&self, tok: TokenType) -> u8 {
        match tok {
            TokenType::PLUS | TokenType::MINUS => 1,
            TokenType::ASTERISK | TokenType::SLASH => 2,
            TokenType::EQ | TokenType::LT | TokenType::GT | TokenType::LTE | TokenType::GTE  => 2,
            // TokenType::CAST_SYMBOL => 3
        
            // TODO: More
            _ => 0
        }
    }

    pub fn parse_expression(&mut self, precedence: u8) -> Expression {
        let mut left = self.parse_factor();
    

        while let Some(token) = self.current_token {
            if !BINARY_OPERATORS.contains(&token.token_type) {    
                break;
            }

            let new_precedence = self.get_operator_precedence(token.token_type.clone());
            if new_precedence < precedence {
                break;
            }
    
            let op = self.current_token.unwrap().clone();
            self.eat(op.token_type.clone());
    
            let mut right = self.parse_factor();  // Right side with higher precedence
            
            let token_after_factor = self.current_token.unwrap().clone();
            
            if BINARY_OPERATORS.contains(&token_after_factor.token_type) {
                let next_precedence = self.get_operator_precedence(self.current_token.unwrap().token_type);
                if new_precedence < next_precedence {
                    let next_expr = self.parse_expression(next_precedence+1);
                    if matches!(next_expr, Expression::VARIABLE(_)) {
                        right = Expression::BINARY { left: Box::new(right), op: token_after_factor.literal.clone(), right: Box::new(next_expr) };
                    } else {
                        right = Expression::BINARY { left: Box::new(next_expr), op: token_after_factor.literal.clone(), right: Box::new(right) };
                    }
                }
            }
    
            left = Expression::BINARY {
                left: Box::new(left),
                right: Box::new(right),
                op: op.literal.clone(),
            };
        }
    
        left
    }    

    pub fn parse_expression_enforce_semicolon(&mut self) -> Expression {
        let exp = self.parse_expression(0);  // Start with lowest precedence
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
        let current_token = self.current_token.expect("Unexpected end of input");
        match &current_token.token_type {
            TokenType::INTEGER => {
                let parsed_value = current_token.literal.parse::<i64>().expect("Token has wrong type.");
                let expr = Expression::LITERAL(Literal::INTEGER(parsed_value));
                self.eat(current_token.token_type.clone());
                expr
            }
            TokenType::FLOAT => {
                let parsed_value = current_token.literal.parse::<f64>().expect("Token has wrong type.");
                let expr = Expression::LITERAL(Literal::FLOAT(parsed_value));
                self.eat(current_token.token_type.clone());
                expr
            }
            TokenType::IDENTIFIER => {
                let expr = Expression::VARIABLE(Variable { name: current_token.literal.clone() });
                self.eat(current_token.token_type.clone());
                expr
            }
            TokenType::LPAREN => {
                self.eat(TokenType::LPAREN);
                let inner_expression = self.parse_expression(0);
                self.eat(TokenType::RPAREN);
                inner_expression
            }
            _ if BINARY_OPERATORS.contains(&current_token.token_type) => {
                self.eat(self.current_token.unwrap().token_type.clone());
                self.parse_factor()
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

