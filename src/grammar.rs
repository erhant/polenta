use pest::Parser;
use pest::{iterators::Pair, pratt_parser::PrattParser};

#[derive(pest_derive::Parser)]
#[grammar = "./polenta.pest"]
pub struct PolentaParser;

impl PolentaParser {
    /// Parses an input string into a vector of statements.
    pub fn parse_input(input: &str) -> Result<Vec<Stmt>, pest::error::Error<Rule>> {
        let pairs = Self::parse(Rule::polenta, input)?;

        let stmts = pairs
            .into_iter()
            .filter_map(|pair| match pair.as_rule() {
                Rule::EOI => {
                    // TODO: log
                    None
                }
                rule => {
                    assert_eq!(rule, Rule::polenta_stmts);
                    Some(Self::parse_statement(pair.into_inner().next().unwrap()))
                }
            })
            .collect();

        Ok(stmts)
    }

    /// Parses a given pair into a statement.
    pub fn parse_statement(pair: Pair<Rule>) -> Stmt {
        match pair.as_rule() {
            Rule::expr_stmt => parse_expr_stmt(pair),
            Rule::let_stmt => parse_let_stmt(pair),
            Rule::let_poly_stmt => parse_let_poly_stmt(pair),
            _rule => unreachable!("Expected statement, found {:?}", _rule),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Eval,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Integer(u64),
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

#[derive(Debug, Clone)]
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
        // binary *, /
        // binary ^, %,
        // binary @
        // unary -
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(modulo, Right) | Op::infix(power, Right))
            .op(Op::infix(eval, Right))
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
                Rule::eval => BinaryOp::Eval,
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

// TODO: parse Stmt

fn parse_expr_stmt(pair: Pair<Rule>) -> Stmt {
    assert_eq!(pair.as_rule(), Rule::expr_stmt);
    let mut pairs = pair.into_inner();

    // expr ;
    // ^^^^
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    assert!(pairs.next().is_none());
    Stmt::Expr(expr)
}

fn parse_let_stmt(pair: Pair<Rule>) -> Stmt {
    debug_assert_eq!(pair.as_rule(), Rule::let_stmt);
    let mut pairs = pair.into_inner();

    // let <identifier> = <expr> ;
    //     ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::identifier);
    let identifier = pair.as_str().to_string();

    // let <identifier> = <expr> ;
    //                    ^^^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    debug_assert!(pairs.next().is_none());
    Stmt::Let(identifier, expr)
}

fn parse_let_poly_stmt(pair: Pair<Rule>) -> Stmt {
    debug_assert_eq!(pair.as_rule(), Rule::let_poly_stmt);
    let mut pairs = pair.into_inner();

    // let <identifier> ( <identifier> ) = <expr> ;
    //     ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::identifier);
    let identifier = pair.as_str().to_string();

    // let <identifier> ( <identifier> ) = <expr> ;
    //                    ^^^^^^^^^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::identifier);
    let term = pair.as_str().to_string();

    // let <identifier> ( <identifier> ) = <expr> ;
    //                                     ^^^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    debug_assert!(pairs.next().is_none());
    Stmt::LetPoly(identifier, term, expr)
}

#[cfg(test)]
mod tests {
    use crate::grammar::PolentaParser;

    fn run_test(expression: &str) {
        let stmts = PolentaParser::parse_input(expression).unwrap();
        for stmt in stmts {
            println!("{:?}", stmt);
        }
    }

    #[test]
    fn test_expr_stmt() {
        run_test("-1 + x * 3 * 3;");
    }

    #[test]
    fn test_let_stmt() {
        run_test("let a = -1 + 3 * 3;");
    }

    #[test]
    fn test_multi_let_stmt() {
        run_test("let a = -1 + 2; let b = 3 * 4;");
    }

    #[test]
    fn test_poly_powers() {
        run_test("let P(x) = x^2 + x + 4;");
    }

    #[test]
    fn test_let_poly_stmt() {
        run_test("let P(x) = -1 + 3 * x;");
    }

    #[test]
    fn test_let_poly_eval_stmt() {
        run_test("let P(x) = x + 2; let e = P@2;");
    }
}
