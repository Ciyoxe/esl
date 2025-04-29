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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Numeric literals
    NumBinInt,   // 0b0101
    NumHexInt,   // 0xABC
    NumDecInt,   // 1234
    NumDecFloat, // 1234.5

    // Operators
    OpAdd,    // +
    OpSub,    // -
    OpMul,    // *
    OpDiv,    // /
    OpMod,    // %
    OpGt,     // >
    OpGe,     // >=
    OpLt,     // <
    OpLe,     // <=
    OpNe,     // !=
    OpEq,     // ==
    OpOr,     // |
    OpAnd,    // &
    OpNot,    // !
    OpDot,    // .
    OpComma,  // ,
    OpRng,    // ..
    OpErr,    // ?
    OpLam,    // ->
    OpAsg,    // =
    OpAddAsg, // +=
    OpSubAsg, // -=
    OpMulAsg, // *=
    OpDivAsg, // /=
    OpModAsg, // %=
    OpAndAsg, // &=
    OpOrAsg,  // |=
    OpAs,     // as

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
    Colon,     // :
    Semicolon, // ;
    RoundL,    // (
    RoundR,    // )
    SquareL,   // [
    SquareR,   // ]
    CurlyL,   // {
    CurlyR,   // }

    // Other tokens
    Attribute,  // @attr
    Identifier, // some_ident
    String,     // "string"
    DocComment, // /// comment

    // Errors
    ErrUnexpectedChar,     // unexpected character
    ErrUnterminatedString, // unclosed string
    ErrAttributeName,      // wrong attribute format
}
