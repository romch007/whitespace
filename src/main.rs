mod interpreter;
mod lexer;
mod parser;

use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file).unwrap();

    let lexer = lexer::Lexer::new(content);
    let tokens = lexer.lex();

    let mut parser = parser::Parser::new(tokens);
    parser.parse().unwrap();

    let mut vm = interpreter::VM::new();
    if let Err(error) = vm.execute(&parser.output) {
        println!("error was: {error}");
        println!("stack: {:?}", vm.stack);
        println!("heap: {:?}", vm.heap);
    }
}
