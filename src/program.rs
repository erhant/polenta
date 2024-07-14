use core::panic;
use std::collections::HashMap;

use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};

use crate::{
    grammar::{BinaryOp, Expr, Stmt, UnaryOp},
    utils::{felt_as_poly, poly_as_felt, poly_pow},
};

pub struct Polenta<F: IsPrimeField> {
    /// Symbol table as a map from identifiers to polynomials.
    /// Constant values are stored as constant polynomials.
    symbols: HashMap<String, Polynomial<FieldElement<F>>>,
}

impl<F: IsPrimeField> Polenta<F> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn process_expr(
        &mut self,
        expr: Expr,
        term: Option<&String>,
    ) -> Polynomial<FieldElement<F>> {
        match expr {
            Expr::Identifier(identifier) => {
                // if this identifier is a term, treat it as P(x) = x
                if term
                    .and_then(|t| if t == &identifier { Some(t) } else { None })
                    .is_some()
                {
                    Polynomial::new_monomial(FieldElement::from(1), 1)
                } else {
                    // otherwise, look up the identifier in the symbol table
                    self.symbols
                        .get(&identifier)
                        .cloned()
                        .unwrap_or_else(|| panic!("Unknown identifier: {}", identifier))
                }
            }
            Expr::Integer(value) => Polynomial::new_monomial(FieldElement::from(value), 0),
            Expr::UnaryOp { op, rhs } => match op {
                UnaryOp::Minus => -self.process_expr(*rhs, term),
            },
            Expr::BinaryOp { lhs, op, rhs } => {
                let lhs = self.process_expr(*lhs, term);
                let rhs = self.process_expr(*rhs, term);

                match op {
                    BinaryOp::Add => lhs + rhs,
                    BinaryOp::Subtract => lhs - rhs,
                    BinaryOp::Multiply => lhs * rhs,
                    BinaryOp::Divide => lhs / rhs,
                    BinaryOp::Modulo => lhs.long_division_with_remainder(&rhs).1,
                    BinaryOp::Eval => felt_as_poly(lhs.evaluate(&poly_as_felt(rhs))),
                    BinaryOp::Power => poly_pow(lhs, poly_as_felt(rhs)),
                }
            }
        }
    }

    pub fn process_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Let(identifier, expr) => {
                let poly = self.process_expr(expr, None);
                self.symbols.insert(identifier, poly);
            }
            Stmt::LetPoly(identifier, term, expr) => {
                let poly = self.process_expr(expr, Some(&term));
                self.symbols.insert(identifier, poly);
            }
            Stmt::Expr(expr) => {
                let poly = self.process_expr(expr, None);
                todo!("maybe we make a print statement?");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grammar::PolentaParser, utils::poly_print};

    type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    fn run_test(expression: &str) {
        let mut polenta = Polenta::<F>::new();
        let stmts = PolentaParser::parse_input(expression).unwrap();
        for stmt in stmts {
            polenta.process_statement(stmt);
        }

        for (symbol, value) in polenta.symbols.iter() {
            println!("{} = {}", symbol, poly_print(value.clone()));
        }
    }

    #[test]
    fn test_let_many() {
        run_test("let x = 4 * 3; let y = 12 + x;");
    }

    #[test]
    fn test_poly_powers() {
        run_test("let P(x) = x^3 + 3*x^2 + x + 4;");
    }

    #[test]
    fn test_dummy_eval() {
        run_test("let a = 2@7;");
    }

    #[test]
    fn test_many_powers() {
        run_test("let P(x) = 3*x^2^3;");
    }

    #[test]
    fn test_poly_eval() {
        let input = r#"
        let P(x) = 4*x + 2; 
        let t = 2;
        let a = 5 + P@(2);
        "#;
        run_test(input);
    }
}

// "let P(x) = 4*x + 2; let a = P(2);"
