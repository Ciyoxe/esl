use crate::{
    parser::{
        Parser,
        errors::ParsingError,
        nodes::{Node, NodeKind},
    },
    tokenizer::token::{Token, TokenKind},
};

/// x: T = default
#[derive(Debug, Clone)]
pub struct LambdaArg {
    /// x
    pub name: Box<Node>,
    /// T
    pub type_hint: Option<Box<Node>>,
    /// default
    pub default_value: Option<Box<Node>>,
}

impl LambdaArg {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        visit(&self.name);
        if let Some(hint) = &self.type_hint {
            visit(hint);
        }
        if let Some(default) = &self.default_value {
            visit(default);
        }
    }
}

/// (a,b,c) -> { ... }
#[derive(Debug, Clone)]
pub struct Lambda {
    /// a, b, c
    pub args: Vec<LambdaArg>,
    /// block or expr
    pub body: Box<Node>,
}

impl Lambda {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        for arg in &self.args {
            arg.visit_children(&mut visit);
        }
        visit(&self.body);
    }
}

impl Parser<'_> {
    fn p_lambda_arg(&mut self) -> Option<LambdaArg> {
        let name = self.p_identifier()?;

        let type_hint = if self.advance_on(TokenKind::OpTypedef) {
            Some(Box::new(self.p_expression().unwrap_or_else(|| {
                self.make_error_here(ParsingError::ExpectedDifferentToken { expected: "type" })
            })))
        } else {
            None
        };

        let default_value = if self.advance_on(TokenKind::OpAsg) {
            Some(Box::new(self.p_expression().unwrap_or_else(|| {
                self.make_error_here(ParsingError::ExpectedDifferentToken { expected: "default value" })
            })))
        } else {
            None
        };

        Some(LambdaArg {
            name: Box::new(name),
            type_hint,
            default_value,
        })
    }

    pub fn p_lambda(&mut self) -> Option<Node> {
        self.make_node(|this| {
            // Check for OpLam before consuming args
            let is_lambda = match this.next().map(|t| &t.kind) {
                Some(TokenKind::Identifier | TokenKind::RoundBraces { .. }) => {
                    matches!(this.next_at(1).map(|t| &t.kind), Some(TokenKind::OpLam))
                }
                _ => false,
            };

            if !is_lambda {
                return None;
            }

            // Parse args
            let args = match &this.next()?.kind {
                TokenKind::Identifier => {
                    let name = this.p_identifier()?;
                    vec![LambdaArg {
                        name: Box::new(name),
                        type_hint: None,
                        default_value: None,
                    }]
                }
                TokenKind::RoundBraces { children } => {
                    let mut inner = Parser::new(this.src, children);
                    let mut args = Vec::new();

                    while let Some(arg) = inner.p_lambda_arg() {
                        args.push(arg);
                        if !inner.advance_on(TokenKind::OpComma) {
                            break;
                        }
                    }

                    if inner.pos < children.len() {
                        args.push(LambdaArg {
                            name: Box::new(this.make_error_for_tokens(
                                ParsingError::UnexpectedToken,
                                &children[inner.pos..],
                            )),
                            type_hint: None,
                            default_value: None,
                        });
                    }

                    this.advance();
                    args
                }
                _ => return None,
            };

            this.advance(); // consume OpLam

            let body = this.p_block()
                .or_else(|| this.p_expression())
                .unwrap_or_else(|| {
                    this.make_error_here(ParsingError::ExpectedDifferentToken {
                        expected: "lambda body",
                    })
                });

            Some(NodeKind::Lambda(Lambda {
                args,
                body: Box::new(body),
            }))
        })
    }
}
