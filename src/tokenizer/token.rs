use crate::tokenizer::error::TokenizeError;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub range: std::ops::Range<usize>,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { range: 0..0, kind }
    }
    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Numeric literals
    NumBinInt,   // 0b0101
    NumHexInt,   // 0xABC
    NumDecInt,   // 1234
    NumDecFloat, // 1234.5

    // Operators
    OpAdd,     // +
    OpSub,     // -
    OpMul,     // *
    OpDiv,     // /
    OpMod,     // %
    OpRng,     // ..
    OpGt,      // >
    OpGe,      // >=
    OpLt,      // <
    OpLe,      // <=
    OpNe,      // !=
    OpEq,      // ==
    OpOr,      // |
    OpAnd,     // &
    OpNot,     // !
    OpDot,     // .
    OpComma,   // ,
    OpCatch,   // ?
    OpLam,     // ->
    OpAsg,     // =
    OpAddAsg,  // +=
    OpSubAsg,  // -=
    OpMulAsg,  // *=
    OpDivAsg,  // /=
    OpModAsg,  // %=
    OpAndAsg,  // &=
    OpOrAsg,   // |=
    OpAs,      // as
    OpRef,     // ref
    OpTypedef, // :

    // Keywords
    KwIf,
    KwOr,
    KwMatch,
    KwFor,
    KwIn,
    KwWhile,
    KwLoop,
    KwLet,
    KwVar,
    KwFun,
    KwType,
    KwStruct,
    KwEnum,
    KwTrait,
    KwUse,
    KwModule,
    KwPub,
    KwTrue,
    KwFalse,

    // Other tokens
    Ignore,     // _
    Semicolon,  // ;
    Attribute,  // @attr
    Identifier, // some_ident
    String,     // "string"
    DocComment, // /// comment

    // Structure
    RoundBraces  { children: Vec<Token> },
    SquareBraces { children: Vec<Token> },
    CurlyBraces  { children: Vec<Token> },

    // Errors
    Error(TokenizeError),
}
