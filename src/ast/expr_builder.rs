use crate::{ast::{build::build_expression, Operation, AstNode}, Rule};
use pest::iterators::{Pair, Pairs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct OpInfo {
    pub precedence: u8,
    pub assoc: Assoc,
}

impl OpInfo {
    pub const fn new(precedence: u8, assoc: Assoc) -> Self {
        Self { precedence, assoc }
    }

    pub fn of(rule: &Rule) -> Option<Self> {
        match rule {
            Rule::op_add => Some(Self::new(0, Assoc::Left)),
            Rule::op_sub => Some(Self::new(0, Assoc::Left)),
            Rule::op_mul => Some(Self::new(0, Assoc::Left)),
            Rule::op_div => Some(Self::new(0, Assoc::Left)),
            Rule::op_neg => Some(Self::new(0, Assoc::Left)),
            Rule::op_not => Some(Self::new(0, Assoc::Left)),
            Rule::op_cat => Some(Self::new(0, Assoc::Left)),

            _ => None,
        }
    }
}
