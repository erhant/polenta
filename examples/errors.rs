use polenta::Polenta;
type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

fn main() -> miette::Result<()> {
    let input = "let 3 = /++++x;";

    Polenta::<F>::new().interpret(input)?;
    // println!("{:?}", err);

    Ok(())
}
