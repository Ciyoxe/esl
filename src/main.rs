pub mod tokenizer;

use tokenizer::token::{Token, TokenKind};

fn debug_file(path: &str) {
    let src = std::fs::read(path).unwrap();
    let mut tokenizer = tokenizer::Tokenizer::new(&src);
    tokenizer.tokenize();
    print_token_tree(&tokenizer.tokens, &src, 0);
}

fn main() {
    debug_file("test.txt");
}

fn print_token_tree(tokens: &[Token], src: &[u8], indent: usize) {
    for token in tokens {
        let content = std::str::from_utf8(&src[token.range.clone()]).unwrap_or("<invalid utf8>");
        let pad = " ".repeat(indent);
        match &token.kind {
            TokenKind::RoundBraces { children } => {
                println!("{pad}RoundBraces -> {content}");
                print_token_tree(children, src, indent + 2);
            }
            TokenKind::SquareBraces { children } => {
                println!("{pad}SquareBraces -> {content}");
                print_token_tree(children, src, indent + 2);
            }
            TokenKind::CurlyBraces { children } => {
                println!("{pad}CurlyBraces -> {content}");
                print_token_tree(children, src, indent + 2);
            }
            _ => {
                println!("{pad}{:?} -> {content}", token.kind);
            }
        }
    }
}
