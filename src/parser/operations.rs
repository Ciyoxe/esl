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
    Dot,     // a.b
    FullRng, // ..
    FromRng, // a..
    ToRng,   // ..a
    Rng,     // a..b
    Err,     // a?
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationArity {
    Binary,
    UnaryPrefix,
    UnaryPostfix,
    Nullary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperationSettings {
    pub kind: OperationKind,
    pub arity: OperationArity,
    pub precedence: u32,
}

impl OperationSettings {
    pub fn for_token(token_kind: TokenKind) -> &'static [OperationSettings] {
        match token_kind {
            TokenKind::OpAdd => &[OperationSettings {
                precedence: 6,
                arity: OperationArity::Binary,
                kind: OperationKind::Add,
            }],
            TokenKind::OpSub => &[
                OperationSettings {
                    precedence: 6,
                    arity: OperationArity::Binary,
                    kind: OperationKind::Sub,
                },
                OperationSettings {
                    precedence: 1,
                    arity: OperationArity::UnaryPrefix,
                    kind: OperationKind::Neg,
                },
            ],
            TokenKind::OpMul => &[OperationSettings {
                kind: OperationKind::Mul,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpDiv => &[OperationSettings {
                kind: OperationKind::Div,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpMod => &[OperationSettings {
                kind: OperationKind::Mod,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpGt => &[OperationSettings {
                kind: OperationKind::Gt,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpGe => &[OperationSettings {
                kind: OperationKind::Ge,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpLt => &[OperationSettings {
                kind: OperationKind::Lt,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpLe => &[OperationSettings {
                kind: OperationKind::Le,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpNe => &[OperationSettings {
                kind: OperationKind::Ne,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpEq => &[OperationSettings {
                kind: OperationKind::Eq,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpOr => &[OperationSettings {
                kind: OperationKind::Or,
                arity: OperationArity::Binary,
                precedence: 3,
            }],
            TokenKind::OpAnd => &[OperationSettings {
                kind: OperationKind::And,
                arity: OperationArity::Binary,
                precedence: 4,
            }],
            TokenKind::OpNot => &[OperationSettings {
                kind: OperationKind::Not,
                arity: OperationArity::UnaryPrefix,
                precedence: 9,
            }],
            TokenKind::OpDot => &[OperationSettings {
                kind: OperationKind::Dot,
                arity: OperationArity::Binary,
                precedence: 11,
            }],
            TokenKind::OpRng => &[
                OperationSettings {
                    kind: OperationKind::Rng,
                    arity: OperationArity::Binary,
                    precedence: 2,
                },
                OperationSettings {
                    kind: OperationKind::FromRng,
                    arity: OperationArity::UnaryPostfix,
                    precedence: 10,
                },
                OperationSettings {
                    kind: OperationKind::ToRng,
                    arity: OperationArity::UnaryPrefix,
                    precedence: 9,
                },
                OperationSettings {
                    kind: OperationKind::FullRng,
                    arity: OperationArity::Nullary,
                    precedence: 2,
                },
            ],
            TokenKind::OpErr => &[OperationSettings {
                kind: OperationKind::Err,
                arity: OperationArity::UnaryPostfix,
                precedence: 10,
            }],
            TokenKind::OpLam => &[OperationSettings {
                kind: OperationKind::Lam,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAsg => &[OperationSettings {
                kind: OperationKind::Asg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAddAsg => &[OperationSettings {
                kind: OperationKind::AddAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpSubAsg => &[OperationSettings {
                kind: OperationKind::SubAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpMulAsg => &[OperationSettings {
                kind: OperationKind::MulAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpDivAsg => &[OperationSettings {
                kind: OperationKind::DivAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpModAsg => &[OperationSettings {
                kind: OperationKind::ModAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAndAsg => &[OperationSettings {
                kind: OperationKind::AndAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpOrAsg => &[OperationSettings {
                kind: OperationKind::OrAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAs => &[OperationSettings {
                kind: OperationKind::As,
                arity: OperationArity::Binary,
                precedence: 8,
            }],

            _ => unreachable!(),
        }
    }
}
