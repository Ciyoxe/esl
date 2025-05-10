use parser::{Debugger, Expression, Node};

pub mod tokenizer;
pub mod parser;

fn debug_file(path: &str) {
    let src = std::fs::read(path).unwrap();
    let mut tokenizer = tokenizer::Tokenizer::new(&src);
    tokenizer.tokenize();

    let mut parser = parser::Parser::new(&src, &tokenizer.tokens);
    let node = Node::parse::<Expression>(&mut parser).unwrap();

    Debugger::print_nodes_tree(&node, &parser);
}

fn main() {
    debug_file("test.txt");
}
