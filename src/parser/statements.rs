use crate::{
    parser::{
        Parser,
        errors::ParsingError,
        expressions::Operation,
        nodes::{Node, NodeKind},
    },
    tokenizer::token::TokenKind,
};

/// target = value, target += value, etc.
#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: Box<Node>,
    pub value: Box<Node>,
    /// None for `=`, Some(Add) for `+=`, etc.
    pub operation: Option<Operation>,
}

impl Assignment {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        visit(&self.target);
        visit(&self.value);
    }
}

impl Parser<'_> {
    fn p_assignment_or_expr(&mut self) -> Option<Node> {
        let start = self.pos;
        let expr = self.p_expression()?;

        let compound = match self.next().map(|t| &t.kind) {
            Some(TokenKind::OpAsg) => Some(None),
            Some(TokenKind::OpAddAsg) => Some(Some(Operation::Add)),
            Some(TokenKind::OpSubAsg) => Some(Some(Operation::Sub)),
            Some(TokenKind::OpMulAsg) => Some(Some(Operation::Mul)),
            Some(TokenKind::OpDivAsg) => Some(Some(Operation::Div)),
            Some(TokenKind::OpModAsg) => Some(Some(Operation::Mod)),
            Some(TokenKind::OpAndAsg) => Some(Some(Operation::And)),
            Some(TokenKind::OpOrAsg) => Some(Some(Operation::Or)),
            _ => None,
        };

        let Some(compound) = compound else {
            return Some(expr);
        };
        self.advance();

        let value = self.p_expression().unwrap_or_else(|| {
            self.make_error_here(ParsingError::ExpectedDifferentToken {
                expected: "value",
            })
        });

        let range = self.tks.get(start).map_or(0, |t| t.range.start)..value.range.end;

        Some(Node {
            kind: NodeKind::Assignment(Assignment {
                target: Box::new(expr),
                value: Box::new(value),
                operation: compound,
            }),
            range,
        })
    }
}

/// { stmt; stmt; expr }
#[derive(Debug, Clone)]
pub struct Block {
    /// list of exprs or statements
    pub items: Vec<Node>,
    /// if block returns value (without ending semicolon)
    pub returns_last: bool,
}

impl Block {
    pub fn visit_children<'a>(&'a self, visit: impl FnMut(&'a Node)) {
        self.items.iter().for_each(visit);
    }
}

impl Parser<'_> {
    fn p_statement(&mut self) -> Option<Node> {
        self.p_value_declaration()
            .or_else(|| self.p_assignment_or_expr())
    }
    pub fn p_block(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let TokenKind::CurlyBraces { children } = &this.next()?.kind else {
                return None;
            };

            let mut inner = Parser::new(this.src, children);
            let mut items = Vec::new();
            let mut returns_last = false;

            while let Some(stmt) = inner.p_statement() {
                items.push(stmt);
                if !inner.advance_on(TokenKind::Semicolon) {
                    returns_last = true;
                    break;
                }
            }

            if inner.pos < children.len() {
                items.push(this.make_error_for_tokens(
                    ParsingError::UnexpectedToken,
                    &children[inner.pos..],
                ));
            }

            this.advance();
            Some(NodeKind::Block(Block { items, returns_last }))
        })
    }
}

/// let x: T = expr
#[derive(Debug, Clone)]
pub struct ValueDeclaration {
    /// x
    pub declaraion: Box<Node>,
    /// T
    pub type_hint: Option<Box<Node>>,
    /// expr
    pub assigned_value: Box<Node>,
    /// let (immutable) or var (mutable)
    pub mutable: bool,

    /// if there is an error
    pub error: Option<Box<Node>>,
}

impl ValueDeclaration {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        visit(&self.declaraion);
        if let Some(hint) = &self.type_hint {
            visit(hint);
        }
        visit(&self.assigned_value);
        if let Some(err) = &self.error {
            visit(err);
        }
    }
}

impl Parser<'_> {
    pub fn p_value_declaration(&mut self) -> Option<Node> {
        self.make_node(|this| {
            let mutable = match this.next().map(|t| &t.kind) {
                Some(TokenKind::KwVar) => true,
                Some(TokenKind::KwLet) => false,
                _ => return None,
            };
            this.advance();
            // TODO: add destructuring, like let { x, y, z } = make_vec();
            let decl = this.p_identifier().unwrap_or_else(|| {
                this.make_error_here(ParsingError::ExpectedDifferentToken {
                    expected: "variable name",
                })
            });
            let type_hint = if this.advance_on(TokenKind::OpTypedef) {
                Some(Box::new(this.p_expression().unwrap_or_else(|| {
                    this.make_error_here(ParsingError::ExpectedDifferentToken { expected: "type" })
                })))
            } else {
                None
            };
            let error = if this.advance_on(TokenKind::OpAsg) {
                None
            } else {
                Some(Box::new(this.make_error_here(
                    ParsingError::ExpectedDifferentToken { expected: "=" },
                )))
            };
            let expr = this.p_expression().unwrap_or_else(|| {
                this.make_error_here(ParsingError::ExpectedDifferentToken {
                    expected: "variable value",
                })
            });

            Some(NodeKind::ValueDeclaration(ValueDeclaration {
                mutable,
                error,
                type_hint,
                declaraion: Box::new(decl),
                assigned_value: Box::new(expr),
            }))
        })
    }
}

