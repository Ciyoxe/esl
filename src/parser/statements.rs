use crate::{
    parser::{Parser, errors::ParsingError, nodes::{Node, NodeKind}},
    tokenizer::token::TokenKind,
};

#[derive(Debug, Clone)]
pub struct ValueDeclaration {
    pub decl: Box<Node>,
    pub expr: Box<Node>,
    pub mutable: bool,
    pub error: Option<Box<Node>>,
}

impl ValueDeclaration {
    pub fn visit_children<'a>(&'a self, mut visit: impl FnMut(&'a Node)) {
        visit(&self.decl);
        visit(&self.expr);
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
            // TODO: add destructuring, like let { x, y, z } = make_vec();
            let decl = this.p_identifier().unwrap_or_else(|| {
                this.make_error_here(ParsingError::ExpectedDifferentToken {
                    expected: "variable name",
                })
            });
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

            this.advance();
            Some(NodeKind::ValueDeclaration(ValueDeclaration {
                mutable,
                error,
                decl: Box::new(decl),
                expr: Box::new(expr),
            }))
        })
    }
}
