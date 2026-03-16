use lambdaworks_math::{
    field::{element::FieldElement, traits::IsPrimeField},
    polynomial::Polynomial,
};
use std::collections::HashMap;

/// Supported prime fields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldType {
    Babybear31,
    Goldilocks,
    Stark252,
    Mersenne31,
}

impl FieldType {
    pub fn name(&self) -> &'static str {
        match self {
            FieldType::Babybear31 => "babybear31",
            FieldType::Goldilocks => "goldilocks",
            FieldType::Stark252 => "stark252",
            FieldType::Mersenne31 => "mersenne31",
        }
    }

    pub fn order(&self) -> &'static str {
        match self {
            FieldType::Babybear31 => "2013265921",
            FieldType::Goldilocks => "18446744069414584321",
            FieldType::Stark252 => {
                "3618502788666131213697322783095070105623107215331596699973092056135872020481"
            }
            FieldType::Mersenne31 => "2147483647",
        }
    }

    pub fn all() -> &'static [FieldType] {
        &[
            FieldType::Babybear31,
            FieldType::Goldilocks,
            FieldType::Stark252,
            FieldType::Mersenne31,
        ]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "babybear31" => Some(FieldType::Babybear31),
            "goldilocks" => Some(FieldType::Goldilocks),
            "stark252" => Some(FieldType::Stark252),
            "mersenne31" => Some(FieldType::Mersenne31),
            _ => None,
        }
    }
}

/// Export symbols as (name, hex coefficients) pairs.
pub fn export_symbols<F: IsPrimeField>(
    symbols: &HashMap<String, Polynomial<FieldElement<F>>>,
) -> Vec<(String, Vec<String>)> {
    symbols
        .iter()
        .map(|(name, poly)| {
            let hex_coeffs = poly
                .coefficients()
                .iter()
                .map(|c| format!("{}", c.representative()))
                .collect();
            (name.clone(), hex_coeffs)
        })
        .collect()
}

/// Import symbols from hex coefficients, reducing mod p automatically via `from_hex`.
pub fn import_symbols<F: IsPrimeField>(
    symbols: &mut HashMap<String, Polynomial<FieldElement<F>>>,
    data: &[(String, Vec<String>)],
) {
    for (name, hex_coeffs) in data {
        let coeffs: Vec<FieldElement<F>> = hex_coeffs
            .iter()
            .map(|h| FieldElement::<F>::from_hex(h).expect("coefficient conversion failed"))
            .collect();
        symbols.insert(name.clone(), Polynomial::new(&coeffs));
    }
}
