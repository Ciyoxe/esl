#[derive(Debug, Clone)]
pub struct AstNode {
    pub value: AstNodeContent,
    pub range: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub enum AstNodeContent {
    Integer     { value: u64 },
    Float       { value: f64 },
    Boolean     { value: bool },
    Identifier  { value: String },

    // Reverse polish notation
    Expression  { operations: Vec<Operation> },
}

#[derive(Debug, Clone)]
pub enum Operation {
    Value(AstNode),

    Neg,
    Not,
    Cat,
    Add,
    Sub,
    Mul,
    Div,

    FnCall   (Vec<AstNode>),
    ValCtor  (Vec<AstNode>),
    TypeCtor (Vec<AstNode>),
}
