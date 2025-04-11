use crate::tokenizer::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationKind {
    Add,        // a + b
    Sub,        // a - b
    Neg,        // - a
    Mul,        // a * b
    Div,        // a / b
    Mod,        // a % b
    Gt,         // a > b
    Ge,         // a >= b
    Lt,         // a < b
    Le,         // a <= b
    Ne,         // a != b
    Eq,         // a == b
    Or,         // a | b
    And,        // a & b
    Not,        // !a
    Dot,        // a.b
    Comma,      // a, b
    FullRng,    // ..
    FromRng,    // a..
    ToRng,      // ..a
    Rng,        // a..b
    Err,        // a?
    Lam,        // a -> b
    Asg,        // a = b
    AddAsg,     // a += b
    SubAsg,     // a -= b
    MulAsg,     // a *= b
    DivAsg,     // a /= b
    ModAsg,     // a %= b
    AndAsg,     // a &= b
    OrAsg,      // a |= b
    As,         // a as b

    // Generated operations (without exact tokens, but corresponding to braces)
    FuncCall,   // Callable( Args... )
    ValueCtor,  // Type{ ValueArgs... }
    TypeCtor,   // Type[ TypeArgs... ]
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
    pub fn for_token(token_kind: TokenKind) -> &'static [Self] {
        match token_kind {
            TokenKind::OpAdd => &[Self {
                precedence: 6,
                arity: OperationArity::Binary,
                kind: OperationKind::Add,
            }],
            TokenKind::OpSub => &[
                Self {
                    precedence: 6,
                    arity: OperationArity::Binary,
                    kind: OperationKind::Sub,
                },
                Self {
                    precedence: 1,
                    arity: OperationArity::UnaryPrefix,
                    kind: OperationKind::Neg,
                },
            ],
            TokenKind::OpMul => &[Self {
                kind: OperationKind::Mul,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpDiv => &[Self {
                kind: OperationKind::Div,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpMod => &[Self {
                kind: OperationKind::Mod,
                arity: OperationArity::Binary,
                precedence: 7,
            }],
            TokenKind::OpGt => &[Self {
                kind: OperationKind::Gt,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpGe => &[Self {
                kind: OperationKind::Ge,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpLt => &[Self {
                kind: OperationKind::Lt,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpLe => &[Self {
                kind: OperationKind::Le,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpNe => &[Self {
                kind: OperationKind::Ne,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpEq => &[Self {
                kind: OperationKind::Eq,
                arity: OperationArity::Binary,
                precedence: 5,
            }],
            TokenKind::OpOr => &[Self {
                kind: OperationKind::Or,
                arity: OperationArity::Binary,
                precedence: 3,
            }],
            TokenKind::OpAnd => &[Self {
                kind: OperationKind::And,
                arity: OperationArity::Binary,
                precedence: 4,
            }],
            TokenKind::OpNot => &[Self {
                kind: OperationKind::Not,
                arity: OperationArity::UnaryPrefix,
                precedence: 9,
            }],
            TokenKind::OpDot => &[Self {
                kind: OperationKind::Dot,
                arity: OperationArity::Binary,
                precedence: 11,
            }],
            TokenKind::OpComma => &[Self {
                kind: OperationKind::Comma,
                arity: OperationArity::Binary,
                precedence: 0,
            }],
            TokenKind::OpRng => &[
                Self {
                    kind: OperationKind::Rng,
                    arity: OperationArity::Binary,
                    precedence: 2,
                },
                Self {
                    kind: OperationKind::FromRng,
                    arity: OperationArity::UnaryPostfix,
                    precedence: 10,
                },
                Self {
                    kind: OperationKind::ToRng,
                    arity: OperationArity::UnaryPrefix,
                    precedence: 9,
                },
                Self {
                    kind: OperationKind::FullRng,
                    arity: OperationArity::Nullary,
                    precedence: 2,
                },
            ],
            TokenKind::OpErr => &[Self {
                kind: OperationKind::Err,
                arity: OperationArity::UnaryPostfix,
                precedence: 10,
            }],
            TokenKind::OpLam => &[Self {
                kind: OperationKind::Lam,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAsg => &[Self {
                kind: OperationKind::Asg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAddAsg => &[Self {
                kind: OperationKind::AddAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpSubAsg => &[Self {
                kind: OperationKind::SubAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpMulAsg => &[Self {
                kind: OperationKind::MulAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpDivAsg => &[Self {
                kind: OperationKind::DivAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpModAsg => &[Self {
                kind: OperationKind::ModAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAndAsg => &[Self {
                kind: OperationKind::AndAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpOrAsg => &[Self {
                kind: OperationKind::OrAsg,
                arity: OperationArity::Binary,
                precedence: 1,
            }],
            TokenKind::OpAs => &[Self {
                kind: OperationKind::As,
                arity: OperationArity::Binary,
                precedence: 8,
            }],
            TokenKind::RoundL => &[Self {
                kind: OperationKind::FuncCall,
                arity: OperationArity::Binary,
                precedence: 11,
            }],
            TokenKind::CurvedL => &[Self {
                kind: OperationKind::ValueCtor,
                arity: OperationArity::Binary,
                precedence: 11,
            }],
            TokenKind::SquareL => &[Self {
                kind: OperationKind::TypeCtor,
                arity: OperationArity::Binary,
                precedence: 11,
            }],

            _ => &[],
        }
    }
}
