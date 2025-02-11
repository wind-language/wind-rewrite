#[cfg(test)]
mod simple_parsing_test {
    use wind::frontend::{lexer::{self, Lexer}, parser::Parser, preprocessor};

    fn prepare(src: &str) -> Lexer {
        let mut prep_lex_inst = lexer::Lexer::new(src.to_string(), true);
        let _ = prep_lex_inst.lex();

        let mut prep_inst = preprocessor::Preprocessor::new(prep_lex_inst.tokens);
        prep_inst.process();

        let lex_inst = lexer::Lexer::new(prep_inst.get_processed(), false);

        lex_inst
    }

    
    #[test]
    fn does_parse() {

        let mut lexer = prepare("var1 = 10;");
        let _ = lexer.lex();


        let mut parser = Parser::new(&lexer);

        let nodes = parser.parse_all_tokens();

        assert!(nodes.len() > 0);
    }

    #[test]
    #[should_panic(expected = "Expected SEMICOLON, but found Some([IDENTIFIER]: \"var2\" at (2..2, 9..12))")]
    fn enforces_semicolons() {
        let mut lexer = prepare("
        var1 = 10
        var2 = 20;"
    );
        let _ = lexer.lex();
        let mut parser = Parser::new(&lexer);
        parser.parse_all_tokens();        
    }

    #[test]
    #[should_panic(expected = "Unexpected end of input")]
    fn expects_expressions_to_be_complete() {
        let mut lexer = prepare("var1 =");
        let _ = lexer.lex();
        let mut parser = Parser::new(&lexer);
        parser.parse_all_tokens();        
    }

}