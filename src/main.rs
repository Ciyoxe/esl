pub mod tokenizer;
pub mod parser;

fn main() {
    let src = "\"some str\"({)}".as_bytes();
    let tok = tokenizer::Tokenizer::new(src);
    tok.print_debug();
}
