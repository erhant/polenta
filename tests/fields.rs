use polenta::{FieldType, PolentaInstance};

#[test]
fn test_migrate_constant_between_fields() {
    // Set a = 42 in Babybear31, migrate to Goldilocks, value should stay 42
    let mut src = PolentaInstance::new(FieldType::Babybear31);
    src.interpret("let a = 42;").unwrap();

    let mut dest = PolentaInstance::new(FieldType::Goldilocks);
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("a;").unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_migrate_polynomial_between_fields() {
    // P(x) = 3*x + 1 in Goldilocks, migrate to Mersenne31
    let mut src = PolentaInstance::new(FieldType::Goldilocks);
    src.interpret("let P(x) = 3*x + 1;").unwrap();

    let mut dest = PolentaInstance::new(FieldType::Mersenne31);
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("P;").unwrap();
    assert_eq!(result, "3*x + 1");
}

#[test]
fn test_migrate_to_smaller_field_reduces() {
    // Value larger than Babybear31 order (2013265921) should be reduced
    let mut src = PolentaInstance::new(FieldType::Goldilocks);
    src.interpret("let a = 2013265922;").unwrap(); // order + 1

    let mut dest = PolentaInstance::new(FieldType::Babybear31);
    dest.migrate_symbols_from(&src);
    let result = dest.interpret("a;").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_migrate_roundtrip() {
    // Babybear31 → Goldilocks → Babybear31 should preserve small values
    let mut a = PolentaInstance::new(FieldType::Babybear31);
    a.interpret("let x = 7;").unwrap();

    let mut b = PolentaInstance::new(FieldType::Goldilocks);
    b.migrate_symbols_from(&a);

    let mut c = PolentaInstance::new(FieldType::Babybear31);
    c.migrate_symbols_from(&b);

    let result = c.interpret("x;").unwrap();
    assert_eq!(result, "7");
}
