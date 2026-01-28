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
    Comma,   // a , b
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
    pub fn from_token(token: &TokenKind) -> Option<Self> {
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
            TokenKind::OpComma => Some(Self::Comma),
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
    
}
