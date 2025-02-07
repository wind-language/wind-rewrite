use macros::BodyNode;

use crate::frontend::token;

pub mod macros;

struct BufferStream {
    src: Vec<token::Token>,
    index: usize,
}

pub struct Preprocessor {
    src: BufferStream,
    plain: String,
    macros: Vec<macros::Macro>
}

impl Preprocessor {
    pub fn new(src: Vec<token::Token>) -> Preprocessor {
        Preprocessor {
            src: BufferStream {
                src: src,
                index: 0,
            },
            plain: "".to_string(),
            macros: vec![]
        }
    }

    fn advance(&mut self, n: usize) {
        self.src.index += n;
    }

    fn expect(&mut self, token_type: token::TokenType) -> token::Token {
        self.skip_ws();
        let token= self.current().clone();
        self.advance(1);
        if token.token_type != token_type {
            panic!("Unexpected token: {:?}", token);
        }
        token
    }

    fn peek(&self, n: usize) -> &token::Token {
        &self.src.src[self.src.index + n]
    }

    fn current(&self) -> &token::Token {
        &self.src.src[self.src.index]
    }

    fn skip_ws(&mut self) {
        while {
            let current_token = self.current().clone();
            current_token.token_type == token::TokenType::WS
        } {
            let current_token = self.current().clone();
            for c in current_token.literal.chars() {
                if c == '\n' {
                    self.plain.push('\n');
                }
            }
            self.advance(1);
        }
    }

    pub fn find_macro(&mut self, name: String) -> Option<&macros::Macro> {
        for macro_ in self.macros.iter() {
            if macro_.name == name {
                return Some(macro_);
            }
        }
        None
    }
    
    fn expand_macro(&mut self, macro_: &macros::Macro, args: Vec<String>) {
        let params_map: Vec<(String, String)> = macro_.params.iter().zip(args.iter()).map(|(a, b)| (a.clone(), b.clone())).collect();
        for node in macro_.body.iter() {
            match node {
                BodyNode::Lexical(content) => {
                    self.expand_identstr(content.clone());
                }
                BodyNode::Param(param) => {
                    if let Some((_, value)) = params_map.iter().find(|(a, _)| a == param) {
                        self.expand_identstr(value.clone())
                    }
                }
                BodyNode::ArgIteration(body) => {
                    for arg in args.iter() {
                        let mut macro_clone = macro_.clone();
                        macro_clone.params.push("arg".to_string());
                        self.expand_macro(
                            &macros::Macro::new(
                                macro_clone.name.clone(),
                                macro_clone.params.clone(),
                                body.clone()
                            ),
                            vec![arg.clone()]
                        );
                    }
                }
            }
        }
    }

    fn process_macro_find(&mut self, macro_: &macros::Macro) {
        if self.current().token_type == token::TokenType::LPAREN {
            self.expect(token::TokenType::LPAREN);
            let mut paren_count = 1;
            let mut args: Vec<String> = vec![];
            let mut latest_arg = String::new();
    
            while paren_count > 0 && self.src.index < self.src.src.len() {
                let tk = self.current();
                match tk.token_type {
                    token::TokenType::LPAREN => {
                        paren_count += 1;
                        latest_arg.push('(');
                        self.advance(1);
                    }
                    token::TokenType::RPAREN => {
                        paren_count -= 1;
                        if paren_count == 0 {
                            break;
                        }
                        latest_arg.push(')');
                        self.advance(1);
                    }
                    token::TokenType::COMMA => {
                        if paren_count == 1 {
                            args.push(latest_arg.clone());
                            latest_arg.clear();
                            self.expect(token::TokenType::COMMA);
                        } else {
                            latest_arg.push(',');
                            self.advance(1);
                        }
                    }
                    _ => {
                        latest_arg.push_str(&tk.literal);
                        self.advance(1);
                    }
                }
            }
    
            if !latest_arg.is_empty() {
                args.push(latest_arg);
            }
    
            self.expect(token::TokenType::RPAREN);
            self.expand_macro(macro_, args);
        } else {
            self.expand_macro(macro_, vec![]);
        }
    }

