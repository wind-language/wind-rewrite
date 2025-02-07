use crate::reporter;
use crate::frontend::preprocessor;
use crate::frontend::lexer;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const HELP: &str = r#"
📚 Help
📦 Usage: {} <source file>
📖 Options:
  --help:    Display this information.
  --version: Display version information.
"#;

pub struct CompilationInstance {
    pub sources: Vec<String>,
    pub output: String
}

impl CompilationInstance {
    pub fn new() -> CompilationInstance {
        CompilationInstance {
            sources: Vec::new(),
            output: String::new()
        }
    }
}

fn process_file(file: &str) -> Result<(), reporter::usr::CliError> {
    let src = std::fs::read_to_string(file).map_err(|e| reporter::usr::CliError::file_read_error(file.to_string(), e))?;
    println!("📖 Source:\n{}", src);

    let mut prep_lex_inst = lexer::Lexer::new(src, true);
    if let Err(e) = prep_lex_inst.lex() {
        eprintln!("{}", e);
    }
    //prep_lex_inst.dump_tokens();
    let mut prep_inst = preprocessor::Preprocessor::new(prep_lex_inst.tokens);
    prep_inst.process();
    let mut lex_inst = lexer::Lexer::new(prep_inst.get_processed(), false);
    if let Err(e) = lex_inst.lex() {
        eprintln!("{}", e);
    }
    lex_inst.dump_tokens();
    Ok(())
}

pub fn run_cli() -> Result<(), reporter::usr::CliError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(reporter::usr::CliError::missing_file_usage(args[0].clone()));
    }
    let mut instance = CompilationInstance::new();

    for arg in args.iter().skip(1) {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--help" => {
                    println!("{}", HELP);
                    return Ok(());
                },
                "--version" => {
                    println!("📦 Version: {}", VERSION);
                    println!("👤 Authors: {}", AUTHORS);
                    return Ok(());
                },
                _ => {
                    return Err(reporter::usr::CliError::Unknown);
                }
            }
        } else {
            instance.sources.push(arg.clone());
        }
    }

    for source in instance.sources.iter() {
        println!("📂 Reading file: {}", source);
        if let Err(e) = process_file(source) {
            eprintln!("{}", e);
        }
    }
    
    Ok(())
}