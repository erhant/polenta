use lambdaworks_math::{
    field::{element::FieldElement, fields::u64_goldilocks_field::Goldilocks64Field},
    polynomial::Polynomial,
};
use pest::Parser;
use polenta::grammar::{parse, parse_term, PolentaRule, TermSign};

type F = Goldilocks64Field;

fn main() {
    const INPUT: &str = "2*x - 2*x + 5";
    let mut program = parse(INPUT).expect("should parse");

    let mut last_sign = TermSign::default();
    let mut poly = Polynomial::<FieldElement<F>>::zero();
    for poly_pair in program.next().unwrap().into_inner() {
        match poly_pair.as_rule() {
            PolentaRule::Term => {
                let monomial = parse_term::<F>(poly_pair);
                poly = poly
                    + if last_sign == TermSign::Positive {
                        monomial
                    } else {
                        -monomial
                    };
            }
            PolentaRule::OpAdd => {
                last_sign = TermSign::from(poly_pair.as_str());
            }
            _ => {
                println!("{:?}", poly_pair);
                unreachable!()
            }
        }
    }

    println!("Parsed polynomial: {:?}", poly);
}
