use super::*;
use crate::tokenizer::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Operation {
    Add, // a + b
    Sub, // a - b
    Neg, // - a
    Mul, // a * b
    Div, // a / b
    Mod, // a % b

    Gt, // a > b
    Ge, // a >= b
    Lt, // a < b
    Le, // a <= b
    Ne, // a != b
    Eq, // a == b

    Or,  // a | b
    And, // a & b
    Not, // !a

    Dot,     // a . b
    Try,     // a ?
    Ref,     // ref a
    Typedef, // a : b

    Asg,    // a = b
    AddAsg, // a += b
    SubAsg, // a -= b
    MulAsg, // a *= b
    DivAsg, // a /= b
    ModAsg, // a %= b
    AndAsg, // a &= b
    OrAsg,  // a |= b

    FuncCall { args: Vec<Node> },  // Callable( Args... )
    ValueCtor { args: Vec<Node> }, // Type{ ValueArgs... }
    TypeCtor { args: Vec<Node> },  // Type[ TypeArgs... ]
}

impl Operation {
    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        match &self {
            Operation::FuncCall { args } => args.iter().for_each(visit),
            Operation::TypeCtor { args } => args.iter().for_each(visit),
            Operation::ValueCtor { args } => args.iter().for_each(visit),
            _ => (),
        }
    }

    fn from_token_prefix(token: &TokenKind) -> Option<Self> {
        match token {
            TokenKind::OpNot => Some(Self::Not),
            TokenKind::OpRef => Some(Self::Ref),
            TokenKind::OpSub => Some(Self::Neg),
            _ => None,
        }
    }
    fn from_token_postfix(token: &TokenKind) -> Option<Self> {
        match token {
            TokenKind::OpTry => Some(Self::Try),
            TokenKind::RoundBraces { .. } => Some(Self::FuncCall { args: Vec::new() }),
            TokenKind::CurlyBraces { .. } => Some(Self::ValueCtor { args: Vec::new() }),
            TokenKind::SquareBraces { .. } => Some(Self::TypeCtor { args: Vec::new() }),
            _ => None,
        }
    }
    fn from_token_infix(token: &TokenKind) -> Option<Self> {
        match token {
            TokenKind::OpAdd => Some(Self::Add),
            TokenKind::OpSub => Some(Self::Sub),
            TokenKind::OpMul => Some(Self::Mul),
            TokenKind::OpDiv => Some(Self::Div),
            TokenKind::OpMod => Some(Self::Mod),
            TokenKind::OpGt => Some(Self::Gt),
            TokenKind::OpGe => Some(Self::Ge),
            TokenKind::OpLt => Some(Self::Lt),
            TokenKind::OpLe => Some(Self::Le),
            TokenKind::OpNe => Some(Self::Ne),
            TokenKind::OpEq => Some(Self::Eq),
            TokenKind::OpOr => Some(Self::Or),
            TokenKind::OpAnd => Some(Self::And),
            TokenKind::OpDot => Some(Self::Dot),
            TokenKind::OpAsg => Some(Self::Asg),
            TokenKind::OpAddAsg => Some(Self::AddAsg),
            TokenKind::OpSubAsg => Some(Self::SubAsg),
            TokenKind::OpMulAsg => Some(Self::MulAsg),
            TokenKind::OpDivAsg => Some(Self::DivAsg),
            TokenKind::OpModAsg => Some(Self::ModAsg),
            TokenKind::OpAndAsg => Some(Self::AndAsg),
            TokenKind::OpOrAsg => Some(Self::OrAsg),
            TokenKind::OpTypedef => Some(Self::Typedef),
            _ => None,
        }
    }
    fn get_precedence(&self) -> u32 {
        // TODO: not the final result, just for fun
        match &self {
            Operation::Dot
            | Operation::Try
            | Operation::FuncCall { .. }
            | Operation::TypeCtor { .. }
            | Operation::ValueCtor { .. } => 90,
            Operation::Neg | Operation::Not | Operation::Ref => 80,
            Operation::Mul | Operation::Div | Operation::Mod => 70,
            Operation::Add | Operation::Sub => 60,
            Operation::Gt | Operation::Ge | Operation::Lt | Operation::Le => 50,
            Operation::Eq | Operation::Ne => 40,
            Operation::And => 30,
            Operation::Or => 20,
            Operation::Typedef => 10,
            Operation::Asg
            | Operation::AddAsg
            | Operation::SubAsg
            | Operation::MulAsg
            | Operation::DivAsg
            | Operation::ModAsg
            | Operation::AndAsg
            | Operation::OrAsg => 0,
        }
    }
}

