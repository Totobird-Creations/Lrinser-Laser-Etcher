use std::process::exit;
use std::fs;

use super::lexer;
use super::parser;
use super::simplifier;
use super::interpreter;
use super::renderer;
use super::printer;



pub fn run(filename: &str) {
    let script = read(filename);

    let lexer_res = lexer::lex(filename.to_string(), script);
    if !lexer_res.success {
        println!("{}", lexer_res.exception);
        exit(1);
    }

    let parser_res = parser::parse(lexer_res.tokens);
    if !parser_res.success {
        println!("{}", parser_res.exception);
        exit(1);
    }

    let simplifier_res = simplifier::simplify(parser_res.nodes);

    let interpreter_res = interpreter::interpret(simplifier_res);
    if !interpreter_res.success {
        for exception in interpreter_res.exceptions {
            println!("{}", exception);
        }
        exit(1);
    }

    let renderer_res = renderer::render(interpreter_res.data);
    if !renderer_res.success {
        println!("{}", renderer_res.exception);
        exit(1);
    }

    let printer_res = printer::print("".to_string());
    if !printer_res.success {
        println!("{}", printer_res.exception);
        exit(1);
    }
}



fn read(filename: &str) -> String {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_e)      => {
            println!("File does not exist.");
            exit(1);
        }
    };

    return contents;
}
