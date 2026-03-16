use lambdaworks_math::field::fields::fft_friendly::{
    babybear::Babybear31PrimeField, stark_252_prime_field::Stark252PrimeField,
    u64_goldilocks::U64GoldilocksPrimeField,
    u64_mersenne_montgomery_field::Mersenne31MontgomeryPrimeField,
};

use crate::{
    fields::{export_symbols, import_symbols, FieldType},
    interpreter::Polenta,
    utils::PolentaUtilExt,
    PolentaError,
};

/// A type-erased Polenta interpreter that can switch between concrete field types at runtime.
pub enum PolentaInstance {
    Babybear31(Polenta<Babybear31PrimeField>),
    Goldilocks(Polenta<U64GoldilocksPrimeField>),
    Stark252(Polenta<Stark252PrimeField>),
    Mersenne31(Polenta<Mersenne31MontgomeryPrimeField>),
}

impl PolentaInstance {
    pub fn new(field_type: FieldType) -> Self {
        match field_type {
            FieldType::Babybear31 => PolentaInstance::Babybear31(Polenta::new()),
            FieldType::Goldilocks => PolentaInstance::Goldilocks(Polenta::new()),
            FieldType::Stark252 => PolentaInstance::Stark252(Polenta::new()),
            FieldType::Mersenne31 => PolentaInstance::Mersenne31(Polenta::new()),
        }
    }

    pub fn field_type(&self) -> FieldType {
        match self {
            PolentaInstance::Babybear31(_) => FieldType::Babybear31,
            PolentaInstance::Goldilocks(_) => FieldType::Goldilocks,
            PolentaInstance::Stark252(_) => FieldType::Stark252,
            PolentaInstance::Mersenne31(_) => FieldType::Mersenne31,
        }
    }

    pub fn interpret(&mut self, input: &str) -> Result<String, PolentaError> {
        match self {
            PolentaInstance::Babybear31(p) => {
                let polys = p.interpret(input)?;
                Ok(Polenta::<Babybear31PrimeField>::poly_print(
                    polys.last().unwrap(),
                ))
            }
            PolentaInstance::Goldilocks(p) => {
                let polys = p.interpret(input)?;
                Ok(Polenta::<U64GoldilocksPrimeField>::poly_print(
                    polys.last().unwrap(),
                ))
            }
            PolentaInstance::Stark252(p) => {
                let polys = p.interpret(input)?;
                Ok(Polenta::<Stark252PrimeField>::poly_print(
                    polys.last().unwrap(),
                ))
            }
            PolentaInstance::Mersenne31(p) => {
                let polys = p.interpret(input)?;
                Ok(Polenta::<Mersenne31MontgomeryPrimeField>::poly_print(
                    polys.last().unwrap(),
                ))
            }
        }
    }

    /// Migrate symbols from another instance by converting coefficients via hex.
    /// `from_hex` on the target field auto-reduces mod p, so this works for any field pair.
    pub fn migrate_symbols_from(&mut self, other: &PolentaInstance) {
        let exported = match other {
            PolentaInstance::Babybear31(p) => export_symbols(&p.symbols),
            PolentaInstance::Goldilocks(p) => export_symbols(&p.symbols),
            PolentaInstance::Stark252(p) => export_symbols(&p.symbols),
            PolentaInstance::Mersenne31(p) => export_symbols(&p.symbols),
        };
        match self {
            PolentaInstance::Babybear31(p) => import_symbols(&mut p.symbols, &exported),
            PolentaInstance::Goldilocks(p) => import_symbols(&mut p.symbols, &exported),
            PolentaInstance::Stark252(p) => import_symbols(&mut p.symbols, &exported),
            PolentaInstance::Mersenne31(p) => import_symbols(&mut p.symbols, &exported),
        }
    }
}
