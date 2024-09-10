use crate::{errors::InterpreterError, utils::PolentaUtilExt};
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

    pub fn interpret(
        &mut self,
        input: &str,
    ) -> Result<Vec<Polynomial<FieldElement<F>>>, InterpreterError> {
        PolentaParser::parse_input(input)
            .unwrap()
            .into_iter()
            .map(|stmt| self.process_statement(stmt))
            .collect()
    }

    fn process_expr(
        &mut self,
        expr: Expr,
        term: Option<&String>,
    ) -> Result<Polynomial<FieldElement<F>>, InterpreterError> {
        match expr {
            Expr::Identifier(identifier) => {
                // if this identifier is a term, treat it as P(x) = x
                if term
                    .and_then(|t| if t == &identifier { Some(t) } else { None })
                    .is_some()
                {
                    Ok(Polynomial::new_monomial(FieldElement::one(), 1))
                } else {
                    // otherwise, look up the identifier in the symbol table
                    let value = self.symbols.get(&identifier).cloned();

                    match value {
                        Some(value) => Ok(value),
                        None => Err(InterpreterError::UnknownIdentifier(identifier).into()),
                    }
                }
            }
            Expr::Integer(value) => Ok(Polynomial::new_monomial(FieldElement::from(value), 0)),
            Expr::UnaryOp { op, rhs } => match op {
                UnaryOp::Minus => Ok(-self.process_expr(*rhs, term)?),
            },
            Expr::BinaryOp { lhs, op, rhs } => {
                let lhs = self.process_expr(*lhs, term)?;
                let rhs = self.process_expr(*rhs, term)?;

                match op {
                    BinaryOp::Add => Ok(lhs + rhs),
                    BinaryOp::Sub => Ok(lhs - rhs),
                    BinaryOp::Mul => Ok(lhs * rhs),
                    BinaryOp::Div => {
                        if rhs.coeff_len() == 0 {
                            Err(InterpreterError::DivisionByZero)
                        } else {
                            Ok(lhs / rhs)
                        }
                    }

                    BinaryOp::Mod => Ok(lhs.long_division_with_remainder(&rhs).1),
                    BinaryOp::Pow => Ok(Self::poly_pow(&lhs, Self::poly_as_felt(&rhs))),
                    BinaryOp::Evl => {
                        Ok(Self::felt_as_poly(lhs.evaluate(&Self::poly_as_felt(&rhs))))
                    }
                }
            }
        }
    }

    /// TODO: !!!
    ///
    /// The value of last evaluated "expression statement" is stored at `!!` symbol for internal testing.
    fn process_statement(
        &mut self,
        stmt: Stmt,
    ) -> Result<Polynomial<FieldElement<F>>, InterpreterError> {
        // TODO: can we avoid the cloning here?
        match stmt {
            Stmt::Let(identifier, expr) => {
                let poly = self.process_expr(expr, None)?;
                self.symbols.insert(identifier, poly.clone());
                Ok(poly)
            }
            Stmt::LetPoly(identifier, term, expr) => {
                let poly = self.process_expr(expr, Some(&term))?;
                self.symbols.insert(identifier, poly.clone());
                Ok(poly)
            }
            Stmt::Expr(expr) => {
                let poly = self.process_expr(expr, None)?;
                self.symbols.insert("!!".to_string(), poly.clone());
                Ok(poly)
            }
            Stmt::Assert(l_expr, r_expr) => {
                let l_poly = self.process_expr(l_expr, None)?;
                let r_poly = self.process_expr(r_expr, None)?;
                if l_poly == r_poly {
                    Ok(Polynomial::new_monomial(FieldElement::<F>::one(), 0))
                } else {
                    Ok(Polynomial::zero())
                }
                // self.symbols.insert("!!".to_string(), poly.clone());
            }
        }
    }
}
