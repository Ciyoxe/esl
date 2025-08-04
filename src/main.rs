use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

pub mod ast;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct EslParser;

fn print_pair(pair: Pair<Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{}{:?}: {}", indent_str, pair.as_rule(), pair.as_str().trim());

    for inner in pair.into_inner() {
        print_pair(inner, indent + 1);
    }
}

fn main() {
    let src = std::fs::read_to_string("test.txt").unwrap();
    let res = EslParser::parse(Rule::file, &src);

    match res {
        Ok(mut pairs) => {
            let ast = ast::build::build_expression(pairs.next().unwrap());
            println!("{:#?}", ast);
        }
        Err(err) => {
            println!("{}", err.to_string());
        }
    }
}
