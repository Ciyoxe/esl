pub mod primitives;
pub use primitives::*;

pub mod expressions;
pub use expressions::*;

pub mod operations;
pub use operations::*;

use super::Parser;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum NodeKind {
    // primitives
    IntegerLiteral(IntegerLiteral),
    FloatingLiteral(FloatingLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),

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
    
    fn visit_children(&self, _visit: impl FnMut(&Node)) {}
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
    pub fn visit_children(&self, visit: impl FnMut(&Node)) {
        match &self.kind {
            // primitives
            NodeKind::IntegerLiteral(v) => v.visit_children(visit),
            NodeKind::FloatingLiteral(v) => v.visit_children(visit),
            NodeKind::StringLiteral(v) => v.visit_children(visit),
            NodeKind::BooleanLiteral(v) => v.visit_children(visit),
            
            // expressions
            NodeKind::Operation(v) => v.visit_children(visit),
            NodeKind::Expression(v) => v.visit_children(visit),
        }
    }
}
