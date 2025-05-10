pub mod primitives;
pub use primitives::*;

pub mod expressions;
pub use expressions::*;

pub mod operations;
pub use operations::*;

pub mod debugger;
pub use debugger::*;

use super::Parser;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum NodeKind {
    // primitives
    IntegerLiteral(IntegerLiteral),
    FloatingLiteral(FloatingLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Identifier(Identifier),
    DontCare(DontCare),
    Void(Void),

    // expressions
    Operation(Operation),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub range: Range<usize>,
}

pub trait INode: Sized {
    fn parse(parser: &mut Parser) -> Option<Self>;    
    fn into_node(self) -> NodeKind;
    
    fn visit_children<'a>(&'a self, _visit: impl FnMut(&'a Node)) {}
    fn visit_errors(&self, _visit: impl FnMut(&'static str)) {}
}

impl Node {
    #[inline]
    pub fn parse<T: INode>(parser: &mut Parser) -> Option<Self> {
        let start_pos = parser.pos;
        let node_kind = T::parse(parser);
        
        node_kind.map(|kind| Self {
            kind: kind.into_node(),
            range: start_pos..parser.pos,
        })
    }

    #[inline]
    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        match &self.kind {
            // primitives
            NodeKind::IntegerLiteral(v) => v.visit_children(visit),
            NodeKind::FloatingLiteral(v) => v.visit_children(visit),
            NodeKind::StringLiteral(v) => v.visit_children(visit),
            NodeKind::BooleanLiteral(v) => v.visit_children(visit),
            NodeKind::Identifier(v) => v.visit_children(visit),
            NodeKind::DontCare(v) => v.visit_children(visit),
            NodeKind::Void(v) => v.visit_children(visit),
            // expressions
            NodeKind::Operation(v) => v.visit_children(visit),
            NodeKind::Expression(v) => v.visit_children(visit),
        }
    }
}
