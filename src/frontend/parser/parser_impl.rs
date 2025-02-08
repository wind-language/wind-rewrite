use std::{collections::LinkedList, iter::Peekable, slice::Iter};

use crate::frontend::{lexer::Lexer, token::{Token, TokenType}};

use super::{ASTNode, Expression, Literal, Variable};


pub struct Parser<'a> {
    token_iterator: Peekable<Iter<'a, Token>>,
    current_token: Option<&'a Token>,
    
    parsed_ast_nodes: LinkedList<ASTNode>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        let mut s = Self {
            token_iterator: lexer.tokens.iter().peekable(),
            current_token: None,
            parsed_ast_nodes: LinkedList::new()
        };

        // get first token 
        s.current_token = s.token_iterator.next();
        
        return s
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

        while matches!(self.current_token.unwrap().token_type, TokenType::PLUS | TokenType::MINUS) {
            let op = self.current_token.clone();
            self.eat(op.unwrap().token_type.clone());
            let right = self.parse_term();
            left = Expression::BINARY{ left: Box::new(left), right: Box::new(right), op: op.unwrap().literal.clone() };
        }
        left
    }

    pub fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_factor();

        while matches!(self.current_token.unwrap().token_type, TokenType::ASTERISK | TokenType::SLASH) {
            let op = self.current_token.clone();
            self.eat(op.unwrap().token_type.clone());
            let right = self.parse_factor();
            left = Expression::BINARY{ left: Box::new(left), right: Box::new(right), op: op.unwrap().literal.clone() };
        }

        left
    }

    pub fn parse_factor(&mut self) -> Expression {
        let unwrapped_current_token = self.current_token.unwrap();
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
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse_assignment(&mut self) -> Expression {
        match &self.current_token.unwrap().token_type {
            TokenType::IDENTIFIER => {

                let var_name = self.current_token.unwrap().literal.clone();
                self.eat(TokenType::IDENTIFIER);
                self.eat(TokenType::ASSIGN);
                let value = self.parse_expression();
                self.eat(TokenType::SEMICOLON);
                return Expression::BINARY{ left: Box::new(Expression::VARIABLE(Variable{name: var_name})), op: "=".to_string(), right: Box::new(value)};
            }        
            _ => {panic!("Expected assignment statement")}
        }
    }

    pub fn parse_all_tokens(&mut self) -> &LinkedList<ASTNode> {
        while self.current_token.is_some() {
            let node = match self.current_token.unwrap().token_type {
                TokenType::IDENTIFIER => {
                    // If the next token is an assignment (`=`), treat it as an assignment statement
                    if let Some(next_token) = self.token_iterator.peek() {
                        if next_token.token_type == TokenType::ASSIGN {
                            ASTNode::EXPRESSION(self.parse_assignment())
                        } else {
                            ASTNode::EXPRESSION(self.parse_expression())
                        }
                    } else {
                        ASTNode::EXPRESSION(self.parse_expression())
                    }
                }
                TokenType::INTEGER | TokenType::FLOAT => {
                    ASTNode::EXPRESSION(self.parse_expression())
                }
                _ => {
                    panic!("Unexpected token: {:?}", self.current_token);
                }
            };
    
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

