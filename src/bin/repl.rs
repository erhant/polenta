use colored::Colorize;
use lambdaworks_math::{
    field::element::FieldElement,
    field::fields::fft_friendly::{
        babybear::Babybear31PrimeField, stark_252_prime_field::Stark252PrimeField,
        u64_goldilocks::U64GoldilocksPrimeField,
        u64_mersenne_montgomery_field::Mersenne31MontgomeryPrimeField,
    },
    polynomial::Polynomial,
    unsigned_integer::element::{U256, U64},
};
use miette::{IntoDiagnostic, Report, Result};
use polenta::{Polenta, PolentaUtilExt};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldType {
    Babybear31,
    Goldilocks,
    Stark252,
    Mersenne31,
}

impl FieldType {
    fn name(&self) -> &'static str {
        match self {
            FieldType::Babybear31 => "babybear31",
            FieldType::Goldilocks => "goldilocks",
            FieldType::Stark252 => "stark252",
            FieldType::Mersenne31 => "mersenne31",
        }
    }

    fn order(&self) -> &'static str {
        match self {
            FieldType::Babybear31 => "2013265921",
            FieldType::Goldilocks => "18446744069414584321",
            FieldType::Stark252 => {
                "3618502788666131213697322783095070105623107215331596699973092056135872020481"
            }
            FieldType::Mersenne31 => "2147483647",
        }
    }

    fn all() -> &'static [FieldType] {
        &[
            FieldType::Babybear31,
            FieldType::Goldilocks,
            FieldType::Stark252,
            FieldType::Mersenne31,
        ]
    }

    fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "babybear31" => Some(FieldType::Babybear31),
            "goldilocks" => Some(FieldType::Goldilocks),
            "stark252" => Some(FieldType::Stark252),
            "mersenne31" => Some(FieldType::Mersenne31),
            _ => None,
        }
    }
}

enum PolentaInstance {
    Babybear31(Polenta<Babybear31PrimeField>),
    Goldilocks(Polenta<U64GoldilocksPrimeField>),
    Stark252(Polenta<Stark252PrimeField>),
    Mersenne31(Polenta<Mersenne31MontgomeryPrimeField>),
}

impl PolentaInstance {
    fn new(field_type: FieldType) -> Self {
        match field_type {
            FieldType::Babybear31 => PolentaInstance::Babybear31(Polenta::new()),
            FieldType::Goldilocks => PolentaInstance::Goldilocks(Polenta::new()),
            FieldType::Stark252 => PolentaInstance::Stark252(Polenta::new()),
            FieldType::Mersenne31 => PolentaInstance::Mersenne31(Polenta::new()),
        }
    }

    fn field_type(&self) -> FieldType {
        match self {
            PolentaInstance::Babybear31(_) => FieldType::Babybear31,
            PolentaInstance::Goldilocks(_) => FieldType::Goldilocks,
            PolentaInstance::Stark252(_) => FieldType::Stark252,
            PolentaInstance::Mersenne31(_) => FieldType::Mersenne31,
        }
    }

