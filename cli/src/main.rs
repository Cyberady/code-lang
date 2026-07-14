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
            eprintln!("File Error:\n{}", error);
            process::exit(1);
        }
    };

    let source = SourceFile::new(path.clone(), text);

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer Error:\n{}", error);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(error) => {
            eprintln!("Parser Error:\n{}", error);
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();

    if let Err(error) = interpreter.execute(&program) {
        eprintln!("Runtime Error:\n{}", error);
        process::exit(1);
    }

    println!("Program executed successfully.");
}
