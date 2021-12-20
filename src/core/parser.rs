use super::nodes;
use super::exceptions;
use super::tokens;
use super::data;



// Success/Failure identification class for parser.
#[derive(Clone, Debug)]
pub struct ParserResult {
    pub success   : bool,
    pub nodes     : Vec<nodes::Node>,
    pub exception : exceptions::ParserException
}

// Success/Failure identification class for getting header function arguments.
#[derive(Clone, Debug)]
pub struct HeaderArgsResult {
    pub success   : bool,
    pub args      : Vec<i32>,
    pub exception : exceptions::ParserException
}



// Parser class
#[derive(Clone, Debug)]
pub struct Parser {
    tokens : Vec<tokens::Token>,
    pos  : usize,
    token  : tokens::Token
}
impl Parser {
    // Initialize variables on creation.
    fn init(&mut self) {
        self.pos = 0;
        self.token = self.tokens[self.pos].clone();
    }



    // Move to next token.
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.tokens.len() {
            self.token = self.tokens[self.pos].clone();
        }
    }
    // Ease of use function for returning a list of nodes.
    fn success(&mut self, nodes: Vec<nodes::Node>) -> ParserResult {
        return ParserResult {
            success   : true,
            nodes     : nodes,
            exception : exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::NoException,
                message : "".to_string(),
                range   : data::Range {
                    filename : "".to_string(),
                    start    : 0,
                    end      : 0
                }
            }
        }
    }
    // Ease of use function for returning an exception.
    fn failure(&mut self, exception: exceptions::ParserException) -> ParserResult {
        return ParserResult {
            success   : false,
            nodes     : vec![],
            exception : exception
        }
    }



    // Start parsing.
    fn parse(&mut self) -> ParserResult {
        let mut nodes = vec![];
        // Repeat until all tokens have been passed through.
        while self.token.name != tokens::TK_EOF {
            // Ignore extra newlines.
            while self.token.name == tokens::TK_EOL {
                self.advance();
            }
            // Check for end of file.
            if self.token.name == tokens::TK_EOF {break}
            let res;


            // If header, parse header function. Else, parse expression.
            if self.token.name == tokens::TK_HEADER {
                res = self.header();
            } else {
                res = self.expression();
            }
            if ! res.success {
                return res;
            }
            // Add new node.
            nodes.append(&mut res.nodes.clone());
            // Next line.
            if self.token.name != tokens::TK_EOL {
                return ParserResult {
                    success   : false,
                    nodes     : vec![],
                    exception : exceptions::ParserException {
                        base    : exceptions::ParserExceptionBase::MissingTokenException,
                        message : "Expected (EOL) not found.".to_string(),
                        range   : self.token.range.clone()
                    }
                }
            }
            self.advance();
        };
        // Return list of nodes.
        return ParserResult {
            success   : true,
            nodes     : nodes,
            exception : exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::NoException,
                message : "".to_string(),
                range   : data::Range {
                    filename : "".to_string(),
                    start    : 0,
                    end      : 0
                }
            }
        };
    }





    // Header found.
    fn header(&mut self) -> ParserResult {
        self.advance();
        // Look for header function name.
        if self.token.name != tokens::TK_HEADFUNC {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (HeadFunc) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        let func = self.token.value.clone();
        self.advance();
        // Look for opening parenthesis.
        if self.token.name != tokens::TK_LPAREN {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (LeftParen) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        self.advance();
        // Idenfify header function and get arguments.
        let res = match func.as_str() {

            "frame"      => self.header_frame(),
            "resolution" => self.header_resolution(),
            "export"     => self.header_export(),
            "print_now"  => self.header_print_now(),

            _       => return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::InternalException,
                message : format!("Invalid header function found: `{}`", func),
                range   : self.token.range.clone()
            })

        };
        if ! res.success {
            return res;
        }
        // Look for closing parenthesis.
        if self.token.name != tokens::TK_RPAREN {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (RightParen) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        self.advance();
        return res;
    }


    // Frame header function found.
    fn header_frame(&mut self) -> ParserResult {
        let range = self.token.range.clone();
        // Get 4 number arguments.
        let res = self.header_get_args(4);
        if ! res.success {
            return self.failure(res.exception);
        }
        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::HeaderFuncFrame {
                x : res.args[0],
                y : res.args[1],
                w : res.args[2],
                h : res.args[3]
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }


    // Resolution header function found.
    fn header_resolution(&mut self) -> ParserResult {
        let range = self.token.range.clone();
        // Get 2 number arguments.
        let res = self.header_get_args(2);
        if ! res.success {
            return self.failure(res.exception);
        }
        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::HeaderFuncResolution {
                w : res.args[0],
                h : res.args[1]
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }


    // Export header function found.
    fn header_export(&mut self) -> ParserResult {
        let range = self.token.range.clone();
        // Get 1 string argument.
        if self.token.name != tokens::TK_STRING {
            return self.failure(
                exceptions::ParserException {
                    base    : exceptions::ParserExceptionBase::MissingTokenException,
                    message : "Expected (String) not found.".to_string(),
                    range   : self.token.range.clone()
                }
            );
        }
        let filename = self.token.value.clone();
        self.advance();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::HeaderFuncExport {
                filename : filename
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }


    // Export header function found.
    fn header_print_now(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::HeaderFuncPrintNow,
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }


    // Get correct number of header function arguments, separated by commas.
    fn header_get_args(&mut self, arg_count : usize) -> HeaderArgsResult {
        let mut args  : Vec<i32> = vec![];
        for i in 0..arg_count {
            let mut multiplier : i32 = 1;
            if self.token.name == tokens::TK_SUBTRACT {
                multiplier = -1;
                self.advance();
            }
            if self.token.name != tokens::TK_NUMBER {
                return HeaderArgsResult {
                    success   : false,
                    args      : vec![],
                    exception : exceptions::ParserException {
                        base    : exceptions::ParserExceptionBase::MissingTokenException,
                        message : "Expected (Integer, Minus) not found.".to_string(),
                        range   : self.token.range.clone()
                    }
                };
            }
            args.push(self.token.value.parse::<i32>().unwrap() * multiplier);
            self.advance();
            if i < arg_count - 1 {
                if self.token.name != tokens::TK_COMMA {
                    return HeaderArgsResult {
                        success   : false,
                        args      : vec![],
                        exception : exceptions::ParserException {
                            base    : exceptions::ParserExceptionBase::MissingTokenException,
                            message : "Expected (Comma) not found.".to_string(),
                            range   : self.token.range.clone()
                        }
                    }
                }
                self.advance();
            }
        }
        return HeaderArgsResult {
            success   : true,
            args      : args,
            exception : exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::NoException,
                message : "".to_string(),
                range   : data::Range {
                    filename : "".to_string(),
                    start    : 0,
                    end      : 0
                }
            }
        };
    }





    // Function found.
    fn function(&mut self) -> ParserResult {
        // Look for function name.
        if self.token.name != tokens::TK_FUNCTION {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (Function) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        let func = self.token.value.clone();
        self.advance();
        // Look for opening parenthesis.
        if self.token.name != tokens::TK_LPAREN {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (LeftParen) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        self.advance();
        // Idenfify function and get arguments.
        let res = match func.as_str() {

            "sin"  => self.function_sin(),
            "cos"  => self.function_cos(),
            "tan"  => self.function_tan(),
            "pow"  => self.function_pow(),
            "root" => self.function_root(),

            _       => return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::InternalException,
                message : format!("Invalid function found: `{}`", func),
                range   : self.token.range.clone()
            })

        };
        if ! res.success {
            return res;
        }
        // Look for closing parenthesis.
        if self.token.name != tokens::TK_RPAREN {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (RightParen) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        self.advance();
        return res;
    }



    // Sin function found.
    fn function_sin(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        // Get 1 expression argument.
        let res = self.term();
        if ! res.success {
            return res;
        }
        let term = res.nodes[0].clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::FunctionSin {
                a : Box::new(term)
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }



    // Cos function found.
    fn function_cos(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        // Get 1 expression argument.
        let res = self.term();
        if ! res.success {
            return res;
        }
        let term = res.nodes[0].clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::FunctionCos {
                a : Box::new(term)
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }



    // Tan function found.
    fn function_tan(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        // Get 1 expression argument.
        let res = self.term();
        if ! res.success {
            return res;
        }
        let term = res.nodes[0].clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::FunctionTan {
                a : Box::new(term)
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }



    // Pow function found.
    fn function_pow(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        // Get 2 expression arguments.
        let mut res = self.term();
        if ! res.success {
            return res;
        }
        let base = res.nodes[0].clone();
        if self.token.name != tokens::TK_COMMA {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (Comma) not found.".to_string(),
                range   : self.token.range.clone()
            })
        }
        self.advance();
        res = self.term();
        if ! res.success {
            return res;
        }
        let exp = res.nodes[0].clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::FunctionPow {
                    base : Box::new(base),
                    exp  : Box::new(exp)
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }



    // Root function found.
    fn function_root(&mut self) -> ParserResult {
        let range = self.token.range.clone();

        // Get 2 expression arguments.
        let mut res = self.term();
        if ! res.success {
            return res;
        }
        let exp = res.nodes[0].clone();
        if self.token.name != tokens::TK_COMMA {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (Comma) not found.".to_string(),
                range   : self.token.range.clone()
            })
        }
        self.advance();
        res = self.term();
        if ! res.success {
            return res;
        }
        let base = res.nodes[0].clone();

        return self.success(vec![nodes::Node {
            base : nodes::NodeBase::FunctionRoot {
                exp        : Box::new(exp),
                base       : Box::new(base),
                user_typed : true
            },
            range : data::Range {
                filename : range.filename,
                start    : range.start,
                end      : self.token.range.end
            }
        }]);
    }





    // term (= term)?
    fn expression(&mut self) -> ParserResult {
        let mut res;
        res = self.term();
        if ! res.success {
            return res;
        }
        let mut expression = res.nodes[0].clone();
        if self.token.name == tokens::TK_EQUALS {
            self.advance();
            res = self.term();
            if ! res.success {
                return res;
            }
        } else {
            res = ParserResult {
                success   : true,
                nodes     : vec![nodes::Node {
                    base : nodes::NodeBase::Variable {
                        name : "y".to_string()
                    },
                    range : expression.range.clone()
                }],
                exception : exceptions::ParserException {
                    base    : exceptions::ParserExceptionBase::NoException,
                    message : "".to_string(),
                    range   : data::Range {
                        filename : "".to_string(),
                        start    : 0,
                        end      : 0
                    }
                }
            };
        }
        expression = nodes::Node {
            base : nodes::NodeBase::EqualsExpression {
                left  : Box::new(expression.clone()),
                right : Box::new(res.nodes[0].clone())
            },
            range : data::Range {
                filename : expression.range.filename,
                start    : expression.range.start,
                end      : res.nodes[0].range.end
            }
        };
        return self.success(vec![expression]);
    }



    // addition_term
    fn term(&mut self) -> ParserResult {
        return self.addition_term();
    }



    // multiplication_term ((\+|-) multiplication_term)*
    fn addition_term(&mut self) -> ParserResult {
        let mut res;
        res = self.multiplication_term();
        if ! res.success {
            return res;
        }
        let mut term = res.nodes[0].clone();
        while [tokens::TK_ADD, tokens::TK_SUBTRACT].contains(&self.token.name.as_str()) {
            let token = self.token.clone();
            self.advance();
            res = self.multiplication_term();
            if ! res.success {
                return res;
            }
            let value = res.nodes[0].clone();
            if token.name == tokens::TK_SUBTRACT {
                term = term - value;
            } else {
                term = term + value;
            }
        }
        return self.success(vec![term]);
    }



    // literal_multiplication ((\*|\/) literal_multiplication)*
    fn multiplication_term(&mut self) -> ParserResult {
        let mut res;
        res = self.literal_multiplication();
        if ! res.success {
            return res;
        }
        let mut term = res.nodes[0].clone();
        while [tokens::TK_MULTIPLY, tokens::TK_DIVIDE].contains(&self.token.name.as_str()) {
            let token = self.token.clone();
            self.advance();
            res = self.literal_multiplication();
            if ! res.success {
                return res;
            }
            let value = res.nodes[0].clone();
            if token.name == tokens::TK_MULTIPLY {
                term = term * value;
            } else {
                term = term / value;
            }
        }
        return self.success(vec![term]);
    }



    // literal (literal_multiplication)*
    fn literal_multiplication(&mut self) -> ParserResult {
        let res = self.literal();
        if ! res.success {
            return res;
        }
        let mut node = res.nodes[0].clone();
        let res = self.literal_multiplication();
        if res.success {
            node = node * res.nodes[0].clone();
        }
        return self.success(vec![node]);
    }



    // (LPAREN term RPAREN) | (NUMBER) | (VARIABLE) | (function)
    fn literal(&mut self) -> ParserResult {
        let token = self.token.clone();
        let node;

        if token.name == tokens::TK_LPAREN {
            self.advance();
            let res = self.term();
            if ! res.success {
                return res;
            }
            if self.token.name != tokens::TK_RPAREN {
                return self.failure(exceptions::ParserException {
                    base    : exceptions::ParserExceptionBase::MissingTokenException,
                    message : "Expected (RightParen) not found.".to_string(),
                    range   : token.range
                });
            }
            self.advance();
            node = res.nodes[0].clone();
        }

        else if token.name == tokens::TK_NUMBER {
            self.advance();
            node = nodes::Node {
                base : nodes::NodeBase::Number {
                    value : token.value.parse().unwrap()
                },
                range : token.range
            };
        }

        else if token.name == tokens::TK_VARIABLE {
            self.advance();
            node = nodes::Node {
                base : nodes::NodeBase::Variable {
                    name  : token.value
                },
                range : token.range
            };
        }

        else if token.name == tokens::TK_FUNCTION {
            return self.function();
        }

        else {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::IllegalTokenException,
                message : "Expected (Literal, Variable, LeftParen) not found.".to_string(),
                range   : token.range
            });
        }

        return self.success(vec![node]);

    }



}



// Function for parsing a vector of tokens.
pub fn parse(tokens: Vec<tokens::Token>) -> ParserResult {
    let mut parser = Parser {
        tokens : tokens,
        pos    : 0,
        token  : tokens::Token {
            name  : tokens::TK_NULL.to_string(),
            value : "".to_string(),
            range : data::Range {
                filename : "".to_string(),
                start    : 0,
                end      : 0
            }
        }
    };
    parser.init();

    return parser.parse();
}
