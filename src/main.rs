use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;
use polenta::grammar::{parse, Polenta};

type F = Goldilocks64Field;

fn main() {
    const INPUT: &str = "2*x - 2*x + 5";

    let mut polenta = Polenta::<F>::new();
    let poly = polenta.parse_polynomial(parse(INPUT).unwrap().next().unwrap());

    println!("Parsed polynomial: {:?}", poly);
}
