pub mod build;
pub mod expr_builder;

#[derive(Debug, Clone)]
pub enum AstNode {
    Integer     { value: u64 },
    Float       { value: f64 },
    Boolean     { value: bool },
    Identifier  { value: String },

    // Reverse polish notation
    Expression  { operations: Vec<Operation> },
}

#[derive(Debug, Clone)]
pub enum Operation {
    Value(Box<AstNode>),

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
