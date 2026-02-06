use std::ops::Range;
use crate::parser::{
    errors::*,
    expressions::*,
    primitives::*, statements::ValueDeclaration,
};

#[derive(Debug, Clone)]
pub enum NodeKind {
    // primitives
    IntegerLiteral(IntegerLiteral),
    FloatingLiteral(FloatingLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Identifier(Identifier),
    DontCare(DontCare),
    Error(ParsingError),

    // Expressions
    Operation(Operation),
    Expression(Expression),

    // Statements
    ValueDeclaration(ValueDeclaration),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub range: Range<usize>,
}

impl Node {
    pub fn new(kind: NodeKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }

    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        match &self.kind {
            NodeKind::Operation(v) => v.visit_children(visit),
            NodeKind::Expression(v) => v.visit_children(visit),
            _ => (),
        }
    }
}
