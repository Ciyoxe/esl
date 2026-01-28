use std::ops::Range;
use crate::parser::primitives::*;

#[derive(Debug, Clone)]
pub enum NodeKind {
    // primitives
    IntegerLiteral(IntegerLiteral),
    FloatingLiteral(FloatingLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Identifier(Identifier),
    DontCare(DontCare),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub range: Range<usize>,
}

impl Node {
    #[inline]
    pub fn new(kind: NodeKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }

    #[inline]
    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        match &self.kind {
            _ => (),
        }
    }

    #[inline]
    pub fn has_errors(&self) -> bool {
        match &self.kind {
            NodeKind::IntegerLiteral(v) => v.has_errors(),
            _ => false,
        }
    }

    #[inline]
    pub fn visit_errors(&self, visit: impl FnMut(&'static str)) {
        match &self.kind {
            NodeKind::IntegerLiteral(v) => v.visit_errors(visit),
            _ => (),
        }
    }
}
