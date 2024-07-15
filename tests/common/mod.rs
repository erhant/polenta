use std::collections::HashMap;

use polenta::{Polenta, PolentaUtilExt};

/// Runs tests over the Goldilocks field (no particular reason for the field choice).
pub fn run_test(input: &str) -> HashMap<String, String> {
    type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

    let mut polenta = Polenta::<F>::new();
    polenta.interpret(input); // ignore returned values, just check symbols
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
