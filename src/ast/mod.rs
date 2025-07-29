pub mod build;
pub mod expr_builder;

#[derive(Debug, Clone)]
pub enum AstNode {
    Integer     { value: u64 },
    Float       { value: f64 },
    Boolean     { value: bool },
    Identifier  { value: String },

    Expression  { operations: Vec<Operation> },
}

#[derive(Debug, Clone)]
pub enum Operation {
    Use (Box<AstNode>),

    Neg,
    Not,
    Cat,

    Add (Box<AstNode>),
    Sub (Box<AstNode>),
    Mul (Box<AstNode>),
    Div (Box<AstNode>),

    FnCall   (Vec<AstNode>),
    ValCtor  (Vec<AstNode>),
    TypeCtor (Vec<AstNode>),
}
