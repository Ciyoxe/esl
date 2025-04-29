pub mod tokenizer;
pub mod parser;

fn main() {
    let src = "a + b + c * (234 + true) * xa".as_bytes();
    let mut tok = tokenizer::Tokenizer::new(src);
    tok.tokenize();
    tok.print_tokens();

    let mut par = parser::Parser::new(src, &tok.tokens);

    println!("_____________");
    par.p_expression();
}
