use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::field::traits::IsPrimeField;
use lambdaworks_math::polynomial::Polynomial;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "polenta.pest"]
pub struct PolentaParser;

/// Alias for Polenta rule.
pub type PolentaRule = Rule;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    PolentaParser::parse(Rule::Main, input)
}

#[derive(Debug, Default, PartialEq)]
pub enum TermSign {
    #[default]
    Positive,
    Negative,
}

impl From<&str> for TermSign {
    fn from(s: &str) -> Self {
        match s {
            "+" => TermSign::Positive,
            "-" => TermSign::Negative,
            _ => unreachable!(),
        }
    }
}

pub fn parse_term<F: IsPrimeField>(term: Pair<Rule>) -> Polynomial<FieldElement<F>> {
    let mut coefficient: FieldElement<F> = FieldElement::<F>::one();
    let mut degree: usize = 1;

    for inner_term in term.into_inner() {
        match inner_term.as_rule() {
            PolentaRule::Coefficient => {
                coefficient = FieldElement::<F>::from(
                    inner_term
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<u64>()
                        .unwrap(),
                );
            }
            PolentaRule::Exponent => {
                degree = inner_term
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
            }
            PolentaRule::Uint => {
                coefficient = FieldElement::<F>::from(inner_term.as_str().parse::<u64>().unwrap());
                degree = 0;
            }
            _ => {
                println!("{:?}", inner_term);
                unreachable!();
            }
        }
    }

    Polynomial::new_monomial(coefficient, degree)
}
