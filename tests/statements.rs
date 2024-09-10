pub mod common;
use common::run_test_for_symbols;

#[test]
fn test_minus() {
    run_test_for_symbols(
        "-1; // should be 18446744069414584320 in goldilocks",
        vec![("!!", "18446744069414584320")],
    );
    run_test_for_symbols("-(-1);", vec![("!!", "1")]);
}

#[test]
fn test_large() {
    run_test_for_symbols(
        "18446744069414584323; // 2 above the order in goldilocks",
        vec![("!!", "2")],
    );
}

#[test]
fn test_div() {
    run_test_for_symbols(
        "1 / 2; // should be 9223372034707292161 in goldilocks",
        vec![("!!", "9223372034707292161")],
    );
    run_test_for_symbols("2 * (1 / 2);", vec![("!!", "1")]);
}

#[test]
fn test_let_1() {
    run_test_for_symbols("let abc = 2 + 3 * 2 ^ 3 - 1;", vec![("abc", "25")]);
}

#[test]
fn test_let_2() {
    run_test_for_symbols(
        "let x = 4 * 3; let y = 12 + x;",
        vec![("x", "12"), ("y", "24")],
    );
}
