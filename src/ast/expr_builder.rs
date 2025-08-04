use crate::{
    Rule,
    ast::{
        AstNode, Operation,
        build::{build_atom, build_expression},
    },
};
use pest::iterators::{Pair, Pairs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct OpInfo {
    pub operation: Operation,
    pub precedence: u32,
    pub assoc: Assoc,
}

impl OpInfo {
    pub const fn new(operation: Operation, precedence: u32, assoc: Assoc) -> Self {
        Self {
            operation,
            precedence,
            assoc,
        }
    }

    pub fn of(pair: &Pair<Rule>) -> Option<Self> {
        fn parse_args_list(pair: &Pair<Rule>) -> Vec<AstNode> {
            pair.clone().into_inner().map(build_expression).collect()
        }

        match pair.as_rule() {
            Rule::op_add => Some(Self::new(Operation::Add, 1, Assoc::Right)),
            Rule::op_sub => Some(Self::new(Operation::Sub, 1, Assoc::Left)),
            Rule::op_mul => Some(Self::new(Operation::Mul, 2, Assoc::Left)),
            Rule::op_div => Some(Self::new(Operation::Div, 2, Assoc::Left)),
            Rule::op_neg => Some(Self::new(Operation::Neg, 3, Assoc::Right)),
            Rule::op_not => Some(Self::new(Operation::Not, 3, Assoc::Right)),
            Rule::op_cat => Some(Self::new(Operation::Cat, 4, Assoc::Left)),

            Rule::func_call  => Some(Self::new(Operation::FnCall(parse_args_list(pair)), 5, Assoc::Left)),
            Rule::value_ctor => Some(Self::new(Operation::ValCtor(parse_args_list(pair)), 5, Assoc::Left)),
            Rule::type_ctor  => Some(Self::new(Operation::TypeCtor(parse_args_list(pair)), 5, Assoc::Left)),

            _ => None,
        }
    }
}

pub fn build_rpn(expr: Pairs<Rule>) -> Vec<Operation> {
    // Shunting-yard algorithm to transform the flat list of Pest pairs

    // level of braces nesting like 0 (100 (200) 100) 0
    let mut level = 0;

    let mut stack: Vec<OpInfo> = Vec::new();
    let mut output: Vec<Operation> = Vec::new();

    for pair in expr {
        if let Some(op_info) = OpInfo::of(&pair) {
            // calculate precedence with level of braces nesting
            let op_info = OpInfo::new(
                op_info.operation,
                op_info.precedence + level,
                op_info.assoc,
            );

            while let Some(top) = stack.last() {
                let higher_prec = top.precedence > op_info.precedence;
                let same_prec_left_assoc =
                    top.precedence == op_info.precedence && op_info.assoc == Assoc::Left;

                if higher_prec || same_prec_left_assoc {
                    output.push(stack.pop().unwrap().operation);
                } else {
                    break;
                }
            }

            stack.push(op_info);
            continue;
        }

        match pair.as_rule() {
            Rule::opening_brace => level += 100,
            Rule::closing_brace => level -= 100,
            Rule::EOI => break,

            _ => output.push(Operation::Value(build_atom(pair))),
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op.operation);
    }

    output
}
