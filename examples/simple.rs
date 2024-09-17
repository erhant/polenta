use polenta::Polenta;
type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

fn main() -> miette::Result<()> {
    let input = r#"
        let P(x) = 3 * x + 1;
        let Q(x) = x / 2;
        let z = Q@P@(5);
        assert z == 8;
    "#;

    Polenta::<F>::new().interpret(input)?;

    Ok(())
}
