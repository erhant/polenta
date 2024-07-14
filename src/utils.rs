use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    polynomial::Polynomial,
};

/// Treats the given constants polynomial as a field element.
pub fn poly_as_felt<F: IsField>(poly: Polynomial<FieldElement<F>>) -> FieldElement<F> {
    assert!(poly.coeff_len() == 1, "Expected a constant polynomial."); // TODO: return error
    poly.leading_coefficient()
}

/// Treats the given field element as a constant polynomial.
pub fn felt_as_poly<F: IsField>(felt: FieldElement<F>) -> Polynomial<FieldElement<F>> {
    Polynomial::new_monomial(felt, 0)
}

pub fn poly_print<F: IsPrimeField>(poly: Polynomial<FieldElement<F>>) -> String {
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

pub fn poly_pow<F: IsField>(
    poly: Polynomial<FieldElement<F>>,
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
