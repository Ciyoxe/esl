use parser_t::{Expression, IntegerLiteral, Node, NodeKind};

pub mod tokenizer;
pub mod parser;
pub mod parser_t;

fn main() {
    let src = "-123? + (5 * 6 + 7)".as_bytes();
    let mut tok = tokenizer::Tokenizer::new(src);
    tok.tokenize();
    tok.print_tokens();

    let mut par = parser_t::Parser::new(src, &tok.tokens);

    let node = Node::parse::<Expression>(&mut par);
    println!("{:#?}", match node {
        Some(Node { kind: NodeKind::Expression(v), .. }) => v.root,
        _ => unreachable!(),
    });
}
