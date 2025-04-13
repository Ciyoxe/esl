use super::{node::Node, operations::OperationSettings};

pub enum ExpressionFlatPart {
    Atom(Node),
    Operation(OperationSettings),
}


pub fn build_expression(parts: Vec<ExpressionFlatPart>) -> Node {
    todo!()
}