use crate::tokenizer::token::TokenKind;
use super::{node::Node, operations::OperationSettings};

pub enum ExpressionFlatPart {
    Atom { node: Node },
    Brace { token: TokenKind, position: usize },
    Operations{ variants: &'static [OperationSettings], position: usize },
}

pub fn build_expression(parts: Vec<ExpressionFlatPart>) -> Node {
    todo!()
}
