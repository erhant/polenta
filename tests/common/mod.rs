//! Common utilities for tests only.
//!
//! Note that all test files should import this with `pub` visibility,
//! otherwise you get `dead_code` warnings.
//!
//! Relevant info: https://stackoverflow.com/a/67902444

use std::collections::HashMap;

use polenta::{InterpreterError, Polenta, PolentaUtilExt};
type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

pub fn run_test_for_error(input: &str) -> InterpreterError {
    let result = Polenta::<F>::new().interpret(input);
    assert!(result.is_err());
    return result.err().unwrap();
}

/// Runs tests over the Goldilocks field (no particular reason for the field choice).
pub fn run_test(input: &str) -> HashMap<String, String> {
    let mut polenta = Polenta::<F>::new();
    polenta.interpret(input).unwrap(); // ignore returned values, just check symbols
    polenta
        .symbols
        .into_iter()
        .map(|(k, v)| (k, Polenta::poly_print(&v)))
        .collect()
}

/// Interprets the given input, and checks for the given key-value pairs of symbols.
///
/// ### Example
///
/// ```rs
/// #[test]
/// fn test_let_2() {
///     run_test_for_symbols(
///         "let x = 4 * 3; let y = 12 + x;",
///         vec![("x", "12"), ("y", "24")],
///     );
/// }
/// ```
pub fn run_test_for_symbols(input: &str, expected_symbol_prints: Vec<(&str, &str)>) {
    let symbols = run_test(input);
    for (key, expected_value) in expected_symbol_prints {
        let value = symbols.get(key).unwrap();
        assert_eq!(value, expected_value);
    }
}
