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
                Rule::EOI => None,
                Rule::polenta_stmts => Some(Self::parse_statement(pair)),
                _ => unreachable!(),
            })
            .collect();

        Ok(stmts)
    }

    /// Parses a given pair into a statement.
    pub fn parse_statement(pair: Pair<Rule>) -> Stmt {
        assert_eq!(pair.as_rule(), Rule::polenta_stmts);

        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::expr_stmt => parse_expr_stmt(pair),
            Rule::let_stmt => parse_let_stmt(pair),
            Rule::let_poly_stmt => parse_let_poly_stmt(pair),
            Rule::assert_stmt => parse_assert_stmt(pair),
            _ => unreachable!(),
        }
    }
}

/// Binary operators.
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Evl,
}

/// Unary operators.
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
}

/// Expressions.
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

/// Statements.
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Expr),
    LetPoly(String, String, Expr),
    Assert(Expr),
}

// Pratt parser for expressions with operator precedence.
lazy_static::lazy_static! {
    static ref EXPR_PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest below.
        // see also: https://en.cppreference.com/w/c/language/operator_precedence
        PrattParser::new()
            .op(Op::infix(eq, Left) | Op::infix(ne, Left)) // ==, !=
            .op(Op::infix(add, Left) | Op::infix(subtract, Left)) // +, -
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left)) // *, /, %
            .op(Op::infix(power, Right)) // ^
            .op(Op::infix(eval, Right)) // @
            .op(Op::prefix(minus)) // -
    };
}

/// Parse an expression from a given `Pair`.
fn parse_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr);
    let pairs = pair.into_inner();

    EXPR_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<u64>().unwrap()),
            Rule::expr => parse_expr(primary),
            Rule::identifier => Expr::Identifier(primary.as_str().to_string()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| Expr::BinaryOp {
            lhs: Box::new(lhs),
            op: match op.as_rule() {
                Rule::add => BinaryOp::Add,
                Rule::subtract => BinaryOp::Sub,
                Rule::multiply => BinaryOp::Mul,
                Rule::divide => BinaryOp::Div,
                Rule::modulo => BinaryOp::Mod,
                Rule::power => BinaryOp::Pow,
                Rule::eval => BinaryOp::Evl,
                Rule::eq => BinaryOp::Eq,
                Rule::ne => BinaryOp::Ne,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            },
            rhs: Box::new(rhs),
        })
        .map_prefix(|op, rhs| Expr::UnaryOp {
            op: match op.as_rule() {
                Rule::minus => UnaryOp::Minus,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            },
            rhs: Box::new(rhs),
        })
        .parse(pairs)
}

/// Parses an assert statement.
///
/// ```rs
/// assert <expr> ;
/// ```
fn parse_assert_stmt(pair: Pair<Rule>) -> Stmt {
    debug_assert_eq!(pair.as_rule(), Rule::assert_stmt);
    let mut pairs = pair.into_inner();

    // assert <expr> ;
    //        ^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    debug_assert!(pairs.next().is_none());
    Stmt::Assert(expr)
}

/// Parses an expression statement.
///
/// ```rs
/// <expr> ;
/// ```
fn parse_expr_stmt(pair: Pair<Rule>) -> Stmt {
    debug_assert_eq!(pair.as_rule(), Rule::expr_stmt);
    let mut pairs = pair.into_inner();

    // expr ;
    // ^^^^
    let pair = pairs.next().unwrap();
    debug_assert_eq!(pair.as_rule(), Rule::expr);
    let expr = parse_expr(pair);

    debug_assert!(pairs.next().is_none());
    Stmt::Expr(expr)
}

/// Parses a let statement.
///
/// ```rs
/// let <identifier> = <expr> ;
/// ```
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

/// Parses a let statement with a polynomial term.
///
/// ```rs
/// let <identifier> ( <identifier> ) = <expr> ;
/// ```
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
