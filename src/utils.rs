use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    polynomial::Polynomial,
};

use crate::program::Polenta;

/// Several utilities related to polynomial and field elements used within Polenta.
pub trait PolentaUtilExt<F: IsField> {
    fn poly_as_felt(poly: &Polynomial<FieldElement<F>>) -> FieldElement<F>;
    fn felt_as_poly(felt: FieldElement<F>) -> Polynomial<FieldElement<F>>;
    fn poly_print(poly: &Polynomial<FieldElement<F>>) -> String;
    fn poly_pow(
        poly: &Polynomial<FieldElement<F>>,
        exponent: FieldElement<F>,
    ) -> Polynomial<FieldElement<F>>;
    fn poly_is_zero(poly: &Polynomial<FieldElement<F>>) -> bool;
    fn poly_from_bool(b: bool) -> Polynomial<FieldElement<F>>;
}

impl<F: IsPrimeField> PolentaUtilExt<F> for Polenta<F> {
    /// Treats the given constants polynomial as a field element.
    fn poly_as_felt(poly: &Polynomial<FieldElement<F>>) -> FieldElement<F> {
        // zero poly has len 0, and constant polys have len 1
        assert!(poly.coeff_len() <= 1, "Expected a constant polynomial."); // TODO: return error
        poly.leading_coefficient()
    }

    /// Returns true if the given polynomial is zero polynomial.
    fn poly_is_zero(poly: &Polynomial<FieldElement<F>>) -> bool {
        poly.coeff_len() == 0
    }

    /// Returns a polynomial representing the given boolean value, i.e. `1` for `true` and `0` for `false`.
    fn poly_from_bool(b: bool) -> Polynomial<FieldElement<F>> {
        if b {
            Polynomial::new_monomial(FieldElement::one(), 0)
        } else {
            Polynomial::zero()
        }
    }

    /// Treats the given field element as a constant polynomial.
    fn felt_as_poly(felt: FieldElement<F>) -> Polynomial<FieldElement<F>> {
        Polynomial::new_monomial(felt, 0)
    }

    /// Pretty-prints a given polynomial.
    fn poly_print(poly: &Polynomial<FieldElement<F>>) -> String {
        let coeff_decimals = poly
            .coefficients()
            .iter()
            .map(|coeff| format!("{}", coeff.representative()))
            .collect::<Vec<_>>();

        let result = coeff_decimals
            .iter()
            .enumerate()
            .rev()
            .map(|(i, coeff)| match (i, coeff.as_str()) {
                (_, "0") => String::new(),
                (0, _) => format!("{}", coeff),
                (1, "1") => "x".to_string(),
                (1, _) => format!("{}*x", coeff),
                (_, "1") => format!("x^{}", i),
                (_, _) => format!("{}*x^{}", coeff, i),
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" + ");

        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }

    /// Multiplies a polynomial with itself many times.
    ///
    /// TODO: do the efficient square-and-multiply method
    fn poly_pow(
        poly: &Polynomial<FieldElement<F>>,
        mut exponent: FieldElement<F>,
    ) -> Polynomial<FieldElement<F>> {
        let (one, zero) = (FieldElement::from(1), FieldElement::from(0));
        let mut result = Polynomial::new_monomial(one.clone(), 0); // 1
        while exponent != zero {
            result = result * poly.clone();
            exponent = exponent - one.clone();
        }

        result
    }
}
