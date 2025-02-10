
#[cfg(test)]
mod tests {
    use wind::frontend::{lexer::{self, Lexer}, preprocessor};

    fn prepare(src: &str) -> Lexer {
        let mut prep_lex_inst = lexer::Lexer::new(src.to_string(), true);
        let _ = prep_lex_inst.lex();

        let mut prep_inst = preprocessor::Preprocessor::new(prep_lex_inst.tokens);
        prep_inst.process();

        let lex_inst = lexer::Lexer::new(prep_inst.get_processed(), false);

        lex_inst
    }
    
    #[test]
    fn lexes() {

        let mut lexer = prepare("a = 10;");
        let _ = lexer.lex();

        assert!(lexer.tokens.len() > 0);
    }

}