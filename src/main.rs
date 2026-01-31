pub mod parser;
pub mod tokenizer;

use tokenizer::token::{Token, TokenKind};
use std::hint::black_box;

fn debug_file(path: &str) {
    let src = std::fs::read(path).unwrap();
    let mut tokenizer = tokenizer::Tokenizer::new(&src);
    tokenizer.tokenize();

    print_token_tree(&tokenizer.tokens, &src, 0);

    let mut parser = parser::Parser::new(&src, &tokenizer.tokens);
    let expr = parser.p_expression();

    match expr {
        Some(e) => parser::debugger::Debugger::print_nodes_tree(&e, &parser),
        _ => println!("No expression"),
    };
}

fn main() {
    println!("=== Expression Parser Benchmark ===\n");

    bench_expression("Simple", "a + b * c - d / e", 10000);
    bench_expression("Medium", &generate_expr_chain(20), 5000);
    bench_expression("Complex", &generate_expr_nested(5), 2000);
    bench_expression("Very Complex", &generate_expr_mixed(10), 1000);

    println!("\n=== File Benchmark ===\n");
    bench_file("test.txt", 100);
}

fn generate_expr_chain(length: usize) -> String {
    let mut expr = String::from("a");
    let ops = [" + ", " - ", " * ", " / ", " & ", " | "];
    for i in 1..length {
        expr.push_str(ops[i % ops.len()]);
        expr.push((b'a' + (i % 26) as u8) as char);
    }
    expr
}

fn generate_expr_nested(depth: usize) -> String {
    if depth == 0 {
        return "x".to_string();
    }
    format!("({} + {} * {})",
        generate_expr_nested(depth - 1),
        generate_expr_nested(depth - 1),
        generate_expr_nested(depth - 1)
    )
}

fn generate_expr_mixed(size: usize) -> String {
    let mut expr = String::new();
    for i in 0..size {
        if i > 0 { expr.push_str(" + "); }
        expr.push_str(&format!("f{}(a, b)", i));
    }
    expr
}

fn bench_expression(name: &str, expr: &str, iterations: usize) {
    let src = expr.as_bytes();
    let bytes = src.len() as f64;
    let mut tok_time = 0.0;
    let mut parse_time = 0.0;

    for _ in 0..iterations {
        let start = std::time::Instant::now();
        let mut tokenizer = tokenizer::Tokenizer::new(src);
        tokenizer.tokenize();
        black_box(&tokenizer.tokens);
        tok_time += start.elapsed().as_secs_f64();

        let start = std::time::Instant::now();
        let mut parser = parser::Parser::new(src, &tokenizer.tokens);
        let result = parser.p_expression();
        black_box(result);
        parse_time += start.elapsed().as_secs_f64();
    }

    let total_mb = bytes * iterations as f64 / 1_000_000.0;
    let total_time = tok_time + parse_time;

    println!("{}: {} bytes", name, expr.len());
    println!("  Total:    {:.2} MB/s ({:.3}s for {:.2} MB)",
        total_mb / total_time, total_time, total_mb);
    println!("  Tokenize: {:.2} MB/s", total_mb / tok_time.max(f64::MIN_POSITIVE));
    println!("  Parse:    {:.2} MB/s", total_mb / parse_time.max(f64::MIN_POSITIVE));
    println!();
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

fn bench_file(path: &str, iterations: usize) {
    let src = std::fs::read(path).unwrap();
    let bytes = src.len() as f64;
    let mut tok_time = 0.0;
    let mut parse_time = 0.0;

    for _ in 0..iterations {
        let start = std::time::Instant::now();
        let mut tokenizer = tokenizer::Tokenizer::new(&src);
        tokenizer.tokenize();
        black_box(&tokenizer.tokens);
        tok_time += start.elapsed().as_secs_f64();

        let start = std::time::Instant::now();
        let mut parser = parser::Parser::new(&src, &tokenizer.tokens);
        let mut count = 0usize;
        while parser.next().is_some() {
            if parser.p_expression().is_some() {
                count += 1;
            } else {
                parser.advance(); // skip token to avoid infinite loop
            }
        }
        black_box(count);
        parse_time += start.elapsed().as_secs_f64();
    }

    let total_mb = bytes * iterations as f64 / (1024.0 * 1024.0);
    let total_time = (tok_time + parse_time).max(f64::MIN_POSITIVE);
    let total_mb_s = total_mb / total_time;
    let tok_mb_s = total_mb / tok_time.max(f64::MIN_POSITIVE);
    let parse_mb_s = total_mb / parse_time.max(f64::MIN_POSITIVE);

    println!(
        "bench: {:.2} MB processed in {:.3}s -> {:.2} MB/s ({} iters)",
        total_mb, total_time, total_mb_s, iterations
    );
    println!(
        "  tokenize: {:.3}s -> {:.2} MB/s",
        tok_time, tok_mb_s
    );
    println!(
        "  parse:    {:.3}s -> {:.2} MB/s",
        parse_time, parse_mb_s
    );
}
