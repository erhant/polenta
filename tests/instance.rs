use polenta::PolentaInstance;

#[test]
fn test_migrate_constant_between_fields() {
    // Set a = 42 in Babybear31, migrate to Goldilocks, value should stay 42
    let mut src = PolentaInstance::new_from_name("babybear31").unwrap();
    src.interpret("let a = 42;").unwrap();

    let mut dest = PolentaInstance::new_from_name("goldilocks").unwrap();
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("a;").unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_migrate_polynomial_between_fields() {
    // P(x) = 3*x + 1 in Goldilocks, migrate to Mersenne31
    let mut src = PolentaInstance::new_from_name("goldilocks").unwrap();
    src.interpret("let P(x) = 3*x + 1;").unwrap();

    let mut dest = PolentaInstance::new_from_name("mersenne31").unwrap();
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("P;").unwrap();
    assert_eq!(result, "3*x + 1");
}

#[test]
fn test_migrate_to_smaller_field_reduces() {
    // Value larger than Babybear31 order (2013265921) should be reduced
    let mut src = PolentaInstance::new_from_name("goldilocks").unwrap();
    src.interpret("let a = 2013265922;").unwrap(); // order + 1

    let mut dest = PolentaInstance::new_from_name("babybear31").unwrap();
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("a;").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_migrate_roundtrip() {
    // Babybear31 → Goldilocks → Babybear31 should preserve small values
    let mut a = PolentaInstance::new_from_name("babybear31").unwrap();
    a.interpret("let x = 7;").unwrap();

    let mut b = PolentaInstance::new_from_name("goldilocks").unwrap();
    b.migrate_symbols_from(&a);

    let mut c = PolentaInstance::new_from_name("babybear31").unwrap();
    c.migrate_symbols_from(&b);

    let result = c.interpret("x;").unwrap();
    assert_eq!(result, "7");
}
