use std::process::exit;
use std::fs;

use super::lexer;
use super::parser;
use super::simplifier;
use super::interpreter;
use super::renderer;
use super::printer;



// Function for easily running from filename. 
pub fn run(filename: &str) {
    // Read script file.
    let script = read(filename);

    // Lex text file.
    let lexer_res = lexer::lex(filename.to_string(), script);
    if !lexer_res.success {
        println!("{}", lexer_res.exception);
        exit(1);
    }

    // Parse token list.
    let parser_res = parser::parse(lexer_res.tokens);
    if !parser_res.success {
        println!("{}", parser_res.exception);
        exit(1);
    }

    // Simplify node list.
    let simplifier_res = simplifier::simplify(parser_res.nodes);

    // Interpret node list.
    let interpreter_res = interpreter::interpret(simplifier_res);
    if !interpreter_res.success {
        for exception in interpreter_res.exceptions {
            println!("{}", exception);
        }
        exit(1);
    }

    // Render interpreter data.
    let renderer_res = renderer::render(interpreter_res.data);
    if !renderer_res.success {
        println!("{}", renderer_res.exception);
        exit(1);
    }

    // Print export file.
    let printer_res = printer::print(renderer_res.export_filename);
    if !printer_res.success {
        println!("{}", printer_res.exception);
        exit(1);
    }
}



// Read text file.
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
