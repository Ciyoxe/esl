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
            _ => None
        }
    }
    fn from_token_postfix(token: &TokenKind) -> Option<Self> {
        match token {
            TokenKind::OpTry => Some(Self::Try),
            _ => None
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
    fn is_right_associative(&self) -> bool {
        matches!(self, Operation::Neg | Operation::Not | Operation::Ref)
    }
}

impl Parser<'_> {
    pub fn p_operation_prefix(&mut self) -> Option<Node> {
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
    pub fn p_operation_postfix(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let operation = match this.next() {
                Some(next) => match Operation::from_token_postfix(&next.kind) {
                    Some(op) => op,
                    None => return None,
                },
                None => return None,
            };
            this.advance();
            Some(NodeKind::Operation(operation))
        })
    }
    pub fn p_operation_infix(&mut self) -> Option<Node> {
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
    pub fn p_expressions_list(&mut self) -> Vec<Node> {
        let mut expressions = Vec::new();

        loop {
            let Some(expression) = self.p_expression() else {
                break;
            };
            expressions.push(expression);

            if !self.advance_on(TokenKind::OpComma) {
                break;
            }
            if self.next().is_none() {
                break;
            }
        }

        expressions
    }

    pub fn p_operand(&mut self) -> Option<Node> {
        self.p_identifier()
            .or_else(|| self.p_boolean_literal())
            .or_else(|| self.p_dont_care())
            .or_else(|| self.p_floating_literal())
            .or_else(|| self.p_integer_literal())
            .or_else(|| self.p_string_literal())
    }
    fn p_grouped_expression(&mut self) -> Option<Node> {
        let expression = {
            let children = match self.next().map(|token| &token.kind) {
                Some(TokenKind::CurlyBraces { children })
                | Some(TokenKind::RoundBraces { children })
                | Some(TokenKind::SquareBraces { children }) => children,
                _ => return None,
            };
            let mut parser = Parser::new(self.src, children);
            parser.p_expression()?
        };

        self.advance();
        Some(expression)
    }
    fn p_call_operation(&mut self) -> Option<Node> {
        enum CallKind {
            Func,
            Type,
            Value,
        }

        self.make_node(|this| {
            let (call_kind, args) = {
                let (children, call_kind) = match this.next().map(|token| &token.kind) {
                    Some(TokenKind::RoundBraces { children }) => (children, CallKind::Func),
                    Some(TokenKind::SquareBraces { children }) => (children, CallKind::Type),
                    Some(TokenKind::CurlyBraces { children }) => (children, CallKind::Value),
                    _ => return None,
                };
                let mut parser = Parser::new(this.src, children);
                let args = parser.p_expressions_list();
                (call_kind, args)
            };

            this.advance();
            let operation = match call_kind {
                CallKind::Func => Operation::FuncCall { args },
                CallKind::Type => Operation::TypeCtor { args },
                CallKind::Value => Operation::ValueCtor { args },
            };

            Some(NodeKind::Operation(operation))
        })
    }
    pub fn p_flat_expr_slice(&mut self) -> Vec<Node> {
        let mut expr_parts = Vec::<Node>::new();
        let mut expect_operand = true;

        loop {
            if expect_operand {
                if let Some(operand) = self.p_operand() {
                    expr_parts.push(operand);
                    expect_operand = false;
                    continue;
                }
                if let Some(operation) = self.p_operation_prefix() {
                    expr_parts.push(operation);
                    continue;
                }
                if let Some(expression) = self.p_grouped_expression() {
                    expr_parts.push(expression);
                    expect_operand = false;
                    continue;
                }
                break;
            }

            if let Some(operation) = self.p_call_operation() {
                expr_parts.push(operation);
                continue;
            }
            if let Some(operation) = self.p_operation_postfix() {
                expr_parts.push(operation);
                continue;
            }
            if let Some(operation) = self.p_operation_infix() {
                expr_parts.push(operation);
                expect_operand = true;
                continue;
            }
            break;
        }

        expr_parts
    }
    pub fn p_expression(&mut self) -> Option<Node> {
        let start = self.pos;
        self.make_node(|this| {
            let flat = this.p_flat_expr_slice();
            let has_operand = flat
                .iter()
                .any(|node| !matches!(&node.kind, NodeKind::Operation(_)));
            if !has_operand {
                this.pos = start;
                return None;
            }

            let rpn = to_rpn(flat);
            Some(NodeKind::Expression(Expression { rpn }))
        })
    }
}

fn to_rpn(flat: Vec<Node>) -> Vec<Node> {
    let mut output = Vec::<Node>::new();
    let mut ops = Vec::<Node>::new();

    for node in flat {
        match &node.kind {
            NodeKind::Operation(operation) => {
                let precedence = operation.get_precedence();
                let right_assoc = operation.is_right_associative();

                while let Some(last) = ops.last() {
                    let NodeKind::Operation(last_op) = &last.kind else {
                        unreachable!();
                    };
                    let last_precedence = last_op.get_precedence();
                    let should_pop = if right_assoc {
                        last_precedence > precedence
                    } else {
                        last_precedence >= precedence
                    };

                    if should_pop {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(node);
            }
            _ => output.push(node),
        }
    }

    output.extend(ops.into_iter().rev());
    output
}
