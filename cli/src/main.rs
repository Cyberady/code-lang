use std::{env, fs, process};

use interpreter::interpreter::Interpreter;
use lexer::{lexer::Lexer, source::SourceFile};
use parser::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: code <file.code>");
        process::exit(1);
    }

    let path = &args[1];

    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("File Error");
            eprintln!("{error}");
            process::exit(1);
        }
    };

    let source = SourceFile::new(path.clone(), text);

    // -------------------------
    // Lexer
    // -------------------------
    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    // -------------------------
    // Parser
    // -------------------------
    let mut parser = Parser::new(tokens);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    // -------------------------
    // Interpreter
    // -------------------------
    let mut interpreter = Interpreter::new(&source);

    match interpreter.execute(&program) {
        Ok(()) => {
            println!("Program executed successfully.");
        }

        Err(error) => {
            let diagnostic = interpreter.diagnostic(&error);

            eprintln!("{}", diagnostic.render());

            process::exit(1);
        }
    }
}
