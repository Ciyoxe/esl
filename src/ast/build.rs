use crate::{Rule, ast::*};
use pest::iterators::Pair;

pub fn build_integer(pair: Pair<Rule>) -> AstNode {
    assert!(pair.as_rule() == Rule::integer);

    AstNode::Integer {
        value: pair.as_str().parse::<u64>().unwrap(),
    }
}

pub fn build_float(pair: Pair<Rule>) -> AstNode {
    assert!(pair.as_rule() == Rule::float);

    AstNode::Float {
        value: pair.as_str().parse::<f64>().unwrap(),
    }
}

pub fn build_bool(pair: Pair<Rule>) -> AstNode {
    assert!(pair.as_rule() == Rule::boolean);

    AstNode::Boolean {
        value: pair.as_str().as_bytes()[0] == b't',
    }
}

pub fn build_identifier(pair: Pair<Rule>) -> AstNode {
    assert!(pair.as_rule() == Rule::identifier);

    AstNode::Identifier {
        value: pair.as_str().to_string(),
    }
}

pub fn build_atom(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => build_integer(pair),
        Rule::float => build_float(pair),
        Rule::boolean => build_bool(pair),
        Rule::identifier => build_identifier(pair),

        _ => panic!("Unexpected atom rule: {:?}", pair.as_rule()),
    }
}

pub fn build_expression(pair: Pair<Rule>) -> AstNode {
    assert!(pair.as_rule() == Rule::expression);

    AstNode::Expression {
        operations: crate::ast::expr_builder::build_rpn(pair.into_inner()),
    }
}