impl Parser<'_> {
    fn p_operation_prefix(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let operation = match this.next() {
                Some(next) => match Operation::from_token_prefix(&next.kind) {
                    Some(op) => op,
                    None => return None,
                },
                None => return None,
            };
            this.advance();
            Some(NodeKind::Operation(operation))
        })
    }
    fn p_operation_postfix(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let mut operation = match this.next() {
                Some(next) => match Operation::from_token_postfix(&next.kind) {
                    Some(op) => op,
                    None => return None,
                },
                None => return None,
            };
            // Handle call-like operations
            match &mut operation {
                Operation::FuncCall { args }
                | Operation::TypeCtor { args }
                | Operation::ValueCtor { args } => {
                    let next_nodes = match &this.next_unwrap().kind {
                        TokenKind::CurlyBraces { children }
                        | TokenKind::RoundBraces { children }
                        | TokenKind::SquareBraces { children } => children,

                        _ => {
                            unreachable!("Call-like operation can be parsed only for braces tokens")
                        }
                    };
                    args.extend(this.p_args_list(next_nodes));
                }

                _ => (),
            }
            this.advance();
            Some(NodeKind::Operation(operation))
        })
    }
    fn p_operation_infix(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let operation = match this.next() {
                Some(next) => match Operation::from_token_infix(&next.kind) {
                    Some(op) => op,
                    None => return None,
                },
                None => return None,
            };
            this.advance();
            Some(NodeKind::Operation(operation))
        })
    }
    // expr, expr, ..., expr, with optional trailing comma
    fn p_args_list(&self, tokens: &[Token]) -> Vec<Node> {
        let mut inner_parser = Parser::new(self.src, tokens);
        let mut nodes = Vec::<Node>::new();

        loop {
            match inner_parser.p_expression() {
                Some(expr) => nodes.push(expr),
                _ => break,
            }
            match inner_parser.next().map(|t| &t.kind) {
                Some(TokenKind::OpComma) => (),
                _ => break,
            }
        }

        if inner_parser.pos < tokens.len() {
            nodes.push(self.make_error_for_tokens(
                ParsingError::UnexpectedCallArgument,
                &tokens[inner_parser.pos..],
            ));
        }

        nodes
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    rpn: Vec<Node>,
}

impl Expression {
    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        self.rpn.iter().for_each(visit);
    }
}

impl Parser<'_> {
    fn p_operand(&mut self) -> Option<Node> {
        self.p_identifier()
            .or_else(|| self.p_boolean_literal())
            .or_else(|| self.p_dont_care())
            .or_else(|| self.p_floating_literal())
            .or_else(|| self.p_integer_literal())
            .or_else(|| self.p_string_literal())
            .or_else(|| self.p_nested_expr())
    }
    // (expr)
    fn p_nested_expr(&mut self) -> Option<Node> {
        match self.next().map(|t| &t.kind) {
            Some(TokenKind::RoundBraces { children }) => {
                Parser::new(self.src, children).p_expression()
            }
            _ => None,
        }
    }
    // prefix ops - operand - postfix ops
    fn p_atom(&mut self) -> Vec<Node> {
        let mut nodes = Vec::<Node>::new();
        let mut was_prefix = false;
        let mut was_operand = false;
        let mut was_postfix = false;

        while let Some(prefix) = self.p_operation_prefix() {
            nodes.push(prefix);
            was_prefix = true;
        }

        if let Some(operand) = self.p_operand() {
            nodes.push(operand);
            was_operand = true;
        } else if was_prefix {
            nodes.push(self.make_error_here(ParsingError::NoOperandAfterPrefixOperator));
            was_operand = true;
        }

        while let Some(postfix) = self.p_operation_postfix() {
            nodes.push(postfix);
            was_postfix = true;
        }

        if was_postfix && !was_operand {
            nodes.push(self.make_error_here(ParsingError::PostfixOperatorWithoutOperand));
        }

        nodes
    }
    // atom - infix op - atom - infix op - ...
    fn p_flat_expr(&mut self) -> Vec<Node> {
        let mut nodes = Vec::<Node>::new();

        let mut required_atom = false;
        let mut was_atom: bool;

        loop {
            let atom_nodes = self.p_atom();
            if atom_nodes.is_empty() {
                if required_atom {
                    nodes.push(self.make_error_here(ParsingError::NoOperandAfterInfixOperation));
                    was_atom = true;
                } else {
                    break;
                }
            } else {
                nodes.extend(atom_nodes);
                was_atom = true;
            }

            match self.p_operation_infix() {
                Some(op) => {
                    if was_atom {
                        nodes.push(op);
                    } else {
                        nodes.push(
                            self.make_error_here(ParsingError::InfixOperationWithoutLeftOperand),
                        );
                    }
                    required_atom = true;
                }
                None => {
                    required_atom = false;
                }
            }
        }

        nodes
    }
    pub fn p_expression(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let mut flat_form = this.p_flat_expr();

            if flat_form.is_empty() {
                return None;
            }

            // TODO: make rpn building
            Some(NodeKind::Expression(Expression { rpn: flat_form }))
        })
    }
}
