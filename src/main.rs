use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./src/lang/grammar.pest"]
pub struct PolentaParser;

fn parse_expression(pair: pest::iterators::Pair<Rule>) {
    let mut pairs = pair.into_inner();
    let lhs = pairs.next().unwrap();
    let op = pairs.next().unwrap();
    let rhs = pairs.next().unwrap();
    println!("lhs: {:?}, op: {:?}, rhs: {:?}", lhs.as_str(), op.as_str(), rhs.as_str());
}

fn main() {
    let mut example = PolentaParser::parse(Rule::MAIN, "2*x + 3*x^2 + 5").expect("parse");

    parse_expression(example.next().unwrap());


}
