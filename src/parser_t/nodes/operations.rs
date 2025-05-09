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

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub left_operand: Option<Box<Node>>,
    pub right_operand: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct OperationDefinition {
    pub kind: OperationKind,
    pub range: Range<usize>,
    pub left_binding_power: u16,
    pub right_binding_power: u16,
}

impl INode for Operation {
    fn parse(_parser: &mut crate::parser_t::Parser) -> Option<Self> {
        // they shoud be handled during expression parsing
        // direct parsing is not possible (we need to know arity of the operation before)
        unreachable!()
    }

    fn into_node(self) -> super::NodeKind {
        NodeKind::Operation(self)
    }

    fn visit_children(&self, mut iter: impl FnMut(&Node)) {
        if let Some(left) = &self.left_operand {
            iter(left.as_ref());
        }
        if let Some(right) = &self.right_operand {
            iter(right.as_ref());
        }
    }
}

impl OperationDefinition {
    fn into_operation_node(self, left_operand: Option<Box<Node>>, right_operand: Option<Box<Node>>) -> Node {
        Node {
            range: self.range,
            kind: Operation {
                kind: self.kind,
                right_operand,
                left_operand,
            }.into_node()
        }
    }

    fn parse_prefix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpSub) => Some(Self {
                kind: OperationKind::Neg,
                left_binding_power: 0,
                right_binding_power: 40,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpNot) => Some(Self {
                kind: OperationKind::Not,
                left_binding_power: 0,
                right_binding_power: 40,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpRng) => Some(Self {
                kind: OperationKind::ToRng,
                left_binding_power: 0,
                right_binding_power: 40,
                range: parser.advance().range.clone(),
            }),

            _ => None,
        }
    }

    fn parse_infix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpAdd) => Some(Self {
                kind: OperationKind::Add,
                left_binding_power: 10,
                right_binding_power: 11,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpSub) => Some(Self {
                kind: OperationKind::Sub,
                left_binding_power: 10,
                right_binding_power: 11,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpMul) => Some(Self {
                kind: OperationKind::Mul,
                left_binding_power: 20,
                right_binding_power: 21,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpDiv) => Some(Self {
                kind: OperationKind::Div,
                left_binding_power: 20,
                right_binding_power: 21,
                range: parser.advance().range.clone(),
            }),

            _ => None,
        }
    }

    fn parse_postfix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpRng) => Some(Self {
                kind: OperationKind::FromRng,
                left_binding_power: 30,
                right_binding_power: 0,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::OpCatch) => Some(Self {
                kind: OperationKind::Catch,
                left_binding_power: 30,
                right_binding_power: 0,
                range: parser.advance().range.clone(),
            }),
            Some(TokenKind::RoundL) => Some(Self {
                kind: OperationKind::FuncCall,
                left_binding_power: 50,
                right_binding_power: 0,
                range: parser.advance().range.clone(),
            }),

            _ => None,
        }
    }
}
