use parser::{Debugger, Expression, Node};

pub mod tokenizer;
pub mod parser;

fn main() {
    let src = "(1 + 2).. * 3".as_bytes();
    let mut tok = tokenizer::Tokenizer::new(src);
    tok.tokenize();
    tok.print_tokens();

    let mut par = parser::Parser::new(src, &tok.tokens);

    let node = Node::parse::<Expression>(&mut par);
    match node {
        Some(node) => Debugger::print_nodes_tree(&node, &par),
        None => println!("Error parsing expression"),
    }
}
