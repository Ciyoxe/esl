#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub range: std::ops::Range<usize>,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self {
            range: 0..0, 
            kind
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Numeric literals
    NumBinInt,      // 0b0101
    NumHexInt,      // 0xABC
    NumDecInt,      // 1234
    NumDecFloat,    // 1234.5

    // Operators
    OpAdd,          // +
    OpSub,          // -
    OpMul,          // *
    OpDiv,          // /
    OpMod,          // %
    OpGt,           // >
    OpGe,           // >=
    OpLt,           // <
    OpLe,           // <=
    OpNe,           // !=
    OpEq,           // ==
    OpOr,           // |
    OpAnd,          // &
    OpNot,          // !
    OpDot,          // .
    OpRng,          // ..
    OpErr,          // ?
    OpLam,          // ->
    OpAsg,          // =
    OpAddAsg,       // +=
    OpSubAsg,       // -=
    OpMulAsg,       // *=
    OpDivAsg,       // /=
    OpModAsg,       // %=
    OpAndAsg,       // &=
    OpOrAsg,        // |=
    OpAs,           // as

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
    KwVoid,
    KwTrue,
    KwFalse,

    // Delimiters
    Comma,          // ,
    Colon,          // :
    Semicolon,      // ;
    RoundL,         // (
    RoundR,         // )
    SquareL,        // [
    SquareR,        // ]
    CurvedL,        // {
    CurvedR,        // }

    // Other tokens
    Attribute,      // @attr
    Identifier,     // some_ident
    String,         // "string"
    DocComment,     // /// comment

    // Errors
    ErrChar,        // unexpected character
    ErrStr,         // unclosed string
    ErrNum,         // wrong number format
    ErrAttr,        // wrong attribute format
}