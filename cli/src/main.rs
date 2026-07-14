use std::{env, fs};

use interpreter::interpreter::Interpreter;
use lexer::{lexer::Lexer, source::SourceFile};
use parser::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: code <file.code>");
        std::process::exit(1);
    }

    let path = &args[1];

    let text = fs::read_to_string(path).expect("Failed to read source file.");

    let source = SourceFile::new(path.clone(), text);

    let mut lexer = Lexer::new(&source);

    let tokens = lexer.tokenize().expect("Lexer error");

    let mut parser = Parser::new(tokens);

    let program = parser.parse().expect("Parser error");

    let mut interpreter = Interpreter::new();

    interpreter.execute(&program).expect("Runtime error");

    println!("Program executed successfully.");
}
