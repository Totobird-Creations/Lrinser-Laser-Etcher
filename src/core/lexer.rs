use std::collections::HashMap;

use super::data;
use super::tokens;
use super::exceptions;



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct LexerResult {
    pub success   : bool,
    pub tokens    : Vec<tokens::Token>,
    pub exception : exceptions::LexerException
}



// Lexer class
#[derive(Clone, Debug)]
struct Lexer {
    filename : String,
    script   : String,
    chars    : Vec<char>,
    pos      : usize,
    ch       : char,
    end      : bool
}
impl Lexer {
    // Initialize variables on creation.
    pub fn init(&mut self) {
        if self.script.len() >= 1 {
            self.ch = self.chars[0];
            self.end = false;
        } else {
            self.ch = ' ';
            self.end = true;
        }
    }

    // Move to next character.
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.chars.len() {
            self.ch = self.chars[self.pos];
        } else {
            self.ch = ' ';
            self.end = true;
        }
    }

    // Get data::Range class with start as argument and end as current.
    fn get_range(&mut self, start : usize) -> data::Range {
        return data::Range {
            filename : self.filename.clone(),
            start    : start,
            end      : self.pos
        }
    }

    // Start lexing.
    pub fn lex(&mut self) -> LexerResult {
        let mut tokens : Vec<tokens::Token> = vec![];

        // Repeat until all characters have been passed through.
        while ! self.end {

            // Ignore spaces and tabs.
            if [' ', '\t'].contains(&self.ch) {
                self.advance();
            }

            
            // Line feed and carriage return mean newline.
            else if ['\n', '\r'].contains(&self.ch) {
                tokens.push(tokens::Token {
                    name  : tokens::TK_EOL.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            // Variables, Header functions, and Functions.
            else if data::ALPHABETIC.contains(self.ch) {
                let mut identifier = "".to_string();
                let     start      = self.pos;
                while (! self.end) && ((data::ALPHABETIC.to_string() + "_").contains(self.ch)) {
                    identifier += self.ch.to_string().as_str();
                    self.advance();
                }
                if data::HEADFUNCS.contains(&identifier.as_str()) {
                    tokens.push(tokens::Token {
                        name  : tokens::TK_HEADFUNC.to_string(),
                        value : identifier.to_string(),
                        range : self.get_range(start)
                    });
                } else if data::FUNCTIONS.contains(&identifier.as_str()) {
                    tokens.push(tokens::Token {
                        name  : tokens::TK_FUNCTION.to_string(),
                        value : identifier.to_string(),
                        range : self.get_range(start)
                    })
                } else {
                    for (i, ch) in identifier.chars().enumerate() {
                        tokens.push(tokens::Token {
                            name  : tokens::TK_VARIABLE.to_string(),
                            value : ch.to_string(),
                            range : data::Range {
                                filename : self.filename.clone(),
                                start    : start + i,
                                end      : start + i
                            }
                        });
                    }
                };
            }


            // Numbers.
            else if data::NUMERIC.contains(self.ch) {
                let mut num   = "".to_string();
                let mut dots  = 0;
                let     start = self.pos;
                while (! self.end) && ((data::NUMERIC.to_string() + "._").contains(self.ch)) {
                    if self.ch == '.' {
                        if dots >= 1 {
                            break
                        };
                        dots += 1;
                        num += ".";
                        self.advance();
                        continue
                    } else if self.ch == '_' {
                        self.advance();
                        continue;
                    }
                    num += self.ch.to_string().as_str();
                    self.advance();
                }
                tokens.push(tokens::Token {
                    name  : tokens::TK_NUMBER.to_string(),
                    value : num,
                    range : self.get_range(start)
                });
            }


            // Strings.
            else if self.ch == '"' {
                let mut string = "".to_string();
                let     start  = self.pos;

                let mut escchars = HashMap::new();
                escchars.insert('\\' , "\\");
                escchars.insert('n'  , "\n");
                escchars.insert('\n' , "\n");
                escchars.insert('t'  , "\t");
                escchars.insert('\'' , "\'");
                escchars.insert('\"' , "\"");
                let mut escaped = false;

                self.advance();

                while (! self.end) && (self.ch != '"' || escaped) {
                    if escaped {
                        if ! escchars.contains_key(&self.ch) {
                            return LexerResult {
                                success   : false,
                                tokens    : tokens,
                                exception : exceptions::LexerException {
                                    base    : exceptions::LexerExceptionBase::EscapeException,
                                    message : format!("Can not escape charater: `{}`.", data::escapify(self.ch.to_string())),
                                    range   : self.get_range(self.pos)
                                }
                            }
                        }

                        string += escchars.get(&self.ch).unwrap();
                        escaped = false;

                    } else {
                        if self.ch == '\\' {
                            escaped = true;
                        } else if self.ch == '\n' {
                            return LexerResult {
                                success   : false,
                                tokens    : tokens,
                                exception : exceptions::LexerException {
                                    base    : exceptions::LexerExceptionBase::EndException,
                                    message : format!("Invalid EOL."),
                                    range   : self.get_range(self.pos)
                                }
                            };
                        } else {
                            string += self.ch.to_string().as_str();
                        }
                    }
                    self.advance();
                }

                if self.end {
                    return LexerResult {
                        success   : false,
                        tokens    : tokens,
                        exception : exceptions::LexerException {
                            base    : exceptions::LexerExceptionBase::EndException,
                            message : format!("Invalid EOF."),
                            range   : self.get_range(self.pos)
                        }
                    };
                }

                tokens.push(tokens::Token {
                    name  : tokens::TK_STRING.to_string(),
                    value : string,
                    range : self.get_range(start)
                });

                self.advance();
            }


            // Operation Characters.


            else if self.ch == '=' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_EQUALS.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == '+' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_ADD.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == '-' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_SUBTRACT.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == '*' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_MULTIPLY.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == '/' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_DIVIDE.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            // Parenthesis.


            else if self.ch == '(' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_LPAREN.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == ')' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_RPAREN.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            // Header function hash.
            else if self.ch == '#' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_HEADER.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            // Comma.
            else if self.ch == ',' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_COMMA.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            // Unknown characters result in error.
            else {
                return LexerResult {
                    success   : false,
                    tokens    : tokens,
                    exception : exceptions::LexerException {
                        base    : exceptions::LexerExceptionBase::IllegalCharacterException,
                        message : format!("Illegal character `{}` was found.", data::escapify(self.ch.to_string())).to_string(),
                        range   : self.get_range(self.pos)
                    }
                }
            }



        }

        // End with EOL and EOF for easier parsing.

        tokens.push(tokens::Token {
            name  : tokens::TK_EOL.to_string(),
            value : "".to_string(),
            range : self.get_range(self.pos)
        });

        tokens.push(tokens::Token {
            name  : tokens::TK_EOF.to_string(),
            value : "".to_string(),
            range : self.get_range(self.pos)
        });

        // Return list of tokens.

        return LexerResult {
            success     : true,
            tokens      : tokens,
            exception   : exceptions::LexerException {
                base    : exceptions::LexerExceptionBase::NoException,
                message : "".to_string(),
                range   : self.get_range(self.pos)
            }
        }

    }
}



// Function for lexing a string.
pub fn lex(filename: String, script: String) -> LexerResult {
    let mut lexer = Lexer {
        filename : filename,
        script   : script.clone(),
        chars    : script.clone().chars().collect(),
        pos      : 0,
        ch       : ' ',
        end      : false
    };
    lexer.init();

    return lexer.lex();
}
