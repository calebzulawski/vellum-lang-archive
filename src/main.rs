mod ast;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().skip(1).next().unwrap();
    let contents = std::fs::read_to_string(&filename)?;
    let program = GrammarParser::parse(Rule::program, &contents);
    match program {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
    Ok(())
}
