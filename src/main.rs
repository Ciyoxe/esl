pub mod tokenizer;
pub mod parser;

fn main() {
    let src = "\"some str\"({)} @11233".as_bytes();
    let mut tok = tokenizer::Tokenizer::new(src);
    tok.tokenize();
    tok.print_tokens();
}
