use crate::tokenizer::token::TokenKind;

use super::{INode, NodeKind, Parser};

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: u64,
    pub overflowed: bool,
}

impl INode for IntegerLiteral {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::NumBinInt) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range);
                let value = u64::from_str_radix(std::str::from_utf8(bytes).unwrap(), 2);

                match value {
                    Ok(v)  => Some(Self { value: v, overflowed: false, }),
                    Err(_) => Some(Self { value: 0, overflowed: true, }),
                }
            }
            Some(TokenKind::NumHexInt) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range);
                let value = u64::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16);
                
                match value {
                    Ok(v)  => Some(Self { value: v, overflowed: false, }),
                    Err(_) => Some(Self { value: 0, overflowed: true, }),
                }
            }
            Some(TokenKind::NumDecInt) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range);
                let value = str::parse::<u64>(std::str::from_utf8(bytes).unwrap());
                
                match value {
                    Ok(v)  => Some(Self { value: v, overflowed: false, }),
                    Err(_) => Some(Self { value: 0, overflowed: true, }),
                }
            }

            _ => None,
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::IntegerLiteral(self)
    }

    fn visit_errors(&self, mut visit: impl FnMut(&'static str)) {
        if self.overflowed {
            visit("Integer value too big, max is 2^64");
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatingLiteral {
    pub value: f64,
}

impl INode for FloatingLiteral {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::NumDecFloat) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range);
                let value = str::parse::<f64>(std::str::from_utf8(bytes).unwrap()).unwrap();

                Some(Self { value })
            }
            _ => None,
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::FloatingLiteral(self)
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl INode for StringLiteral {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::String) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range.start + 1..range.end - 1);

                Some(Self { 
                    value: String::from_utf8(bytes.to_vec()).unwrap()
                })
            }
            _ => None,
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::StringLiteral(self)
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl INode for BooleanLiteral {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::KwTrue)  => Some(Self { value: true }),
            Some(TokenKind::KwFalse) => Some(Self { value: false }),
            _ => None,
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::BooleanLiteral(self)
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl INode for Identifier {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.next().map(|token| token.kind) {
            Some(TokenKind::Identifier) => {
                let range = parser.advance().range.clone();
                let bytes = parser.get_src(range).to_vec();
                Some(Self { value: String::from_utf8(bytes).unwrap() })
            }
            _ => None,
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::Identifier(self)
    }
}

#[derive(Debug, Clone)]
pub struct Void {}

impl INode for Void {
    fn parse(parser: &mut Parser) -> Option<Self> {
        // just ()
        if 
            matches!(parser.next().map(|token| token.kind), Some(TokenKind::RoundL)) &&
            matches!(parser.lookahead(1).map(|token| token.kind), Some(TokenKind::RoundR)) 
        {
            parser.advance();
            parser.advance();
            Some(Self {})
        } else {
            None
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::Void(self)
    }
}

#[derive(Debug, Clone)]
pub struct DontCare {}

impl INode for DontCare {
    fn parse(parser: &mut Parser) -> Option<Self> {
        if parser.advance_if(TokenKind::Ignore) {
            Some(Self {})
        } else {
            None
        }
    }

    fn into_node(self) -> NodeKind {
        NodeKind::DontCare(self)
    }
}