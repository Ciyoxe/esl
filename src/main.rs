pub mod tokenizer;

struct Parser {}

fn main() {
    let src = "\"some str\"\"".as_bytes();
    let mut tok = tokenizer::tokenizer::Tokenizer::new(src);
    tok.tokenize().iter().for_each(|t| {
        println!("{:?}", t);
    });
}