    fn find_str_param_macro(&mut self, macro_: &macros::Macro, s: String) {
        if !s.contains('(') {
            return;
        }
        let Some(start_index) = s.find('(') else {
            return;
        };
        let mut paren_count = 0;
        let mut latest_arg = String::new();
        let mut args = Vec::new();
        for c in s[start_index..].chars() {
            match c {
                '(' => {
                    if paren_count > 0 {
                        latest_arg.push('(');
                    }
                    paren_count += 1;
                }
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        if !latest_arg.is_empty() {
                            args.push(latest_arg);
                        }
                        break;
                    } else {
                        latest_arg.push(')');
                    }
                }
                ',' => {
                    if paren_count == 1 {
                        args.push(latest_arg.clone());
                        latest_arg.clear();
                    } else {
                        latest_arg.push(',');
                    }
                }
                _ => {
                    latest_arg.push(c);
                }
            }
        }
        self.expand_macro(macro_, args);
    }

    fn expand_identstr(&mut self, str: String) {
        let mut name = str.clone();
        let mut prm: bool =false;
        if let Some(index) = name.find('(') {
            name = name[..index].to_string();
            if name.starts_with(' ') {
                name = name[1..].to_string();
            }
            prm = true;
        }

        let macro_exists = self.macros.iter()
            .any(|m| m.name == name);
        if macro_exists {
            if let Some(index) = self.macros.iter().position(|m| m.name == name) {
                let macro_ = self.macros[index].clone();
                if prm {
                    self.find_str_param_macro(&macro_, str);
                    return;
                }
                self.process_macro_find(&macro_);
                return;
            }
        }
        
        self.plain.push_str(&str);
    }

    fn process_ident(&mut self) {
        let token_literal = self.expect(token::TokenType::IDENTIFIER).literal;
        self.expand_identstr(token_literal);
    }
    

    fn process_bnode(&mut self, mut params: Vec<String>) -> BodyNode {
        let cur = self.current().clone();
        match cur.token_type {
            token::TokenType::IDENTIFIER => {
                if params.contains(&self.current().literal) {
                    BodyNode::Param(self.expect(token::TokenType::IDENTIFIER).literal)
                }
                else {
                    BodyNode::Lexical(self.expect(token::TokenType::IDENTIFIER).literal)
                }
            }
            token::TokenType::NOT => {
                let next = self.peek(1).literal.clone();
                if next.contains(&"for::arg".to_string()) {
                    self.expect(token::TokenType::NOT);
                    self.expect(token::TokenType::IDENTIFIER);
                    self.skip_ws();
                    let mut braces = (self.current().token_type == token::TokenType::LBRACE) as usize;
                    let mut body: Vec<macros::BodyNode> = vec![];
                    self.expect(token::TokenType::LBRACE);
                    params.push("arg".to_string());
                    while braces > 0 {
                        if self.current().token_type == token::TokenType::LBRACE {
                            braces += 1;
                        } else if self.current().token_type == token::TokenType::RBRACE {
                            braces -= 1;
                        }
                        if braces == 0 {
                            break;
                        }
                        body.push(self.process_bnode(params.clone()));
                    }
                    self.expect(token::TokenType::RBRACE);
                    params.pop();
                    BodyNode::ArgIteration(body)
                }
                else {
                    self.advance(1);
                    BodyNode::Lexical(cur.literal.clone())
                }
            }
            _ => {
                self.advance(1);
                BodyNode::Lexical(cur.literal.clone())
            }
        }
    }

    fn process_macrodef(&mut self) {
        self.expect(token::TokenType::NOT);
        self.expect(token::TokenType::IDENTIFIER);
        let name = self.expect(token::TokenType::IDENTIFIER).literal;
        let mut params: Vec<String> = vec![];
        self.expect(token::TokenType::COLON);
        self.skip_ws();
        if self.current().token_type == token::TokenType::LPAREN {
            self.expect(token::TokenType::LPAREN);
            while self.current().token_type != token::TokenType::RPAREN && self.src.index < self.src.src.len() {
                params.push(self.expect(token::TokenType::IDENTIFIER).literal);
                if self.current().token_type == token::TokenType::COMMA {
                    self.expect(token::TokenType::COMMA);
                }
            }
            self.expect(token::TokenType::RPAREN);
        }
        let mut body: Vec<macros::BodyNode> = vec![];


        self.skip_ws();
        let mut braces = (self.current().token_type == token::TokenType::LBRACE) as usize;
        if self.current().token_type == token::TokenType::LBRACE {
            self.expect(token::TokenType::LBRACE);
            self.skip_ws();
            while braces > 0 {
                if self.current().token_type == token::TokenType::LBRACE {
                    braces += 1;
                } else if self.current().token_type == token::TokenType::RBRACE {
                    braces -= 1;
                }
                if braces <= 0 {
                    self.advance(1);
                    break;
                }
                body.push(self.process_bnode(params.clone()));
            }
            if body.len() > 0 {
                if let Some(BodyNode::Lexical(content)) = body.last() {
                    if content == "\n" {
                        body.pop();
                    }
                }
            }
        } else {
            body.push(self.process_bnode(params.clone()));
            self.advance(1);
        }
        self.add_macro(macros::Macro::new(name, params, body));
    }

    pub fn process(&mut self) {
        while self.src.index < self.src.src.len() {
            let token = {
                let t = self.current();
                t
            };
            let token_literal = token.literal.clone();
            let token_type = token.token_type.clone();
    
            match token_type {
                token::TokenType::IDENTIFIER => {
                    self.process_ident();
                }
                token::TokenType::NOT => {
                    if self.peek(1) == &token::Token::new(token::TokenType::IDENTIFIER, "macro".to_string(), (0..0, 0..0), 0) {
                        self.process_macrodef();
                    }
                }
                _ => {
                    self.plain.push_str(&token_literal);
                    self.advance(1);
                }
            }
        }
    }
    
    pub fn add_macro(&mut self, macro_: macros::Macro) {
        self.macros.push(macro_);
    }

    pub fn get_processed(&self) -> String {
        self.plain.clone()
    }
}
