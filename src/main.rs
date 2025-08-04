pub mod parser;
pub mod ast;

fn main() {
    let src = std::fs::read_to_string("test.txt").unwrap();
}
