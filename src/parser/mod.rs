pub mod nodes;
pub use nodes::*;

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

    pub fn lookahead(&self, n: usize) -> Option<&Token> {
        self.tks.get(self.pos + n)
    }

    pub fn advance(&mut self) -> &Token {
        self.pos += 1;
        &self.tks[self.pos - 1]
    }

    pub fn advance_if(&mut self, kind: TokenKind) -> bool {
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
}
