pub mod common;

#[cfg(test)]
use common::run_test_for_error;

#[test]
fn test_poly_powers() {
    let err = run_test_for_error("let a = b;");
    println!("{:?}", err);
}
