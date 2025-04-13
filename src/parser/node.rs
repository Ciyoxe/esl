pub struct Node {
    /// Range for Node with Token indices
    pub range: std::ops::Range<usize>,
    pub kind: NodeKind,
}

pub enum NodeKind {
    /*************************************************
     *                  PRIMITIVES                   *
     *************************************************/
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
    Identifier {
        name: Vec<u8>,
    },

    /*************************************************
     *                  EXPRESSIONS                  *
     *************************************************/
     Expression {
         /// Errors related to braces
         errors: Vec<Node>,
         root: Box<Node>,
    },
    ExpressionList {
        /// Comma separated expressions, can be empty in calls like `f()`
        expressions: Vec<Node>,
    },
    Operation {
        left_operand: Option<Box<Node>>,
        /// ExpressionList for calls or ctors
        right_operand: Option<Box<Node>>,
    },

    /*************************************************
     *                    ERRORS                     *
     *************************************************/
    /// If numeric literal is bigger than u64::MAX
    ErrNumberOverflow,
    /// If brace is mismatched like `(a + b}`
    ErrMismatchedBrace,
    /// If brace is unclosed like `(a + b` or `{a + b`
    ErrUnclosedBrace,
    /// If there is missing operand like `a +` or `(a +`
    ErrMissingOperand,
}
