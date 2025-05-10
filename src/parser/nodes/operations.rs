use super::*;
use crate::tokenizer::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationKind {
    Add,     // a + b
    Sub,     // a - b
    Neg,     // - a
    Mul,     // a * b
    Div,     // a / b
    Mod,     // a % b
    Gt,      // a > b
    Ge,      // a >= b
    Lt,      // a < b
    Le,      // a <= b
    Ne,      // a != b
    Eq,      // a == b
    Or,      // a | b
    And,     // a & b
    Not,     // !a
    Dot,     // a . b
    Comma,   // a , b
    FromRng, // a ..
    ToRng,   // .. a
    Rng,     // a .. b
    Catch,   // a ?
    Lam,     // a -> b
    Asg,     // a = b
    AddAsg,  // a += b
    SubAsg,  // a -= b
    MulAsg,  // a *= b
    DivAsg,  // a /= b
    ModAsg,  // a %= b
    AndAsg,  // a &= b
    OrAsg,   // a |= b
    As,      // a as b
    TypeDef, // a : b

    FuncCall,  // Callable( Args... )
    ValueCtor, // Type{ ValueArgs... }
    TypeCtor,  // Type[ TypeArgs... ]
}

#[derive(Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub left_operand: Option<Box<Node>>,
    pub right_operand: Option<Box<Node>>,
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Operation").field("kind", &self.kind).finish()
    }
}

impl INode for Operation {
    fn parse(_parser: &mut crate::parser::Parser) -> Option<Self> {
        // they shoud be handled during expression parsing
        // direct parsing is not possible (we need to know arity of the operation before)
        unreachable!()
    }

    fn into_node(self) -> super::NodeKind {
        NodeKind::Operation(self)
    }

    fn visit_children<'a>(&'a self, mut iter: impl FnMut(&'a Node)) {
        if let Some(left) = &self.left_operand {
            iter(left.as_ref());
        }
        if let Some(right) = &self.right_operand {
            iter(right.as_ref());
        }
    }
}


#[derive(Debug, Clone)]
pub struct OperationDefinition {
    pub kind: OperationKind,
    pub range: Range<usize>,
    pub left_binding_power: u16,
    pub right_binding_power: u16,
}

impl OperationDefinition {
    pub fn parse(parser: &mut Parser, kind: OperationKind, left_binding_power: u16, right_binding_power: u16) -> Self {
        parser.advance();

        Self {
            kind,
            range: parser.pos - 1..parser.pos,
            left_binding_power,
            right_binding_power,
        }
    }

    pub fn into_operation_node(self, left_operand: Option<Box<Node>>, right_operand: Option<Box<Node>>) -> Node {
        Node {
            range: self.range,
            kind: Operation {
                kind: self.kind,
                right_operand,
                left_operand,
            }.into_node()
        }
    }

    pub fn parse_prefix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpSub) => Some(Self::parse(parser, OperationKind::Neg, 0, 40)),
            Some(TokenKind::OpNot) => Some(Self::parse(parser, OperationKind::Not, 0, 40)),
            Some(TokenKind::OpRng) => Some(Self::parse(parser, OperationKind::ToRng, 0, 40)),

            _ => None,
        }
    }

    pub fn parse_infix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpAdd) => Some(Self::parse(parser, OperationKind::Add, 10, 11)),
            Some(TokenKind::OpSub) => Some(Self::parse(parser, OperationKind::Sub, 10, 11)),
            Some(TokenKind::OpMul) => Some(Self::parse(parser, OperationKind::Mul, 20, 21)),
            Some(TokenKind::OpDiv) => Some(Self::parse(parser, OperationKind::Div, 20, 21)),
            Some(TokenKind::OpRng) => Some(Self::parse(parser, OperationKind::Rng, 100, 101)),
            Some(TokenKind::RoundL) => Some(Self::parse(parser, OperationKind::FuncCall, 50, 0)),

            _ => None,
        }
    }

    pub fn parse_postfix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpRng) => Some(Self::parse(parser, OperationKind::FromRng, 30, 0)),
            Some(TokenKind::OpCatch) => Some(Self::parse(parser, OperationKind::Catch, 30, 0)),

            _ => None,
        }
    }
}
