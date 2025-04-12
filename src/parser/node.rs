pub struct Node {
    /// Range for Node with Token indices
    pub range: std::ops::Range<usize>,
    pub kind: NodeKind,
}

pub enum NodeKind {
    // Literals
    IntegerLiteral {
        value: u64,
    },
    FloatingLiteral {
        value: f64,
    },
    BooleanLiteral {
        value: bool,
    },
    StringLiteral {
        value: Vec<u8>,
    },

    // Expressions
    Identifier {
        name: Vec<u8>,
    },

    /// Operands can be None for unary operations (or nullary as '(..)' full range)  
    /// Operands -> another Operation, Identifier, Literal, Call or Expressionable construction (if branch, for example)
    Operation {
        left_operand: Option<Box<Node>>,
        right_operand: Option<Box<Node>>,
    },
    /// Callable is part that called (most likely Identifier, but can be expression like `(a + b)(someArg)`)  
    /// Arguments -> any expression parts (like Operations, Literals, etc)
    Call {
        callable: Box<Node>,
        arguments: Vec<Node>,
    },

    // Errors
    ErrNumberOverflow,
    ErrMissingOperand,
}
