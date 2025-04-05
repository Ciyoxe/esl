pub mod tokenizer;

fn main() {
    let src = "\"some str\"\"".as_bytes();
    let tok = tokenizer::Tokenizer::new(src);
    tok.print_debug();
}
