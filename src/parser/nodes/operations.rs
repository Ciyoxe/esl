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
    Try,     // a ?
    Lam,     // a -> b
    Ref,     // ref a
    As,      // a as b
    Typedef, // a : b

    Asg,     // a = b
    AddAsg,  // a += b
    SubAsg,  // a -= b
    MulAsg,  // a *= b
    DivAsg,  // a /= b
    ModAsg,  // a %= b
    AndAsg,  // a &= b
    OrAsg,   // a |= b

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
            Some(TokenKind::OpSub)     => Some(Self::parse(parser, OperationKind::Neg, 0, 200)),
            Some(TokenKind::OpNot)     => Some(Self::parse(parser, OperationKind::Not, 0, 200)),
            Some(TokenKind::OpRef)     => Some(Self::parse(parser, OperationKind::Ref, 0, 200)),
            _ => None,
        }
    }

    pub fn parse_infix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::RoundL)    => Some(Self::parse(parser, OperationKind::FuncCall, 1000, 0)),
            Some(TokenKind::CurlyL)    => Some(Self::parse(parser, OperationKind::ValueCtor, 1000, 0)),
            Some(TokenKind::SquareL)   => Some(Self::parse(parser, OperationKind::TypeCtor, 1000, 0)),

            Some(TokenKind::OpDot)     => Some(Self::parse(parser, OperationKind::Dot, 160, 161)),
            Some(TokenKind::OpAs)      => Some(Self::parse(parser, OperationKind::As, 150, 151)),

            Some(TokenKind::OpDiv)     => Some(Self::parse(parser, OperationKind::Div, 80, 81)),
            Some(TokenKind::OpMod)     => Some(Self::parse(parser, OperationKind::Mod, 80, 81)),
            Some(TokenKind::OpMul)     => Some(Self::parse(parser, OperationKind::Mul, 80, 81)),

            Some(TokenKind::OpAdd)     => Some(Self::parse(parser, OperationKind::Add, 70, 71)),
            Some(TokenKind::OpSub)     => Some(Self::parse(parser, OperationKind::Sub, 70, 71)),

            Some(TokenKind::OpGt)      => Some(Self::parse(parser, OperationKind::Gt, 60, 61)),
            Some(TokenKind::OpGe)      => Some(Self::parse(parser, OperationKind::Ge, 60, 61)),
            Some(TokenKind::OpLt)      => Some(Self::parse(parser, OperationKind::Lt, 60, 61)),
            Some(TokenKind::OpLe)      => Some(Self::parse(parser, OperationKind::Le, 60, 61)),
            Some(TokenKind::OpNe)      => Some(Self::parse(parser, OperationKind::Ne, 60, 61)),
            Some(TokenKind::OpEq)      => Some(Self::parse(parser, OperationKind::Eq, 60, 61)),

            Some(TokenKind::OpAnd)     => Some(Self::parse(parser, OperationKind::And, 50, 51)),

            Some(TokenKind::OpOr)      => Some(Self::parse(parser, OperationKind::Or, 40, 41)),

            Some(TokenKind::OpTypedef) => Some(Self::parse(parser, OperationKind::Typedef, 21, 20)),
            Some(TokenKind::OpLam)     => Some(Self::parse(parser, OperationKind::Lam, 21, 20)),
            Some(TokenKind::OpAsg)     => Some(Self::parse(parser, OperationKind::Asg, 21, 20)),
            Some(TokenKind::OpOrAsg)   => Some(Self::parse(parser, OperationKind::OrAsg, 21, 20)),
            Some(TokenKind::OpAndAsg)  => Some(Self::parse(parser, OperationKind::AndAsg, 21, 20)),
            Some(TokenKind::OpAddAsg)  => Some(Self::parse(parser, OperationKind::AddAsg, 21, 20)),
            Some(TokenKind::OpSubAsg)  => Some(Self::parse(parser, OperationKind::SubAsg, 21, 20)),
            Some(TokenKind::OpMulAsg)  => Some(Self::parse(parser, OperationKind::MulAsg, 21, 20)),
            Some(TokenKind::OpDivAsg)  => Some(Self::parse(parser, OperationKind::DivAsg, 21, 20)),
            Some(TokenKind::OpModAsg)  => Some(Self::parse(parser, OperationKind::ModAsg, 21, 20)),

            Some(TokenKind::OpComma)   => Some(Self::parse(parser, OperationKind::Comma, 10, 11)),

            _ => None,
        }
    }

    pub fn parse_postfix_operation(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::OpTry)     => Some(Self::parse(parser, OperationKind::Try, 100, 0)),

            _ => None,
        }
    }
}
