use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::fft_friendly::{
            babybear::Babybear31PrimeField, stark_252_prime_field::Stark252PrimeField,
            u64_goldilocks::U64GoldilocksPrimeField,
            u64_mersenne_montgomery_field::Mersenne31MontgomeryPrimeField,
        },
        fields::pallas_field::Pallas255PrimeField,
        traits::IsPrimeField,
    },
    polynomial::Polynomial,
};
use std::collections::HashMap;

use crate::{
    interpreter::Polenta,
    utils::{repr_to_decimal, repr_to_hex, PolentaUtilExt},
    PolentaError,
};

/// Field-agnostic surface of a Polenta interpreter.
///
/// Every method returns types that do NOT mention `F`, so the trait is
/// object-safe and we can hold any field behind `Box<dyn IsPolentaInstance>`.
pub trait IsPolentaInstance {
    /// Modulus of the underlying prime field, as a decimal string.
    fn modulus(&self) -> String;

    /// Interpret the given input and return the result as a string.
    fn interpret(&mut self, input: &str) -> Result<String, PolentaError>;

    /// Export symbol table as a vector of (identifier, polynomial) pairs, where the polynomial is represented as a vector of hex coefficient strings.
    fn export(&self) -> Vec<(String, Vec<String>)>;

    /// Import symbol table from a vector of (identifier, polynomial) pairs, where the polynomial is represented as a vector of hex coefficient strings.
    fn import(&mut self, data: &[(String, Vec<String>)]);
}

impl<F: IsPrimeField + 'static> IsPolentaInstance for Polenta<F> {
    fn modulus(&self) -> String {
        let one = <F::RepresentativeType as From<u16>>::from(1u16);
        let m = F::modulus_minus_one() + one;
        repr_to_decimal(&format!("{}", m))
    }

    fn interpret(&mut self, input: &str) -> Result<String, PolentaError> {
        let polys = Polenta::<F>::interpret(self, input)?;
        Ok(Polenta::<F>::poly_print(polys.last().unwrap()))
    }

    fn export(&self) -> Vec<(String, Vec<String>)> {
        export_symbols(&self.symbols)
    }

    fn import(&mut self, data: &[(String, Vec<String>)]) {
        import_symbols(&mut self.symbols, data);
    }
}

/// [Dyn-dispatched](https://doc.rust-lang.org/std/keyword.dyn.html) Polenta instance,
/// tagged with its field's catalog name.
pub struct PolentaInstance {
    inner: Box<dyn IsPolentaInstance>,
    name: &'static str,
}

impl PolentaInstance {
    /// Catalog of supported fields, keyed by name.
    ///
    /// To add a new field, simply provide one more name & factory.
    #[rustfmt::skip]
    pub const fn supported_fields() -> &'static [(&'static str, fn() -> Box<dyn IsPolentaInstance>)] {
      &[
          // top one is treated as the default field
          ("babybear31", || {Box::new(Polenta::<Babybear31PrimeField>::new())}),
          ("goldilocks", || {Box::new(Polenta::<U64GoldilocksPrimeField>::new())}),
          ("stark252",   || {Box::new(Polenta::<Stark252PrimeField>::new())}),
          ("mersenne31", || {Box::new(Polenta::<Mersenne31MontgomeryPrimeField>::new())}),
          ("pallas255",  || {Box::new(Polenta::<Pallas255PrimeField>::new())}),
      ]
    }

    /// Names of all supported fields, in catalog order.
    pub fn supported_field_names() -> Vec<&'static str> {
        Self::supported_fields().iter().map(|(n, _)| *n).collect()
    }

    /// Construct an instance by name (case-insensitive).
    pub fn new_from_name(name: &str) -> Option<PolentaInstance> {
        let needle = name.trim().to_lowercase();
        Self::supported_fields()
            .iter()
            .find(|(n, _)| *n == needle)
            .map(|(n, f)| PolentaInstance {
                inner: f(),
                name: n,
            })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn modulus(&self) -> String {
        self.inner.modulus()
    }

    pub fn interpret(&mut self, input: &str) -> Result<String, PolentaError> {
        self.inner.interpret(input)
    }

    pub fn migrate_symbols_from(&mut self, other: &PolentaInstance) {
        let data = other.inner.export();
        self.inner.import(&data);
    }
}

impl Default for PolentaInstance {
    fn default() -> Self {
        let (name, factory) = Self::supported_fields()[0];
        Self {
            inner: factory(),
            name,
        }
    }
}

/// Export symbols as (name, hex coefficient strings) pairs.
/// Always normalizes to unprefixed hex so `from_hex` can parse on import.
pub fn export_symbols<F: IsPrimeField>(
    symbols: &HashMap<String, Polynomial<FieldElement<F>>>,
) -> Vec<(String, Vec<String>)> {
    symbols
        .iter()
        .map(|(name, poly)| {
            let hex_coeffs = poly
                .coefficients()
                .iter()
                .map(|c| repr_to_hex(&format!("{}", c.representative())))
                .collect();
            (name.clone(), hex_coeffs)
        })
        .collect()
}

/// Import symbols from hex coefficient strings, reducing mod p automatically via `from_hex`.
pub fn import_symbols<F: IsPrimeField>(
    symbols: &mut HashMap<String, Polynomial<FieldElement<F>>>,
    data: &[(String, Vec<String>)],
) {
    for (name, hex_coeffs) in data {
        let coeffs: Vec<FieldElement<F>> = hex_coeffs
            .iter()
            .filter_map(|h| FieldElement::<F>::from_hex(h).ok())
            .collect();
        symbols.insert(name.clone(), Polynomial::new(&coeffs));
    }
}
