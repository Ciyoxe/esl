use crate::{parser::errors::ParsingError, tokenizer::token::TokenKind};

use super::nodes::*;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: u64,
    pub error: Option<Box<Node>>,
}

impl IntegerLiteral {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        if let Some(error) = self.error.as_ref() {
            visit(error);
        }
    }
}

impl Parser<'_> {
    pub fn p_integer_literal(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let literal = match this.next().map(|token| &token.kind) {
                Some(TokenKind::NumBinInt) => {
                    let token = this.advance().clone();
                    let bytes = this.get_src(token.range.clone());
                    let value = u64::from_str_radix(std::str::from_utf8(bytes).unwrap(), 2);

                    match value {
                        Ok(v) => IntegerLiteral {
                            value: v,
                            error: None,
                        },
                        Err(_) => IntegerLiteral {
                            value: 0,
                            error: Some(Box::new(
                                this.make_error_for_tokens(ParsingError::IntegerOverflow, &[token]),
                            )),
                        },
                    }
                }
                Some(TokenKind::NumHexInt) => {
                    let token = this.advance().clone();
                    let bytes = this.get_src(token.range.clone());
                    let value = u64::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16);

                    match value {
                        Ok(v) => IntegerLiteral {
                            value: v,
                            error: None,
                        },
                        Err(_) => IntegerLiteral {
                            value: 0,
                            error: Some(Box::new(
                                this.make_error_for_tokens(ParsingError::IntegerOverflow, &[token]),
                            )),
                        },
                    }
                }
                Some(TokenKind::NumDecInt) => {
                    let token = this.advance().clone();
                    let bytes = this.get_src(token.range.clone());
                    let value = str::parse::<u64>(std::str::from_utf8(bytes).unwrap());

                    match value {
                        Ok(v) => IntegerLiteral {
                            value: v,
                            error: None,
                        },
                        Err(_) => IntegerLiteral {
                            value: 0,
                            error: Some(Box::new(
                                this.make_error_for_tokens(ParsingError::IntegerOverflow, &[token]),
                            )),
                        },
                    }
                }
                _ => return None,
            };
            Some(NodeKind::IntegerLiteral(literal))
        })
    }
}

#[derive(Debug, Clone)]
pub struct FloatingLiteral {
    pub value: f64,
}

impl Parser<'_> {
    pub fn p_floating_literal(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let literal = match this.next().map(|token| &token.kind) {
                Some(TokenKind::NumDecFloat) => {
                    let range = this.advance().range.clone();
                    let bytes = this.get_src(range);
                    let value = str::parse::<f64>(std::str::from_utf8(bytes).unwrap()).unwrap();

                    FloatingLiteral { value }
                }
                _ => return None,
            };
            Some(NodeKind::FloatingLiteral(literal))
        })
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl Parser<'_> {
    pub fn p_string_literal(&mut self) -> Option<Node> {
        self.make_node(|this| {
            // TODO: add escaping chars handling, add interpolation
            let literal = match this.next().map(|token| &token.kind) {
                Some(TokenKind::String) => {
                    let range = this.advance().range.clone();
                    let bytes = this.get_src(range.start + 1..range.end - 1);

                    StringLiteral {
                        value: String::from_utf8(bytes.to_vec()).unwrap(),
                    }
                }
                _ => return None,
            };
            Some(NodeKind::StringLiteral(literal))
        })
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl Parser<'_> {
    pub fn p_boolean_literal(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let literal = match this.next().map(|token| &token.kind) {
                Some(TokenKind::KwTrue) => BooleanLiteral { value: true },
                Some(TokenKind::KwFalse) => BooleanLiteral { value: false },
                _ => return None,
            };
            this.advance();
            Some(NodeKind::BooleanLiteral(literal))
        })
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl Parser<'_> {
    pub fn p_identifier(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let ident = match this.next().map(|token| &token.kind) {
                Some(TokenKind::Identifier) => {
                    let range = this.advance().range.clone();
                    let bytes = this.get_src(range);
                    Identifier {
                        value: String::from_utf8(bytes.to_owned()).unwrap(),
                    }
                }
                _ => return None,
            };
            Some(NodeKind::Identifier(ident))
        })
    }
}

#[derive(Debug, Clone)]
pub struct DontCare {}

impl Parser<'_> {
    pub fn p_dont_care(&mut self) -> Option<Node> {
        self.make_node(|this| match this.next().map(|token| &token.kind) {
            Some(TokenKind::Ignore) => {
                this.advance();
                Some(NodeKind::DontCare(DontCare {}))
            }
            _ => None,
        })
    }
}
