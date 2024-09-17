use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    polynomial::Polynomial,
};

use crate::program::Polenta;

/// Several utilities related to polynomials and field elements used within Polenta.
pub trait PolentaUtilExt<F: IsField> {
    /// Treats the given constants polynomial as a field element.
    fn poly_as_felt(poly: &Polynomial<FieldElement<F>>) -> FieldElement<F>;

    /// Treats the given field element as a constant polynomial.
    fn felt_as_poly(felt: FieldElement<F>) -> Polynomial<FieldElement<F>>;

    /// Pretty-prints a given polynomial.
    fn poly_print(poly: &Polynomial<FieldElement<F>>) -> String;

    /// Multiplies a polynomial with itself many times.
    fn poly_pow(
        poly: &Polynomial<FieldElement<F>>,
        exponent: FieldElement<F>,
    ) -> Polynomial<FieldElement<F>>;

    /// Returns true if the given polynomial is a zero polynomial.
    fn poly_is_zero(poly: &Polynomial<FieldElement<F>>) -> bool;

    /// Returns a polynomial representing the given boolean value, i.e. `1` for `true` and `0` for `false`.
    fn poly_from_bool(b: bool) -> Polynomial<FieldElement<F>>;
}

impl<F: IsPrimeField> PolentaUtilExt<F> for Polenta<F> {
    fn poly_as_felt(poly: &Polynomial<FieldElement<F>>) -> FieldElement<F> {
        // zero poly has len 0, and constant polys have len 1
        assert!(poly.coeff_len() <= 1, "Expected a constant polynomial."); // TODO: return error
        poly.leading_coefficient()
    }

    fn poly_is_zero(poly: &Polynomial<FieldElement<F>>) -> bool {
        poly.coeff_len() == 0
    }

    fn poly_from_bool(b: bool) -> Polynomial<FieldElement<F>> {
        if b {
            Polynomial::new_monomial(FieldElement::one(), 0)
        } else {
            Polynomial::zero()
        }
    }

    fn felt_as_poly(felt: FieldElement<F>) -> Polynomial<FieldElement<F>> {
        Polynomial::new_monomial(felt, 0)
    }

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
                (0, ___) => format!("{}", coeff),
                (1, "1") => "x".to_string(),
                (1, ___) => format!("{}*x", coeff),
                (_, "1") => format!("x^{}", i),
                (_, ___) => format!("{}*x^{}", coeff, i),
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

    fn poly_pow(
        poly: &Polynomial<FieldElement<F>>,
        mut exponent: FieldElement<F>,
    ) -> Polynomial<FieldElement<F>> {
        // TODO: do the efficient square-and-multiply method
        let (one, zero) = (FieldElement::from(1), FieldElement::from(0));
        let mut result = Polynomial::new_monomial(one.clone(), 0); // 1
        while exponent != zero {
            result = result * poly.clone();
            exponent = exponent - one.clone();
        }

        result
    }
}
