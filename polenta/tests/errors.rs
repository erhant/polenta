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
    assert_eq!("Compiler Error".to_string(), err.to_string());
}
