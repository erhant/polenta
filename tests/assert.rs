pub mod common;
use common::run_test_for_assert;

#[test]
fn test_assert_expr() {
    run_test_for_assert("assert 2 + 2 = 4;");
}

#[test]
fn test_assert_let() {
    run_test_for_assert("let a = 3; let b = 4; assert 7 - a = b;");
}

#[test]
fn test_assert_inv() {
    run_test_for_assert("let x = 123; let y = 1 / 123; assert x * y = 1;");
}
