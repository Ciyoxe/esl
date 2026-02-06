pub mod debugger;
pub mod errors;
pub mod expressions;
pub mod nodes;
pub mod primitives;
pub mod statements;
pub mod expressionables;

use crate::tokenizer::token::{Token, TokenKind};
use errors::*;
use nodes::*;

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

    #[inline(always)]
    pub fn next(&self) -> Option<&Token> {
        self.tks.get(self.pos)
    }

    #[inline(always)]
    pub fn next_unwrap(&self) -> &Token {
        &self.tks[self.pos]
    }

    #[inline(always)]
    pub fn next_at(&self, n: usize) -> Option<&Token> {
        self.tks.get(self.pos + n)
    }

    #[inline(always)]
    pub fn advance(&mut self) -> &Token {
        self.pos += 1;
        &self.tks[self.pos - 1]
    }

    #[inline(always)]
    pub fn advance_on(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.next() {
            if token.kind == kind {
                self.advance();
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn rollback(&mut self) {
        if self.pos == 0 {
            unreachable!()
        }

        self.pos -= 1;
    }

    #[inline]
    pub fn make_node(&mut self, f: impl FnOnce(&mut Self) -> Option<NodeKind>) -> Option<Node> {
        let start_token = self.pos;
        let node = f(self)?;

        if self.pos <= start_token {
            panic!("This may be a bug in the parser, node is returned, but no tokens consumed")
        }

        let start = self.tks.get(start_token).map_or(0, |t| t.range.start);
        let end = self
            .tks
            .get(self.pos - 1)
            .map_or(self.src.len(), |t| t.range.end)
            .max(start);

        Some(Node {
            kind: node,
            range: start..end,
        })
    }

    #[inline]
    pub fn make_error_for_tokens(&self, err: ParsingError, tokens: &[Token]) -> Node {
        let start = tokens.first().map_or(0, |t| t.range.start);
        let end = tokens.last().map_or(0, |t| t.range.end).max(start);
        Node {
            kind: NodeKind::Error(err),
            range: start..end,
        }
    }

    #[inline]
    pub fn make_error_for_nodes(&self, err: ParsingError, nodes: &[Node]) -> Node {
        let start = nodes.first().map_or(0, |n| n.range.start);
        let end = nodes.last().map_or(0, |n| n.range.end).max(start);
        Node {
            kind: NodeKind::Error(err),
            range: start..end,
        }
    }

    #[inline]
    pub fn make_error_here(&self, err: ParsingError) -> Node {
        let start = if self.pos > 0 {
            self.tks[self.pos - 1].range.end
        } else {
            0
        };
        let end = self.tks.get(self.pos).map(|t| t.range.start).unwrap_or(self.src.len());
        Node {
            kind: NodeKind::Error(err),
            range: start..end,
        }
    }

    #[inline]
    pub fn make_error_before_token_at(&self, err: ParsingError, token_idx: usize) -> Node {
        let start = if token_idx > 0 {
            self.tks[token_idx - 1].range.end
        } else {
            0
        };
        let end = self.tks.get(token_idx).map(|t| t.range.start).unwrap_or(self.src.len());
        Node {
            kind: NodeKind::Error(err),
            range: start..end,
        }
    }
}
