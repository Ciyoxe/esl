use crate::tokenizer::token::TokenKind;
use super::{node::Node, operations::OperationSettings};

#[derive(Debug, Clone)]
pub enum ExpressionFlatPart {
    Atom { node: Node },
    Brace { token: TokenKind, position: usize },
    Operations{ variants: &'static [OperationSettings], position: usize },
}

pub struct ExpressionParser {

}

pub fn build_expression(parts: Vec<ExpressionFlatPart>) -> () {
    parts.iter().for_each(|p| println!("{:?}", p));
}
