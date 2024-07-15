mod common;
use common::run_test_for_symbols;

#[test]
fn test_poly_powers() {
    run_test_for_symbols(
        "let P(x) = (1+(1-1))* x ^3 + (6/2 )*x^ 2 +      x + 4;",
        vec![("P", "x^3 + 3*x^2 + x + 4")],
    );
}

#[test]
fn test_const_eval() {
    run_test_for_symbols("let a = 2@7;", vec![("a", "2")]);
}

#[test]
fn test_power_precedence() {
    run_test_for_symbols("let P(x) = 3*x^2^3;", vec![("P", "3*x^8")]);
    run_test_for_symbols("let P(x) = (3*x^2)^3;", vec![("P", "27*x^6")]);
}

#[test]
fn test_let_shadowing() {
    run_test_for_symbols(
        r#"
        let P(x) = 3*x;
        let P(x) = 3*P + 5;
    "#,
        vec![("P", "9*x + 5")],
    );
}

#[test]
fn test_multiplications() {
    run_test_for_symbols(
        "let P(x) = (x + 1)*(x + 2)*(x + 4);",
        vec![("P", "x^3 + 7*x^2 + 14*x + 8")],
    );
}

#[test]
fn test_poly_eval() {
    run_test_for_symbols(
        r#"
        let P(x) = 4*x + 2; 
        let t = 2;
        let a = 5 + (P+P)@(t+t);
        "#,
        vec![("P", "4*x + 2"), ("t", "2"), ("a", "41")],
    );
}

#[test]
fn test_poly_eval_chain() {
    run_test_for_symbols(
        r#"
        let P(x) = 3*x + 1; 
        let Q(x) = x/2;
        Q@P@Q@P@3;
        "#,
        vec![("!!", "8")],
    );
}

#[test]
fn test_poly_with_identifier() {
    run_test_for_symbols(
        r#"
        let t = 2;
        let x = 5;
        let P(x) = x^t + 2*x; // x^2 + 2*x
        "#,
        vec![("t", "2"), ("x", "5"), ("P", "x^2 + 2*x")],
    );
}
