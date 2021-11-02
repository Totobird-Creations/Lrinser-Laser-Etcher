use super::data;
use super::tokens;
use super::exceptions;



#[derive(Clone, Debug)]
pub struct LexerResult {
    pub success   : bool,
    pub tokens    : Vec<tokens::Token>,
    pub exception : exceptions::LexerException
}



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
    pub fn init(&mut self) {
        if self.script.len() >= 1 {
            self.ch = self.chars[0];
            self.end = false;
        } else {
            self.ch = ' ';
            self.end = true;
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.chars.len() {
            self.ch = self.chars[self.pos];
        } else {
            self.ch = ' ';
            self.end = true;
        }
    }

    fn get_range(&mut self, start : usize) -> data::Range {
        return data::Range {
            filename : self.filename.clone(),
            start    : start,
            end      : self.pos
        }
    }

    pub fn lex(&mut self) -> LexerResult {
        let mut tokens : Vec<tokens::Token> = vec![];

        while ! self.end {

            if [' ', '\t'].contains(&self.ch) {
                self.advance();
            }


            else if self.ch == '\n' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_EOL.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


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


            else if data::NUMERIC.contains(self.ch) {
                let mut num   = "".to_string();
                let     start = self.pos;
                while (! self.end) && ((data::NUMERIC.to_string() +  "_").contains(self.ch)) {
                    if self.ch == '_' {
                        self.advance();
                        continue;
                    }
                    num += self.ch.to_string().as_str();
                    self.advance();
                }
                tokens.push(tokens::Token {
                    name  : tokens::TK_INTEGER.to_string(),
                    value : num,
                    range : self.get_range(start)
                });
            }


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


            else if self.ch == '#' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_HEADER.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


            else if self.ch == ',' {
                tokens.push(tokens::Token {
                    name  : tokens::TK_COMMA.to_string(),
                    value : "".to_string(),
                    range : self.get_range(self.pos)
                });
                self.advance();
            }


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