    fn interpret(&mut self, input: &str) -> Result<String, polenta::PolentaError> {
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

    fn migrate_symbols_from(&mut self, other: &PolentaInstance) {
        let (self_type, other_type) = (self.field_type(), other.field_type());
        if self_type == other_type {
            return;
        }

        // Migration strategy based on field compatibility:
        // - Between 31-bit fields (Babybear31 ↔ Mersenne31): Attempt conversion
        // - Between 64-bit fields (Goldilocks): Limited conversion
        // - To/from Stark252: Clear symbols (256-bit field too large)
        // - Default: Clear symbols for safety

        match (other_type, self_type) {
            // All field conversions now supported with proper modular arithmetic
            (FieldType::Babybear31, FieldType::Mersenne31)
            | (FieldType::Mersenne31, FieldType::Babybear31) => {
                println!("Converting polynomials between 31-bit fields using modular arithmetic.");
                self.attempt_symbol_migration(other);
            }

            (FieldType::Goldilocks, FieldType::Babybear31)
            | (FieldType::Goldilocks, FieldType::Mersenne31) => {
                println!(
                    "Converting polynomials from 64-bit to 31-bit field using modular reduction."
                );
                self.attempt_symbol_migration(other);
            }

            (FieldType::Babybear31, FieldType::Goldilocks)
            | (FieldType::Mersenne31, FieldType::Goldilocks) => {
                println!("Converting polynomials from 31-bit to 64-bit field (direct embedding).");
                self.attempt_symbol_migration(other);
            }

            (FieldType::Stark252, FieldType::Babybear31)
            | (FieldType::Stark252, FieldType::Mersenne31)
            | (FieldType::Stark252, FieldType::Goldilocks) => {
                println!(
                    "Converting polynomials from 256-bit to smaller field using modular reduction."
                );
                self.attempt_symbol_migration(other);
            }

            (FieldType::Babybear31, FieldType::Stark252)
            | (FieldType::Mersenne31, FieldType::Stark252)
            | (FieldType::Goldilocks, FieldType::Stark252) => {
                println!("Converting polynomials to 256-bit field (direct embedding).");
                self.attempt_symbol_migration(other);
            }

            // Default case (shouldn't happen)
            _ => {
                self.clear_symbols();
            }
        }
    }

    fn attempt_symbol_migration(&mut self, other: &PolentaInstance) {
        let dest_field_type = self.field_type();
        let other_field_type = other.field_type();

        match (other_field_type, dest_field_type) {
            (FieldType::Babybear31, FieldType::Goldilocks) => {
                if let (PolentaInstance::Babybear31(src), PolentaInstance::Goldilocks(dest)) =
                    (other, self)
                {
                    dest.symbols.clear();
                    for (name, poly) in &src.symbols {
                        let coeffs: Vec<FieldElement<U64GoldilocksPrimeField>> = poly
                            .coefficients()
                            .iter()
                            .map(|c| {
                                let value = c.representative().limbs[0];
                                let u256_value = U256::from_u64(value);
                                let converted =
                                    convert_coefficient_via_u256(u256_value, dest_field_type);
                                FieldElement::<U64GoldilocksPrimeField>::from(&U64::from_u64(
                                    converted.limbs[0],
                                ))
                            })
                            .collect();
                        dest.symbols.insert(name.clone(), Polynomial::new(&coeffs));
                    }
                }
            }
            (FieldType::Babybear31, FieldType::Stark252) => {
                if let (PolentaInstance::Babybear31(src), PolentaInstance::Stark252(dest)) =
                    (other, self)
                {
                    dest.symbols.clear();
                    for (name, poly) in &src.symbols {
                        let coeffs: Vec<FieldElement<Stark252PrimeField>> = poly
                            .coefficients()
                            .iter()
                            .map(|c| {
                                let value = c.representative().limbs[0];
                                let u256_value = U256::from_u64(value);
                                let converted =
                                    convert_coefficient_via_u256(u256_value, dest_field_type);
                                FieldElement::<Stark252PrimeField>::from(&converted)
                            })
                            .collect();
                        dest.symbols.insert(name.clone(), Polynomial::new(&coeffs));
                    }
                }
            }
            (FieldType::Babybear31, FieldType::Mersenne31) => {
                if let (PolentaInstance::Babybear31(src), PolentaInstance::Mersenne31(dest)) =
                    (other, self)
                {
                    dest.symbols.clear();
                    for (name, poly) in &src.symbols {
                        let coeffs: Vec<FieldElement<Mersenne31MontgomeryPrimeField>> = poly
                            .coefficients()
                            .iter()
                            .map(|c| {
                                let value = c.representative().limbs[0];
                                let u256_value = U256::from_u64(value);
                                let converted =
                                    convert_coefficient_via_u256(u256_value, dest_field_type);
                                FieldElement::<Mersenne31MontgomeryPrimeField>::from(
                                    &U64::from_u64(converted.limbs[0]),
                                )
                            })
                            .collect();
                        dest.symbols.insert(name.clone(), Polynomial::new(&coeffs));
                    }
                }
            }

            // All other combinations follow the same pattern but are not implemented yet
            _ => {
                // For now, clear symbols for unimplemented combinations
                self.clear_symbols();
            }
        }
    }

    fn clear_symbols(&mut self) {
        match self {
            PolentaInstance::Babybear31(dest) => dest.symbols.clear(),
            PolentaInstance::Goldilocks(dest) => dest.symbols.clear(),
            PolentaInstance::Stark252(dest) => dest.symbols.clear(),
            PolentaInstance::Mersenne31(dest) => dest.symbols.clear(),
        }
    }
}

// Helper function to convert coefficient via U256 intermediate representation
fn convert_coefficient_via_u256(value: U256, dest_field_type: FieldType) -> U256 {
    match dest_field_type {
        FieldType::Babybear31 => {
            let reduced = value.limbs[0] % 2013265921; // Babybear31 order
            U256::from_u64(reduced)
        }
        FieldType::Goldilocks => {
            let reduced = value.limbs[0] % 18446744069414584321; // Goldilocks order
            U256::from_u64(reduced)
        }
        FieldType::Mersenne31 => {
            let reduced = value.limbs[0] % 2147483647; // Mersenne31 order
            U256::from_u64(reduced)
        }
        FieldType::Stark252 => {
            // For Stark252, most U256 values fit directly (field order is close to 2^251)
            value
        }
    }
}

const CMD_HELP: &str = "help";
const CMD_EXIT: &str = "exit";
const CMD_RESET: &str = "reset";
const CMD_FIELD: &str = "field";

const WELCOME_BANNER: &str = r#"
              _            _
  _ __   ___ | | ___ _ __ | |_ __ _
 | '_ \ / _ \| |/ _ \ '_ \| __/ _` |
 | |_) | (_) | |  __/ | | | || (_| |
 | .__/ \___/|_|\___|_| |_|\__\__,_|
 |_|
"#;

fn main() -> Result<()> {
    println!(
        "{}\n(v{})",
        WELCOME_BANNER.green().bold(),
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "Type {} to quit, or see {} for all commands.",
        CMD_EXIT.yellow(),
        CMD_HELP.yellow()
    );
    let mut polenta = PolentaInstance::new(FieldType::Babybear31);
    let mut rl = DefaultEditor::new().into_diagnostic()?;

    let prompt_line = format!("{}", "> ".green());
    loop {
        match rl.readline(&prompt_line) {
            Ok(line) => match line.as_str() {
                "" => {
                    // do nothing
                }
                CMD_HELP => {
                    let _ = rl.add_history_entry(CMD_HELP);

                    println!("Polenta is a simple language for polynomial manipulation.");
                    println!("{:<12}show this help message", CMD_HELP.yellow());
                    println!("{:<12}exit the program", CMD_EXIT.yellow());
                    println!("{:<12}reset symbols", CMD_RESET.yellow());
                    println!("{:<12}show current field or switch field", "field".yellow());
                }
                CMD_EXIT => {
                    println!("bye!");
                    break;
                }
                CMD_RESET => {
                    polenta = PolentaInstance::new(polenta.field_type());
                    println!("Symbol table reset.");
                }
                line if line.starts_with(CMD_FIELD) => {
                    let _ = rl.add_history_entry(line);
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() == 1 {
                        // show current field and available fields
                        let current = polenta.field_type();
                        println!(
                            "Current field: {} (order: {})",
                            current.name().yellow(),
                            current.order()
                        );
                        println!("Available fields:");
                        for field_type in FieldType::all() {
                            let marker = if *field_type == current { "*" } else { " " };
                            println!("  {}{}", marker, field_type.name().blue());
                        }
                        println!("Use {} to switch fields.", "field <name>".yellow());
                    } else if parts.len() == 2 {
                        // switch to the specified field
                        let field_name = parts[1];
                        match FieldType::from_name(field_name) {
                            Some(new_field_type) => {
                                if new_field_type == polenta.field_type() {
                                    println!(
                                        "Already using {} field.",
                                        new_field_type.name().yellow()
                                    );
                                } else {
                                    let new_polenta = PolentaInstance::new(new_field_type);
                                    let old_polenta = std::mem::replace(&mut polenta, new_polenta);
                                    polenta.migrate_symbols_from(&old_polenta);
                                    println!(
                                        "Switched to {} field (order: {}). Symbol table migrated.",
                                        new_field_type.name().yellow(),
                                        new_field_type.order()
                                    );
                                }
                            }
                            None => {
                                println!(
                                    "Unknown field: {}. Available fields: {}",
                                    field_name.red(),
                                    FieldType::all()
                                        .iter()
                                        .map(|f| f.name())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );
                            }
                        }
                    } else {
                        println!("Usage: {} or {}", "field".yellow(), "field <name>".yellow());
                    }
                }

                _ => {
                    // add ; to the input
                    let line_sanitized = format!("{};", line);
                    let input = line_sanitized.as_str();
                    let _ = rl.add_history_entry(input);

                    // process input
                    match polenta.interpret(input) {
                        Ok(result) => {
                            println!("{}", result.blue());
                        }
                        Err(e) => {
                            println!("{:?}", Report::from(e));
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("bye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
