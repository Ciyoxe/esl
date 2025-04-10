pub mod node;
pub mod operations;

use crate::tokenizer::token::{Token, TokenKind};
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
                    let src_slice = std::str::from_utf8(&src_bytes[2..]).unwrap();
                    Some(NodeKind::IntegerLiteral {
                        value: u64::from_str_radix(src_slice, 2).unwrap(),
                    })
                }
                TokenKind::NumHexInt => {
                    let src_slice = std::str::from_utf8(&src_bytes[2..]).unwrap();
                    Some(NodeKind::IntegerLiteral {
                        value: u64::from_str_radix(src_slice, 16).unwrap(),
                    })
                }
                TokenKind::NumDecInt => {
                    let src_slice = std::str::from_utf8(src_bytes).unwrap();
                    Some(NodeKind::IntegerLiteral {
                        value: src_slice.parse().unwrap(),
                    })
                }
                TokenKind::NumDecFloat => {
                    let src_slice = std::str::from_utf8(src_bytes).unwrap();
                    Some(NodeKind::FloatingLiteral {
                        value: src_slice.parse().unwrap(),
                    })
                }
                TokenKind::KwTrue => Some(NodeKind::BooleanLiteral { value: true }),
                TokenKind::KwFalse => Some(NodeKind::BooleanLiteral { value: false }),
                TokenKind::String => Some(NodeKind::StringLiteral { value: src_bytes.to_vec() }),
                TokenKind::Identifier => Some(NodeKind::Identifier { name: src_bytes.to_vec() }),

                _ => None,
            }
        })
    }
}
