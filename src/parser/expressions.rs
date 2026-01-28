use super::*;
use crate::tokenizer::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Operation {
    Add, // a + b
    Sub, // a - b
    Neg, // - a
    Mul, // a * b
    Div, // a / b
    Mod, // a % b

    Gt, // a > b
    Ge, // a >= b
    Lt, // a < b
    Le, // a <= b
    Ne, // a != b
    Eq, // a == b

    Or,  // a | b
    And, // a & b
    Not, // !a

    Dot,     // a . b
    Try,     // a ?
    Lam,     // a -> b
    Ref,     // ref a
    Typedef, // a : b

    Asg,    // a = b
    AddAsg, // a += b
    SubAsg, // a -= b
    MulAsg, // a *= b
    DivAsg, // a /= b
    ModAsg, // a %= b
    AndAsg, // a &= b
    OrAsg,  // a |= b

    FuncCall { args: Vec<Node> },  // Callable( Args... )
    ValueCtor { args: Vec<Node> }, // Type{ ValueArgs... }
    TypeCtor { args: Vec<Node> },  // Type[ TypeArgs... ]
}

impl Operation {
    pub fn visit_children(&self, visit: impl FnMut(&Node)) {
        match &self {
            Operation::FuncCall { args } => args.iter().for_each(visit),
            Operation::TypeCtor { args } => args.iter().for_each(visit),
            Operation::ValueCtor { args } => args.iter().for_each(visit),
            _ => (),
        }
    }

    fn from_token(token: &TokenKind) -> Option<Self> {
        match token {
            TokenKind::OpAdd => Some(Self::Add),
            TokenKind::OpSub => Some(Self::Sub),
            TokenKind::OpMul => Some(Self::Mul),
            TokenKind::OpDiv => Some(Self::Div),
            TokenKind::OpMod => Some(Self::Mod),
            TokenKind::OpGt => Some(Self::Gt),
            TokenKind::OpGe => Some(Self::Ge),
            TokenKind::OpLt => Some(Self::Lt),
            TokenKind::OpLe => Some(Self::Le),
            TokenKind::OpNe => Some(Self::Ne),
            TokenKind::OpEq => Some(Self::Eq),
            TokenKind::OpOr => Some(Self::Or),
            TokenKind::OpAnd => Some(Self::And),
            TokenKind::OpNot => Some(Self::Not),
            TokenKind::OpDot => Some(Self::Dot),
            TokenKind::OpTry => Some(Self::Try),
            TokenKind::OpLam => Some(Self::Lam),
            TokenKind::OpAsg => Some(Self::Asg),
            TokenKind::OpAddAsg => Some(Self::AddAsg),
            TokenKind::OpSubAsg => Some(Self::SubAsg),
            TokenKind::OpMulAsg => Some(Self::MulAsg),
            TokenKind::OpDivAsg => Some(Self::DivAsg),
            TokenKind::OpModAsg => Some(Self::ModAsg),
            TokenKind::OpAndAsg => Some(Self::AndAsg),
            TokenKind::OpOrAsg => Some(Self::OrAsg),
            TokenKind::OpRef => Some(Self::Ref),
            TokenKind::OpTypedef => Some(Self::Typedef),
            TokenKind::RoundBraces { children: _ } => Some(Self::FuncCall { args: Vec::new() }),
            TokenKind::SquareBraces { children: _ } => Some(Self::TypeCtor { args: Vec::new() }),
            TokenKind::CurlyBraces { children: _ } => Some(Self::ValueCtor { args: Vec::new() }),
            _ => None,
        }
    }
    fn get_precedence(&self) -> u32 {
        // TODO: not the final result, just for fun
        match &self {
            Operation::Dot
            | Operation::Try
            | Operation::FuncCall { .. }
            | Operation::TypeCtor { .. }
            | Operation::ValueCtor { .. } => 90,
            Operation::Neg | Operation::Not | Operation::Ref => 80,
            Operation::Mul | Operation::Div | Operation::Mod => 70,
            Operation::Add | Operation::Sub => 60,
            Operation::Gt | Operation::Ge | Operation::Lt | Operation::Le => 50,
            Operation::Eq | Operation::Ne => 45,
            Operation::And => 40,
            Operation::Or => 30,
            Operation::Typedef => 20,
            Operation::Lam => 10,
            Operation::Asg
            | Operation::AddAsg
            | Operation::SubAsg
            | Operation::MulAsg
            | Operation::DivAsg
            | Operation::ModAsg
            | Operation::AndAsg
            | Operation::OrAsg => 0,
        }
    }
}

impl Parser<'_> {
    pub fn p_operation(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let operation = match this.next() {
                Some(next) => match Operation::from_token(&next.kind) {
                    Some(op) => op,
                    None => return None,
                }
                None => return None,
            };
            Some(NodeKind::Operation(operation))
        })
    }
}

pub struct Expression {
    rpn: Vec<Node>,
}

impl Expression {
    pub fn visit_children(&self, visit: impl FnMut(&Node)) {
        self.rpn.iter().for_each(visit);
    }
}