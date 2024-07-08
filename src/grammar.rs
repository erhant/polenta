use pest::{iterators::Pair, pratt_parser::PrattParser};

#[derive(pest_derive::Parser)]
#[grammar = "./polenta.pest"]
pub struct PolentaParser;

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
    Integer(u64),
    UnaryMinus(Box<Expr>),
    Let(String, Box<Expr>),
    LetPoly(String, String, Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

// Pratt parser for expressions with operator precedence.
lazy_static::lazy_static! {
    static ref EXPR_PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest below:
        // binary +, -
        // binary *, /, %, ^
        // unary -
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left) | Op::infix(power, Right))
            .op(Op::prefix(minus))
    };
}

/// Parse an expression. There are 3 types:
///
/// - Plain mathematical operation such as `4 * 2 + 1`
/// - Let expression `let x = 4 * 2 + 1;`
/// - Let poly expression `let P(x) = 4*x^2 + x + 1;`
///
/// When the expression is that of a poly, the `term` parameter is used to determine the term of the poly (e.g. `x`).
pub fn parse_expr(pair: Pair<Rule>, term: Option<&String>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr);
    let pairs = pair.into_inner();

    EXPR_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<u64>().unwrap()),
            Rule::expr => parse_expr(primary, term),
            Rule::identifier => Expr::Identifier(primary.as_str().to_string()),
            _rule => unreachable!("Expr::parse expected atom, found {:?}", _rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::power => Op::Power,
                _rule => unreachable!("Expr::parse expected infix operation, found {:?}", _rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn parse_let_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::let_expr);
    let mut pairs = pair.into_inner();

    // let <identifier> = <expr> ;
    //     ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::identifier);
    let identifier = pair.as_str().to_string();

    // let <identifier> = <expr> ;
    //                    ^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair, None);

    assert!(pairs.next().is_none());
    Expr::Let(identifier, Box::new(expr))
}

pub fn parse_let_poly_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::let_poly_expr);
    let mut pairs = pair.into_inner();

    // let <identifier> ( <identifier> ) = <expr> ;
    //     ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::identifier);
    let identifier = pair.as_str().to_string();

    // let <identifier> ( <identifier> ) = <expr> ;
    //                    ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::identifier);
    let term = pair.as_str().to_string();

    // let <identifier> ( <identifier> ) = <expr> ;
    //                                     ^^^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair, Some(&term));

    assert!(pairs.next().is_none());
    Expr::LetPoly(identifier, term, Box::new(expr))
}

pub fn parse_polenta(pair: Pair<Rule>) -> Vec<Expr> {
    pair.into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::expr => parse_expr(pair, None),
            Rule::let_expr => parse_let_expr(pair),
            Rule::let_poly_expr => parse_let_poly_expr(pair),
            _rule => unreachable!("E, found {:?}", _rule),
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::PolentaParser;
    use pest::Parser;

    fn parse(expression: &str) -> Result<(), String> {
        let mut pairs = PolentaParser::parse(Rule::polenta, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;

        println!("Parsed: {:#?}", parse_polenta(pairs.next().unwrap()));

        Ok(())
    }

    #[test]
    fn test_expr() {
        parse("-1 + x * 3 * 3").expect("should parse");
    }

    #[test]
    fn test_let_expr() {
        parse("let a = -1 + 3 * 3;").expect("should parse");
    }

    #[test]
    fn test_let_poly_expr() {
        parse("let P(x) = -1 + 3 * x;").expect("should parse");
    }
}
