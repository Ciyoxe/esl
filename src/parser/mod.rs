pub mod nodes;
pub mod primitives;
pub mod expressions;
pub mod debugger;

use nodes::*;
use crate::tokenizer::token::{Token, TokenKind};

pub struct Parser<'a> {
    pub pos: usize,
    pub src: &'a [u8],
    pub tks: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a [u8], tokens: &'a [Token]) -> Self {
        Self {
            pos: 0,
            src,
            tks: tokens,
        }
    }

    pub fn get_src(&self, range: std::ops::Range<usize>) -> &'a [u8] {
        &self.src[range]
    }

    pub fn next(&self) -> Option<&Token> {
        self.tks.get(self.pos)
    }

    pub fn next_at(&self, n: usize) -> Option<&Token> {
        self.tks.get(self.pos + n)
    }

    pub fn advance(&mut self) -> &Token {
        self.pos += 1;
        &self.tks[self.pos - 1]
    }

    pub fn advance_on(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.next() {
            if token.kind == kind {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn rollback(&mut self) {
        self.pos -= 1;
    }

    #[inline]
    pub fn make_node(&mut self, f: impl FnOnce(&mut Self) -> Option<NodeKind>) -> Option<Node> {
        let pos = self.pos;
        let tok = f(self);

        if let Some(tok) = tok {
            return Some(Node {
                kind: tok,
                range: pos..self.pos,
            });
        }
        None
    }
}
