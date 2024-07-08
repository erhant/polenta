use std::collections::HashMap;

use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};

use crate::grammar::{Expr, Op};

pub struct Polenta<F: IsPrimeField> {
    symbols: HashMap<String, Polynomial<FieldElement<F>>>,
}

impl<F: IsPrimeField> Polenta<F> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn process_expression(&mut self, expr: Expr) -> Polynomial<FieldElement<F>> {
        match expr {
            Expr::Identifier(identifier) => self
                .symbols
                .get(&identifier)
                .expect("symbol not found")
                .clone(),
            Expr::Let(identifier, expr) => {
                let poly = self.process_expression(*expr);
                self.symbols.insert(identifier, poly.clone());
                poly
            }
            Expr::Integer(value) => Polynomial::new_monomial(FieldElement::from(value), 0),
            Expr::UnaryMinus(expr) => -self.process_expression(*expr),
            Expr::BinOp { lhs, op, rhs } => {
                let lhs = self.process_expression(*lhs);
                let rhs = self.process_expression(*rhs);

                match op {
                    Op::Add => lhs + rhs,
                    Op::Subtract => lhs - rhs,
                    Op::Multiply => lhs * rhs,
                    Op::Divide => lhs / rhs,
                    Op::Modulo => lhs.long_division_with_remainder(&rhs).1,
                    Op::Power => {
                        assert_eq!(rhs.degree(), 0);
                        lhs.scale(&rhs.coefficients[0]) // TODO: is this correct?
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{parse_let_expr, PolentaParser, Rule};
    use pest::Parser;

    type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    fn parse(expression: &str) -> Result<(), String> {
        let mut pairs = PolentaParser::parse(Rule::polenta, expression)
            .map_err(|e| format!("Parse failed: {:?}", e))?;

        let mut polenta = Polenta::<F>::new();
        for pair in pairs.next().unwrap().into_inner() {
            let poly = polenta.process_expression(parse_let_expr(pair));
            println!("Parsed polynomial: {:?}", poly);
        }

        Ok(())
    }

    #[test]
    fn test_1() {
        parse("4 * 3").expect("should parse");
    }
}
