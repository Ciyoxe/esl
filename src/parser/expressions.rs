use crate::{parser::operations::OperationArity, tokenizer::token::TokenKind};
use super::{
    node::{Node, NodeKind},
    operations::OperationSettings,
};

pub enum ExpressionBuildPart {
    Atom { node: Node },
    Token { token: TokenKind, position: usize },
}

enum RawFlatPart {
    Atom {
        node: Node,
    },
    Brace {
        token: TokenKind,
        position: usize,
    },
    Operations {
        variants: &'static [OperationSettings],
        position: usize,
    },
}

impl RawFlatPart {
    fn is_identifier(&self) -> bool {
        matches!(
            self,
            RawFlatPart::Atom {
                node: Node {
                    kind: NodeKind::Identifier { .. },
                    ..
                }
            }
        )
    }

    fn is_closing_brace(&self) -> bool {
        matches!(
            self,
            RawFlatPart::Brace {
                token: TokenKind::RoundR | TokenKind::SquareR | TokenKind::CurlyR,
                ..
            }
        )
    }

    fn is_opening_brace(&self) -> bool {
        matches!(
            self,
            RawFlatPart::Brace {
                token: TokenKind::RoundL | TokenKind::SquareL | TokenKind::CurlyL,
                ..
            }
        )
    }

    fn is_pair_brace_for(&self, brace: TokenKind) -> bool {
        let pair_brace = match brace {
            TokenKind::SquareL => TokenKind::SquareR,
            TokenKind::RoundL => TokenKind::RoundR,
            TokenKind::CurlyL => TokenKind::CurlyR,
            _ => unreachable!(),
        };
        matches!(self, RawFlatPart::Brace { token, .. } if *token == pair_brace)
    }

    fn can_be_right_operand(&self) -> bool {
        self.is_opening_brace()
            || matches!(self, RawFlatPart::Atom { .. })
            || matches!(
                self,
                RawFlatPart::Operations { variants, .. }
                    if variants
                    .iter()
                    .any(|v| matches!(v.arity, OperationArity::UnaryPostfix | OperationArity::Nullary)
                )
            )
    }
}
enum ConcreteFlatPart {
    Atom {
        node: Node,
    },
    Brace {
        token: TokenKind,
        position: usize,
    },
    Operation {
        variant: OperationSettings,
        position: usize,
    },
}

impl ConcreteFlatPart {
    fn can_be_left_operand(&self) -> bool {
        matches!(
            self,
            ConcreteFlatPart::Operation {
                variant: OperationSettings {
                    arity: OperationArity::UnaryPostfix | OperationArity::Nullary,
                    ..
                },
                ..
            } | ConcreteFlatPart::Brace {
                token: TokenKind::RoundL | TokenKind::SquareL | TokenKind::CurlyL,
                ..
            } | ConcreteFlatPart::Atom { .. }
        )
    }
}

pub struct ExpressionParser {
    braces_stack: Vec<TokenKind>,

    pub raw_parts: Vec<RawFlatPart>,
    pub concrete_parts: Vec<ConcreteFlatPart>,
    pub expression_errors: Vec<Node>,
}

impl ExpressionParser {
    pub fn new() -> Self {
        Self {
            braces_stack: Vec::new(),

            raw_parts: Vec::new(),
            concrete_parts: Vec::new(),
            expression_errors: Vec::new(),
        }
    }

