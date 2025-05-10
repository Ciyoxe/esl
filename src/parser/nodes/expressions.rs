use crate::tokenizer::token::TokenKind;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ExpressionError {
    NoExpression,
    UnclosedParentheses,
}

#[derive(Clone)]
pub struct Expression {
    pub root: Box<Node>,
    pub errors: Vec<ExpressionError>,
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression").field("errors", &self.errors).finish()
    }
}


impl INode for Expression {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let mut expr_parser = PrattExprParser::new();
        let root = expr_parser.parse_expression_part(parser, 0);
        let errors = expr_parser.errors;

        root.map(|root| Self {
            root: Box::new(root),
            errors,
        })
    }

    fn into_node(self) -> NodeKind {
        NodeKind::Expression(self)
    }

    fn visit_children<'a>(&'a self, mut iter: impl FnMut(&'a Node)) {
        iter(self.root.as_ref());
    }

    fn visit_errors(&self, mut visit: impl FnMut(&'static str)) {
        for error in &self.errors {
            visit(match error {
                ExpressionError::NoExpression => "Expected expression after opening brace",
                ExpressionError::UnclosedParentheses => "Expected closing brace",
            });
        }
    }
}

pub struct PrattExprParser {
    pub expr: Option<Expression>,
    pub errors: Vec<ExpressionError>,
}

impl PrattExprParser {
    pub fn new() -> Self {
        Self {
            expr: None,
            errors: Vec::new(),
        }
    }

    fn parse_operand(parser: &mut Parser) -> Option<Node> {
        Node::parse::<IntegerLiteral>(parser)
            .or_else(|| Node::parse::<FloatingLiteral>(parser))
            .or_else(|| Node::parse::<StringLiteral>(parser))
            .or_else(|| Node::parse::<BooleanLiteral>(parser))
            .or_else(|| Node::parse::<Identifier>(parser))
            .or_else(|| Node::parse::<DontCare>(parser))
            .or_else(|| Node::parse::<Void>(parser))
    }
    
    fn parse_atom(&mut self, parser: &mut Parser) -> Option<Node> {
        // simple operand like 123
        if let Some(node) = Self::parse_operand(parser) {
            return Some(node);
        }
    
        // if there is no operand - maybe it's opening brace
        if parser.advance_if(TokenKind::RoundL) {
            let inner_expr = self.parse_expression_part(parser, 0);
            if inner_expr.is_none() {
                self.errors.push(ExpressionError::NoExpression);
            }

            if !parser.advance_if(TokenKind::RoundR) {
                self.errors.push(ExpressionError::UnclosedParentheses);
            }

            return inner_expr;
        }

        // need to parse atom, but nothing found
        // it can be when expression is like `(a + )`
        None
    }

    // atom or prefix operator
    fn parse_nud(&mut self, parser: &mut Parser) -> Option<Node> {
        if let Some(node) = self.parse_atom(parser) {
            return Some(node);
        }

        if let Some(definition) = OperationDefinition::parse_prefix_operation(parser) {
            let right_operand = self.parse_expression_part(parser, definition.right_binding_power);
            return Some(definition.into_operation_node(
                None, 
                right_operand.map(|node| Box::new(node)),
            ))
        }

        None
    }

    // postfix or infix operator
    fn parse_led(&mut self, parser: &mut Parser, left_node: &mut Option<Node>, min_binding_power: u16) -> bool {
        let mut infix_parsing_failed = false;
        
        if let Some(definition) = OperationDefinition::parse_infix_operation(parser) {
            if definition.left_binding_power >= min_binding_power {
                let right_operand = self.parse_expression_part(parser, definition.right_binding_power);
                
                // we should handle calls and constructors differently
                // because they require closing braces
                match definition.kind {
                    OperationKind::FuncCall  => if !parser.advance_if(TokenKind::RoundR) {
                        assert_eq!(definition.right_binding_power, 0);
                        self.errors.push(ExpressionError::UnclosedParentheses);
                    },
                    OperationKind::ValueCtor => if !parser.advance_if(TokenKind::CurlyR) {
                        assert_eq!(definition.right_binding_power, 0);
                        self.errors.push(ExpressionError::UnclosedParentheses);
                    },
                    OperationKind::TypeCtor  => if !parser.advance_if(TokenKind::SquareR) {
                        assert_eq!(definition.right_binding_power, 0);
                        self.errors.push(ExpressionError::UnclosedParentheses);
                    },
                    _ => { },
                };

                if right_operand.is_none() {
                    infix_parsing_failed = true;
                }

                // replace left node with new operation node
                // even if right operand is None
                // because we need to know if it's error in infix operator, missing right operand
                // if this can be postfix operator - it will be replaced later
                *left_node = Some(definition.into_operation_node(
                    left_node.take().map(|node| Box::new(node)),
                    right_operand.map(|node| Box::new(node)),
                ));

                if infix_parsing_failed {
                    parser.rollback();
                } else {
                    return true;
                }
            } else {
                // we already parsed operator, but we can't use it due to precedence
                // maybe it's postfix operator (for example, a.. instead of a..b)
                parser.rollback();
            }
        }
        
        if let Some(definition) = OperationDefinition::parse_postfix_operation(parser) {
            if definition.left_binding_power >= min_binding_power {
                // replace left node with new operation node
                *left_node = Some(definition.into_operation_node(
                    if infix_parsing_failed {
                        match left_node.take().unwrap() {
                            Node { kind: NodeKind::Operation(op), .. } => op.left_operand,
                            _ => unreachable!()
                        }
                    } else {
                        left_node.take().map(|node| Box::new(node))
                    },
                    None,
                ));
                return true;
            } else {
                // we already parsed operator, but we can't use it due to precedence
                // this is expression end
                parser.rollback();
            }
        }

        false
    }

    fn parse_expression_part(&mut self,parser: &mut Parser, min_binding_power: u16) -> Option<Node> {
        let mut left_node = self.parse_nud(parser);
        while self.parse_led(parser, &mut left_node, min_binding_power) {}
        left_node
    }
}