use crate::{
    errors::{pest_error_to_miette_error, InterpreterError, PolentaError},
    utils::PolentaUtilExt,
};
use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};
use std::collections::HashMap;

use crate::grammar::{BinaryOp, Expr, PolentaParser, Stmt, UnaryOp};

/// Polenta interpreter.
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

    /// Interprets the given input string and returns the resulting polynomials.
    ///
    /// The input is expected to be composed of several statements, each interpreted in the given
    /// order and resulting in a polynomial.
    ///
    /// May throw out a `PolentaError` if an error occurs during interpretation, either within the
    /// parsing step or the interpretation step.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let input = r#"
    /// let P(x) = 3 * x + 1;
    /// let Q(x) = x / 2;
    /// let z = Q@P@(5);
    /// assert z == 8;
    /// "#;
    ///
    /// Polenta::<F>::new().interpret(input)?;
    /// ```
    pub fn interpret(
        &mut self,
        input: &str,
    ) -> Result<Vec<Polynomial<FieldElement<F>>>, PolentaError> {
        let stmts = PolentaParser::parse_input(input.trim()).map_err(pest_error_to_miette_error)?;

        stmts
            .into_iter()
            .map(|stmt| self.process_statement(stmt).map_err(|e| e.into()))
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
                    // arithmetic operations
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
                    // comparison operations
                    BinaryOp::Eq => Ok(Self::poly_from_bool(lhs == rhs)),
                    BinaryOp::Ne => Ok(Self::poly_from_bool(lhs != rhs)),
                    // evaluation
                    BinaryOp::Evl => {
                        Ok(Self::felt_as_poly(lhs.evaluate(&Self::poly_as_felt(&rhs))))
                    }
                }
            }
        }
    }

    /// The value of last evaluated "expression statement" is stored at `!!` symbol for internal testing.
    fn process_statement(
        &mut self,
        stmt: Stmt,
    ) -> Result<Polynomial<FieldElement<F>>, InterpreterError> {
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
            Stmt::Assert(expr) => {
                let result = self.process_expr(expr, None)?;
                // fail if the result is zero, which means the assertion is false
                // otherwise, return the result as is
                if Self::poly_is_zero(&result) {
                    Err(InterpreterError::AssertionFailed)
                } else {
                    Ok(result)
                }
            }
        }
    }
}
