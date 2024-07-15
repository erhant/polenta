use crate::utils::PolentaUtilExt;
use core::panic;
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};
use std::collections::HashMap;

use crate::grammar::{BinaryOp, Expr, PolentaParser, Stmt, UnaryOp};

pub struct Polenta<F: IsPrimeField> {
    /// Symbol table as a map from identifiers to polynomials.
    /// Constant values are stored as constant polynomials.
    pub symbols: HashMap<String, Polynomial<FieldElement<F>>>,
}

impl<F: IsPrimeField> Default for Polenta<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: IsPrimeField> Polenta<F> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, input: &str) -> Vec<Polynomial<FieldElement<F>>> {
        PolentaParser::parse_input(input)
            .unwrap()
            .into_iter()
            .map(|stmt| self.process_statement(stmt))
            .collect()
    }

    fn process_expr(&mut self, expr: Expr, term: Option<&String>) -> Polynomial<FieldElement<F>> {
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
                    BinaryOp::Sub => lhs - rhs,
                    BinaryOp::Mul => lhs * rhs,
                    BinaryOp::Div => {
                        // TODO: error handling
                        if rhs.coeff_len() == 0 {
                            panic!("division by zero")
                        }
                        lhs / rhs
                    }
                    BinaryOp::Mod => lhs.long_division_with_remainder(&rhs).1,
                    BinaryOp::Pow => Self::poly_pow(&lhs, Self::poly_as_felt(&rhs)),
                    BinaryOp::Evl => Self::felt_as_poly(lhs.evaluate(&Self::poly_as_felt(&rhs))),
                }
            }
        }
    }

    fn process_statement(&mut self, stmt: Stmt) -> Polynomial<FieldElement<F>> {
        // TODO: can we avoid the cloning here?
        match stmt {
            Stmt::Let(identifier, expr) => {
                let poly = self.process_expr(expr, None);
                self.symbols.insert(identifier, poly.clone());
                poly
            }
            Stmt::LetPoly(identifier, term, expr) => {
                let poly = self.process_expr(expr, Some(&term));
                self.symbols.insert(identifier, poly.clone());
                poly
            }
            Stmt::Expr(expr) => {
                let poly = self.process_expr(expr, None);
                self.symbols.insert("!!".to_string(), poly.clone());
                poly
            }
        }
    }
}
