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
pub enum OperationToken {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Mod,     // %
    Gt,      // >
    Ge,      // >=
    Lt,      // <
    Le,      // <=
    Ne,      // !=
    Eq,      // ==
    Or,      // |
    And,     // &
    Not,     // !
    Dot,     // .
    Comma,   // ,
    Rng,     // ..
    Catch,   // ?
    Lam,     // ->
    Asg,     // =
    AddAsg,  // +=
    SubAsg,  // -=
    MulAsg,  // *=
    DivAsg,  // /=
    ModAsg,  // %=
    AndAsg,  // &=
    OrAsg,   // |=
    As,      // as
    Ref,     // ref
    Typedef, // :
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordToken {
    If,
    Or,
    Match,
    For,
    In,
    While,
    Loop,
    Let,
    Var,
    Fun,
    Type,
    Struct,
    Enum,
    Trait,
    Use,
    Module,
    Pub,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorToken {
    UnexpectedChar,         // useless chars
    InvalidByteSequence,    // not a utf8 string

    InvalidNumber,          // overflowed numbers

    MissingClosingRound,    // missing ) `(a + b`
    MissingClosingCurly,    // missing }
    MissingClosingSquare,   // missing ]
    RedundantClosingRound,  // redundant ) `a + b)`
    RedundantClosingCurly,  // redundant }
    RedundantClosingSquare, // redundant ]
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeToken {
    RoundBraces  (Vec<Token>),
    CurlyBraces  (Vec<Token>),
    SquareBraces (Vec<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Integer    (u64),
    Floating   (f64),
    Boolean    (bool),
    DocComment (String),
    Identifier (String),
    String     (String),
    Operation  (OperationToken),
    Keyword    (KeywordToken),
    Scope      (ScopeToken),
    Error      (ErrorToken),
    Semicolon,
    Ignore,
}
