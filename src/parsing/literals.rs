use crate::{
    all, any,
    ast::AstNode,
    parser::{Parser, combinators::repeat},
    parsing::ParsingError,
};

pub fn parse_tag(p: &mut Parser<u8>, tag: &[u8]) -> Option<()> {
    for (i, byte) in tag.iter().enumerate() {
        if !p.peek_at(i).map_or(false, |token| token == byte) {
            return None;
        }
    }
    for _ in 0..tag.len() {
        p.next();
    }
    Some(())
}
pub fn parse_decimal(p: &mut Parser<u8>) -> Option<u8> {
    p.test_and_skip(|token| token.is_ascii_digit()).copied()
}
pub fn parse_hexadecimal(p: &mut Parser<u8>) -> Option<u8> {
    p.test_and_skip(|token| token.is_ascii_hexdigit()).copied()
}

pub fn parse_number(p: &mut Parser<u8>) -> Option<AstNode> {
    if !p.test(|b| b.is_ascii_digit()) {
        return None;
    }

    p.enter_scope(());

    any!(p,
        |p| all!(p,
            |p| parse_tag(p, b"0x"),
            |p| repeat(p, 1.., |p| parse_hexadecimal(p)),
        ),
        |p| all!(p,
            |p| parse_tag(p, b"0b"),
            |p| repeat(p, 1.., |p| parse_decimal(p)),
        ),
        |p| all!(p,
            |p| repeat(p, 1.., |p| parse_decimal(p)),
        ),
    );

    p.exit_scope();
}