    fn consume_token(&mut self, token: TokenKind, position: usize) -> bool {
        // Prevent parsing after comma in top scope
        // To not parse something like `struct_field1: a, struct_field2: b` as one expression
        if token == TokenKind::OpComma && self.braces_stack.is_empty() {
            return false;
        }

        match token {
            TokenKind::RoundL | TokenKind::SquareL | TokenKind::CurlyL => {
                // `x(a, b)` => `x call (a, b)`
                // `(x)(a, b)` => `(x) call (a, b)`
                if self
                    .raw_parts
                    .last()
                    .is_some_and(|part| part.is_closing_brace() || part.is_identifier())
                {
                    self.raw_parts.push(RawFlatPart::Operations {
                        variants: OperationSettings::for_brace(token),
                        position,
                    });
                }

                self.raw_parts.push(RawFlatPart::Brace { token, position });
                self.braces_stack.push(token);
            }

            TokenKind::RoundR | TokenKind::SquareR | TokenKind::CurlyR => {
                if self.braces_stack.is_empty() {
                    return false;
                }

                // collapse () [] {} pairs in stack
                if self
                    .raw_parts
                    .last()
                    .is_some_and(|part| part.is_pair_brace_for(token))
                {
                    self.braces_stack.pop();
                    self.raw_parts.push(RawFlatPart::Brace { token, position });
                } else {
                    self.expression_errors.push(Node {
                        kind: NodeKind::ErrMismatchedBrace,
                        range: position..position + 1,
                    });
                }
            }

            anyother => {
                let variants = OperationSettings::for_exact_token(anyother);
                if !variants.is_empty() {
                    self.raw_parts
                        .push(RawFlatPart::Operations { variants, position });
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn concretize_operations(&mut self) -> Vec<ConcreteFlatPart> {
        let mut concretized = Vec::new();
        let mut has_prev_operand = false;

        for part in self.raw_parts.iter() {
            match part {
                RawFlatPart::Operations { variants, position } => {
                    let has_next_operand = part.can_be_right_operand();
                    let mut matched = false;

                    for variant in variants.into_iter() {
                        if variant.arity == OperationArity::Binary
                            && has_prev_operand
                            && has_next_operand
                        {
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            has_prev_operand = false;
                            matched = true;
                            break;
                        }
                        if variant.arity == OperationArity::UnaryPrefix && has_next_operand {
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            has_prev_operand = false;
                            matched = true;
                            break;
                        }
                        if variant.arity == OperationArity::UnaryPostfix && has_prev_operand {
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            has_prev_operand = true;
                            matched = true;
                            break;
                        }
                        if variant.arity == OperationArity::Nullary {
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            has_prev_operand = true;
                            matched = true;
                            break;
                        }
                    }

                    if !matched {
                        let variant = variants.last().unwrap();
                        if variant.arity == OperationArity::Binary {
                            if !has_prev_operand {
                                concretized.push(ConcreteFlatPart::Atom {
                                    node: Node {
                                        kind: NodeKind::ErrMissingOperand,
                                        range: *position..*position,
                                    },
                                });
                            }
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            if !has_next_operand {
                                concretized.push(ConcreteFlatPart::Atom {
                                    node: Node {
                                        kind: NodeKind::ErrMissingOperand,
                                        range: *position..*position,
                                    },
                                });
                            }
                        }
                        if variant.arity == OperationArity::UnaryPrefix {
                            concretized.push(ConcreteFlatPart::Operation {
                                variant: *variant,
                                position: *position,
                            });
                            if !has_next_operand {
                                concretized.push(ConcreteFlatPart::Atom {
                                    node: Node {
                                        kind: NodeKind::ErrMissingOperand,
                                        range: *position..*position,
                                    },
                                });
                            }
                        }
                        if variant.arity == OperationArity::UnaryPostfix {
                            if !has_prev_operand {
                                concretized.push(ConcreteFlatPart::Atom {
                                    node: Node {
                                        kind: NodeKind::ErrMissingOperand,
                                        range: *position..*position,
                                    },
                                });
                            }
                        }
                    }
                }
                RawFlatPart::Atom { node } => {
                    concretized.push(ConcreteFlatPart::Atom { node: node.clone() });
                    has_prev_operand = true;
                }
                RawFlatPart::Brace { token, position } => {
                    has_prev_operand = matches!(token, TokenKind::RoundL | TokenKind::SquareL | TokenKind::CurlyL);
                    concretized.push(ConcreteFlatPart::Brace {
                        token: *token,
                        position: *position,
                    });
                }
            }
        }

        concretized
    }

    pub fn consume_next_part(&mut self, part: ExpressionBuildPart) -> bool {
        match part {
            ExpressionBuildPart::Atom { node } => {
                self.raw_parts.push(RawFlatPart::Atom { node });
                true
            }
            ExpressionBuildPart::Token { token, position } => self.consume_token(token, position),
        }
    }
}
