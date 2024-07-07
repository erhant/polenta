use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(pest_derive::Parser)]
#[grammar = "./calc.pest"]
pub struct CalculatorParser;

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Integer(i32),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest below:
        // binary +, -
        // binary *, /, %
        // unary -
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left) | Op::infix(power, Right))
            .op(Op::prefix(unary_minus))
    };
}

pub fn parse_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr);
    let pairs = pair.into_inner();

    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::expr => parse_expr(primary),
            Rule::identifier => Expr::Identifier(primary.as_str().to_string()),

            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::power => Op::Power,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn parse_let_expr(pair: Pair<Rule>) -> (String, Expr) {
    assert_eq!(pair.as_rule(), Rule::let_expr);
    let mut pairs = pair.into_inner();

    // let <identifier> = <expr> ;
    //     ^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::identifier);
    let identifier = pair.as_str().to_string();

    // let <identifier> = <expr> ;
    //                    ^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    assert!(pairs.next().is_none());
    (identifier, expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_main_expression(expression: &str) -> Result<(), String> {
        let mut pairs = CalculatorParser::parse(Rule::expr_main, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;

        println!("Parsed: {:#?}", parse_expr(pairs.next().unwrap()));

        Ok(())
    }

    fn parse_let_expression(expression: &str) -> Result<(), String> {
        let mut pairs = CalculatorParser::parse(Rule::let_expr_main, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;

        for pair in pairs.next().unwrap().into_inner() {
            println!("Parsed: {:#?}", parse_let_expr(pair));
        }

        Ok(())
    }

    #[test]
    fn test_parse_main_expression() {
        parse_main_expression("-1 + x * 3 * 3").expect("should parse");
    }

    #[test]
    fn test_parse_let_expression() {
        parse_let_expression("let x = 4 * 3; let y = 6 ^ x;").expect("should parse");
    }
}
