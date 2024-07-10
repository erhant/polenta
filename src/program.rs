use core::panic;
use std::collections::HashMap;

use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};

use crate::grammar::{BinaryOp, Expr, Stmt, UnaryOp};

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
                    Polynomial::new_monomial(FieldElement::from(1), 1) // x
                } else {
                    // otherwise, look up the identifier in the symbol table
                    self.symbols
                        .get(&identifier)
                        .cloned()
                        .unwrap_or_else(|| panic!("Unknown identifier: {}", identifier))
                }
            }
            Expr::Eval(identifier, at) => {
                let poly = self.symbols.get(&identifier).unwrap();
                if let Expr::Integer(at) = *at {
                    let evaluation = poly.evaluate(&FieldElement::from(at));
                    Polynomial::new_monomial(evaluation, 0)
                } else {
                    panic!("Expected integer in evaluation");
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
                    BinaryOp::Power => {
                        todo!("power todo")
                    }
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
                todo!("maybe we make a print statement?");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{parse_polenta, PolentaParser, Rule};
    use pest::Parser;

    type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    fn parse(expression: &str) -> Result<(), String> {
        let pairs = PolentaParser::parse(Rule::polenta, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;
        let mut polenta = Polenta::<F>::new();

        for pair in pairs {
            let stmts = parse_polenta(pair);
            for stmt in stmts {
                polenta.process_statement(stmt);
            }
        }
        println!("{:?}", polenta.symbols);

        Ok(())
    }

    #[test]
    fn test_let_many() {
        parse("let x = 4 * 3; let y = 12 + x;").expect("should parse");
    }

    #[test]
    fn test_poly_eval() {
        parse("let P(x) = 4*x + 2; let a = 5 + P(2);").expect("should parse");
    }
}

// "let P(x) = 4*x + 2; let a = P(2);"
