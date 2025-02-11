#[cfg(test)]
mod simple_parsing_test {
    use wind::frontend::{ast::{ASTNode, Expression, Literal}, lexer::{self, Lexer}, parser::Parser, preprocessor};
    fn prepare(src: &str) -> Lexer {
        let mut prep_lex_inst = lexer::Lexer::new(src.to_string(), true);
        let _ = prep_lex_inst.lex();

        let mut prep_inst = preprocessor::Preprocessor::new(prep_lex_inst.tokens);
        prep_inst.process();

        let lex_inst = lexer::Lexer::new(prep_inst.get_processed(), false);

        lex_inst
    }

    #[test]
    fn parses_simple_operations_correctly() {

        let mut lexer = prepare("var1 = 10;");
        let _ = lexer.lex();

        let mut parser = Parser::new(&lexer);
        let nodes = parser.parse_all_tokens();
        let mut iterator = nodes.iter();
        let first_line = iterator.next().unwrap();

        if let ASTNode::EXPRESSION(Expression::BINARY { left, right, op }) = first_line {            
            if let Expression::VARIABLE(var) = left.as_ref() {
                assert!(var.name == "var1", "Wrong parsing of variable name");
            } else {
                panic!("Expression should be an varible")
            }
            
            if let Expression::LITERAL(var) = right.as_ref() {
                if let Literal::INTEGER(st) = var {
                    assert!(*st == 10);
                } else {
                    panic!("Value should be an integer")
                }
            } else {
                panic!("Expression should be a literal")
            }

            assert!(op == "=", "Wrong parsing of operation")
            
        } else {
            panic!("Expected ASTNode::EXPRESSION(Expression::BINARY)");
        }
    }
}