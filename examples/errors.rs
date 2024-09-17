use polenta::Polenta;
type F = lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;

fn main() {
    let input = r#"
        let a = 5; 
        let b = a + 2^^2;
    "#;

    let err = Polenta::<F>::new().interpret(input).unwrap_err();
    println!("{}", err);
}
