use pest::{iterators::Pair, pratt_parser::PrattParser};

#[derive(pest_derive::Parser)]
#[grammar = "./polenta.pest"]
pub struct PolentaParser;

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
}

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Integer(u64),
    Eval(String, Box<Expr>),
    UnaryOp {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    BinaryOp {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Expr),
    LetPoly(String, String, Expr),
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

/// Parse an expression.
fn parse_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr);
    let pairs = pair.into_inner();

    EXPR_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<u64>().unwrap()),
            Rule::expr => parse_expr(primary),
            Rule::identifier => Expr::Identifier(primary.as_str().to_string()),
            _rule => unreachable!("Expr::parse expected atom, found {:?}", _rule),
        })
        .map_infix(|lhs, op, rhs| Expr::BinaryOp {
            lhs: Box::new(lhs),
            op: match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::subtract => BinaryOp::Subtract,
                Rule::multiply => BinaryOp::Multiply,
                Rule::divide => BinaryOp::Divide,
                Rule::modulo => BinaryOp::Modulo,
                Rule::power => BinaryOp::Power,
                _rule => unreachable!("Expr::parse expected infix operation, found {:?}", _rule),
            },
            rhs: Box::new(rhs),
        })
        .map_prefix(|op, rhs| Expr::UnaryOp {
            op: match op.as_rule() {
                Rule::minus => UnaryOp::Minus,
                _rule => unreachable!("Expr::parse expected prefix operation, found {:?}", _rule),
            },
            rhs: Box::new(rhs),
        })
        .parse(pairs)
}

fn parse_let_expr(pair: Pair<Rule>) -> Stmt {
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
    let expr = parse_expr(pair);

    assert!(pairs.next().is_none());
    Stmt::Let(identifier, expr)
}

fn parse_let_poly_expr(pair: Pair<Rule>) -> Stmt {
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
    let expr = parse_expr(pair);

    assert!(pairs.next().is_none());
    Stmt::LetPoly(identifier, term, expr)
}

pub fn parse_polenta(pair: Pair<Rule>) -> Vec<Stmt> {
    pair.into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::expr => Stmt::Expr(parse_expr(pair)),
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
        let pairs = PolentaParser::parse(Rule::polenta, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;

        for pair in pairs {
            println!("Parsed: {:#?}", parse_polenta(pair));
        }

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
    fn test_multi_let_expr() {
        parse("let a = -1 + 2; let b = 3 * 4;").expect("should parse");
    }

    #[test]
    fn test_let_poly_expr() {
        parse("let P(x) = -1 + 3 * x;").expect("should parse");
    }
}
