use crate::Polenta;

use lambdaworks_math::field::{fields, traits::IsPrimeField};
use miette::{IntoDiagnostic, Result};

/// A wrapper enum to represent various field instances of Polenta.
/// Use as follows:
///
/// ```rs
/// let polenta = PolentaFields::Goldilocks(Polenta::new());
/// ```
pub enum PolentaFields {
    Goldilocks(Polenta<fields::u64_goldilocks_field::Goldilocks64Field>),
    Mersenne31(Polenta<fields::mersenne31::field::Mersenne31Field>),
}

impl Default for PolentaFields {
    /// The default field is Goldilocks.
    fn default() -> Self {
        PolentaFields::Goldilocks(Polenta::new())
    }
}

impl PolentaFields {
    /// A wrapper around the `Polenta::reset` method.
    pub fn reset(&mut self) {
        match self {
            PolentaFields::Goldilocks(polenta) => polenta.reset(),
            PolentaFields::Mersenne31(polenta) => polenta.reset(),
        }
    }

    /// A wrapper around the `Polenta::interpret_readable` method, returns a string.
    pub fn interpret(&mut self, input: &str) -> Result<String> {
        match self {
            PolentaFields::Goldilocks(polenta) => polenta.interpret_readable(input),
            PolentaFields::Mersenne31(polenta) => polenta.interpret_readable(input),
        }
        .into_diagnostic()
    }
}
