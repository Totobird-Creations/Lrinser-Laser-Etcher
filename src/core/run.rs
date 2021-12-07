use std::process::exit;
use std::fs;

use super::logger;
use super::data;
use super::lexer;
use super::parser;
use super::simplifier;
use super::interpreter;
use super::renderer;
use super::printer;



// Function for easily running from filename. 
pub fn run(filename: &str) {
    logger::info(format!("Commencing print on file `{}`.", data::escapify(filename.to_string())));

    // Read script file.
    logger::debug(format!("Reading file `{}`.", data::escapify(filename.to_string())));
    let script = read(filename);

    // Lex text file.
    logger::debug(format!("Lexing script."));
    let lexer_res = lexer::lex(filename.to_string(), script);
    if !lexer_res.success {
        logger::critical("Lexing failed. Error provided:");
        println!("\n{}", lexer_res.exception);
        exit(1);
    }

    // Parse token list.
    logger::debug(format!("Parsing tokens."));
    let parser_res = parser::parse(lexer_res.tokens);
    if !parser_res.success {
        logger::critical("Parsing failed. Error provided:");
        println!("\n{}", parser_res.exception);
        exit(1);
    }

    // Simplify node list.
    logger::warning(format!("Simplifier incomplete. Skipping."));
    let simplifier_res = simplifier::simplify(parser_res.nodes);

    // Interpret node list.
    logger::debug(format!("Interpreting node tree."));
    let interpreter_res = interpreter::interpret(simplifier_res);
    if !interpreter_res.success {
        logger::critical("Interpreting failed. Error provided:");
        for exception in interpreter_res.exceptions {
            println!("{}", exception);
        }
        exit(1);
    }

    // Render interpreter data.
    logger::debug(format!("Rendering data."));
    let renderer_res = renderer::render(interpreter_res.data);
    if !renderer_res.success {
        logger::critical("Rendering failed. Error provided:");
        println!("\n{}", renderer_res.exception);
        exit(1);
    }

    logger::warning(format!("Printer disabled. Skipping."));
    // Print export file.
    /*let printer_res = printer::print(renderer_res.export_filename);
    if !printer_res.success {
        println!("{}", printer_res.exception);
        exit(1);
    }*/
    logger::success("All jobs finished.");
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
