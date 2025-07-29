use crate::{Rule, ast::*};
use pest::iterators::Pair;

pub fn build_integer(pair: Pair<Rule>) -> AstNode {
    AstNode::Integer {
        value: pair.as_str().parse::<u64>().unwrap(),
    }
}

pub fn build_float(pair: Pair<Rule>) -> AstNode {
    AstNode::Float {
        value: pair.as_str().parse::<f64>().unwrap(),
    }
}

pub fn build_bool(pair: Pair<Rule>) -> AstNode {
    AstNode::Boolean {
        value: pair.as_str().as_bytes()[0] == b't',
    }
}

pub fn build_identifier(pair: Pair<Rule>) -> AstNode {
    AstNode::Identifier {
        value: pair.as_str().to_string(),
    }
}

pub fn build_expression(pair: Pair<Rule>) -> AstNode {
    todo!()
}
