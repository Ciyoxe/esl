pub mod parser;
pub mod ast;
pub mod parsing;

fn main() {
    let src = std::fs::read_to_string("test.txt").unwrap();
}
