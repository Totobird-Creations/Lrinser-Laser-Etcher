use super::nodes;
use super::exceptions;
use super::tokens;
use super::data;



#[derive(Clone, Debug)]
pub struct ParserResult {
    pub success   : bool,
    pub nodes     : Vec<nodes::Node>,
    pub exception : exceptions::ParserException
}

#[derive(Clone, Debug)]
pub struct HeaderArgsResult {
    pub success   : bool,
    pub args      : Vec<i32>,
    pub exception : exceptions::ParserException
}



#[derive(Clone, Debug)]
pub struct Parser {
    tokens : Vec<tokens::Token>,
    pos  : usize,
    token  : tokens::Token
}
impl Parser {
    fn init(&mut self) {
        self.pos = 0;
        self.token = self.tokens[self.pos].clone();
    }



    /**************************************************\
    | TOOL METHODS                                     |
    \**************************************************/

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.tokens.len() {
            self.token = self.tokens[self.pos].clone();
        }
    }
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
    fn failure(&mut self, exception: exceptions::ParserException) -> ParserResult {
        return ParserResult {
            success   : false,
            nodes     : vec![],
            exception : exception
        }
    }



    /**************************************************\
    | HANDLES EOLs and EOFs                            |
    \**************************************************/

    fn parse(&mut self) -> ParserResult {
        let mut nodes = vec![];
        while self.token.name != tokens::TK_EOF {
            while self.token.name == tokens::TK_EOL {
                self.advance();
            }
            if self.token.name == tokens::TK_EOF {break}
            let res;


            if self.token.name == tokens::TK_HEADER {
                res = self.header();
            } else {
                res = self.expression();
            }


            if ! res.success {
                return res;
            }
            nodes.append(&mut res.nodes.clone());
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



    /**************************************************\
    | HEADERS                                          |
    \**************************************************/



    fn header(&mut self) -> ParserResult {
        self.advance();
        if self.token.name != tokens::TK_HEADFUNC {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (HeadFunc) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        let func = self.token.value.clone();
        self.advance();
        if self.token.name != tokens::TK_LPAREN {
            return self.failure(exceptions::ParserException {
                base    : exceptions::ParserExceptionBase::MissingTokenException,
                message : "Expected (LeftParen) not found.".to_string(),
                range   : self.token.range.clone()
            });
        }
        self.advance();
        let res = match func.as_str() {

            "frame"      => self.header_frame(),
            "resolution" => self.header_resolution(),

            _       => panic!("Unknown header function: `{}`", func)

        };
        if ! res.success {
            return res;
        }
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



    fn header_frame(&mut self) -> ParserResult {
        let range = self.token.range.clone();
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



    fn header_resolution(&mut self) -> ParserResult {
        let range = self.token.range.clone();
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



    fn header_get_args(&mut self, arg_count : usize) -> HeaderArgsResult {
        let mut args  : Vec<i32> = vec![];
        for i in 0..arg_count {
            let mut multiplier : i32 = 1;
            if self.token.name == tokens::TK_SUBTRACT {
                multiplier = -1;
                self.advance();
            }
            if self.token.name != tokens::TK_INTEGER {
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



    /**************************************************\
    | TERMS AND EQUATIONS                              |
    \**************************************************/



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



    fn term(&mut self) -> ParserResult {
        return self.addition_term();
    }



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

        else if token.name == tokens::TK_INTEGER {
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
