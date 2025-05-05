pub mod expressions;
pub mod node;
pub mod operations;

use crate::tokenizer::token::{Token, TokenKind};
use expressions::ExpressionParser;
use node::{Node, NodeKind};

pub struct Parser<'a> {
    pub pos: usize,
    pub src: &'a [u8],
    pub tokens: &'a [Token],
}

/*************************************************
 *               PUBLIC INTERFACE                *
 *************************************************/
impl<'a> Parser<'a> {
    pub fn new(src: &'a [u8], tokens: &'a [Token]) -> Self {
        Self {
            pos: 0,
            src,
            tokens,
        }
    }
}

/*************************************************
 *                    HELPERS                    *
 *************************************************/
impl<'a> Parser<'a> {
    fn next(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn next_at(&self, pos: usize) -> Option<&Token> {
        self.tokens.get(self.pos + pos)
    }
    fn next_unwrap(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn mov(&mut self) {
        self.pos += 1;
    }

    fn make_node(&mut self, f: impl FnOnce(&mut Self) -> Option<NodeKind>) -> Option<Node> {
        let pos = self.pos;
        let node = f(self);

        if let Some(node) = node {
            return Some(Node {
                kind: node,
                range: pos..self.pos,
            });
        }
        None
    }
}

/*************************************************
 *                 NODES PARSING                 *
 *************************************************/
impl<'a> Parser<'a> {
    pub fn p_primitive(&mut self) -> Option<Node> {
        if self.next().is_none() {
            return None;
        }

        self.make_node(|this| {
            let next_token = this.next_unwrap();
            let src_bytes = &this.src[next_token.range.clone()];

            match next_token.kind {
                TokenKind::NumBinInt => {
                    this.mov();

                    let slice = std::str::from_utf8(&src_bytes[2..]).unwrap();
                    let value = u64::from_str_radix(slice, 2);

                    match value {
                        Ok(value) => Some(NodeKind::IntegerLiteral { value }),
                        Err(_) => Some(NodeKind::ErrNumberOverflow),
                    }
                }
                TokenKind::NumHexInt => {
                    this.mov();

                    let slice = std::str::from_utf8(&src_bytes[2..]).unwrap();
                    let value = u64::from_str_radix(slice, 16);

                    match value {
                        Ok(value) => Some(NodeKind::IntegerLiteral { value }),
                        Err(_) => Some(NodeKind::ErrNumberOverflow),
                    }
                }
                TokenKind::NumDecInt => {
                    this.mov();

                    let slice = std::str::from_utf8(src_bytes).unwrap();
                    let value = slice.parse();

                    match value {
                        Ok(value) => Some(NodeKind::IntegerLiteral { value }),
                        Err(_) => Some(NodeKind::ErrNumberOverflow),
                    }
                }
                TokenKind::NumDecFloat => {
                    this.mov();

                    let slice = std::str::from_utf8(src_bytes).unwrap();
                    let value = slice.parse();

                    match value {
                        Ok(value) => Some(NodeKind::FloatingLiteral { value }),
                        Err(_) => Some(NodeKind::ErrNumberOverflow),
                    }
                }
                TokenKind::KwTrue => {
                    this.mov();

                    Some(NodeKind::BooleanLiteral { value: true })
                }
                TokenKind::KwFalse => {
                    this.mov();

                    Some(NodeKind::BooleanLiteral { value: false })
                }
                TokenKind::String => {
                    this.mov();

                    // TODO: handle escape sequences and missing quotes
                    Some(NodeKind::StringLiteral {
                        value: src_bytes.to_vec(),
                    })
                }
                TokenKind::Identifier => {
                    this.mov();

                    Some(NodeKind::Identifier {
                        name: src_bytes.to_vec(),
                    })
                }

                _ => None,
            }
        })
    }

    pub fn p_expression_atom(&mut self) -> Option<Node> {
        self.p_primitive()
    }

    pub fn p_expression(&mut self) -> Option<Node> {
        let mut parser = ExpressionParser::new();

        None
    }
}
