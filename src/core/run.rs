use std::process::exit;
use std::fs;

use super::logger;
use super::data;
use super::lexer;
use super::parser;
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

    // Interpret node list.
    logger::debug(format!("Interpreting node tree."));
    let interpreter_res = interpreter::interpret(parser_res.nodes);
    if !interpreter_res.success {
        logger::critical("Interpreting failed. Error provided:");
        for exception in interpreter_res.exceptions {
            println!("{}", exception);
        }
        exit(1);
    }

    // Render interpreter data.
    logger::debug(format!("Rendering data."));
    let renderer_res = renderer::render(interpreter_res.data.clone());
    if !renderer_res.success {
        logger::critical("Rendering failed. Error provided:");
        println!("\n{}", renderer_res.exception);
        exit(1);
    }

    // Print export file.
    if interpreter_res.data.print_now {
        logger::debug(format!("Printing image."));
        let printer_res = printer::print(renderer_res.export_filename);
        if !printer_res.success {
            println!("{}", printer_res.exception);
            exit(1);
        }
    } else {
        logger::warning(format!("Printer disabled. Skipping."));
    }
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
