use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    polynomial::Polynomial,
};

use crate::{errors::InterpreterError, interpreter::Polenta};

/// Several utilities related to polynomials and field elements used within Polenta.
pub trait PolentaUtilExt<F: IsField> {
    /// Treats the given constant polynomial as a field element.
    /// Returns an error if the polynomial has non-zero degree.
    fn poly_as_felt(
        poly: &Polynomial<FieldElement<F>>,
    ) -> Result<FieldElement<F>, InterpreterError>;

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
    fn poly_as_felt(
        poly: &Polynomial<FieldElement<F>>,
    ) -> Result<FieldElement<F>, InterpreterError> {
        // zero poly has len 0, and constant polys have len 1
        if poly.coeff_len() > 1 {
            return Err(InterpreterError::ExpectedConstant);
        }
        Ok(poly.leading_coefficient())
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
            .map(|coeff| repr_to_decimal(&format!("{}", coeff.representative())))
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
        exponent: FieldElement<F>,
    ) -> Polynomial<FieldElement<F>> {
        let mut exp = exponent.representative();
        let zero = F::RepresentativeType::from(0u16);
        let one_bit = F::RepresentativeType::from(1u16);

        let mut result = Polynomial::new_monomial(FieldElement::one(), 0);
        let mut base = poly.clone();

        while exp > zero {
            if (exp & one_bit) == one_bit {
                result = result * base.clone();
            }
            exp = exp >> 1usize;
            if exp > zero {
                base = base.clone() * base;
            }
        }

        result
    }
}

/// Converts a representative's string to decimal.
/// Passes through strings that are already decimal (e.g. from u64 Display),
/// and converts hex-prefixed strings (e.g. from UnsignedInteger Display).
fn repr_to_decimal(s: &str) -> String {
    let hex = match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(h) => h,
        None => return s.to_string(),
    };

    // Build decimal digits (least-significant first) via multiply-and-add
    let mut digits: Vec<u32> = vec![0];
    for c in hex.chars() {
        let nibble = c.to_digit(16).expect("invalid hex digit");
        let mut carry = 0u32;
        for d in digits.iter_mut() {
            let val = *d * 16 + carry;
            *d = val % 10;
            carry = val / 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }

        carry = nibble;
        for d in digits.iter_mut() {
            if carry == 0 {
                break;
            }
            let val = *d + carry;
            *d = val % 10;
            carry = val / 10;
        }
        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    let s: String = digits
        .iter()
        .rev()
        .map(|d| char::from_digit(*d, 10).unwrap())
        .collect();
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
