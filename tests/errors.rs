pub mod common;

#[cfg(test)]
use common::run_test_for_error;

#[test]
fn test_unknown_identifier() {
    let err = run_test_for_error("let a = b;");
    assert_eq!("Unknown Identifier: b".to_string(), err.to_string());
}

#[test]
fn test_div_by_zero() {
    let err = run_test_for_error("let a = 3 / (3 - 3);");
    assert_eq!("Division by Zero".to_string(), err.to_string());
}

#[test]
fn test_syntax_error() {
    let err = run_test_for_error("let a = ++;");
    assert_eq!("Syntax Error".to_string(), err.to_string());
}

#[test]
fn test_poly_pow_poly() {
    let err = run_test_for_error("let P(x) = 2 + x; P ^ P;");
    assert_eq!("Expected Constant Polynomial".to_string(), err.to_string());
}

#[test]
fn test_poly_eval_at_poly() {
    let err = run_test_for_error("let P(x) = x + 1; let Q(x) = x + 2; P@Q;");
    assert_eq!("Expected Constant Polynomial".to_string(), err.to_string());
}
