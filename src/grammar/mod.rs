pub mod arith;

// use std::collections::HashMap;

// use lambdaworks_math::field::{element::FieldElement, traits::IsPrimeField};
// use lambdaworks_math::polynomial::Polynomial;
// use pest::{
//     error::Error,
//     iterators::{Pair, Pairs},
//     Parser,
// };
// use pest_derive::Parser;

// /// `PolentaParser` parses a given code with respect to the
// /// provided rule.
// ///
// /// ### Example
// ///
// /// ```rs
// /// let input = "3*x + 5";
// /// PolentaParser::parse(Rule::Main, input);
// /// ```
// #[derive(Parser)]
// #[grammar = "polenta.pest"]
// pub struct PolentaParser;

// /// Alias for `Rule`.
// pub type PolentaRule = Rule;

// enum PolentaValue<F: IsPrimeField> {
//     Poly(Polynomial<FieldElement<F>>),
//     Value(FieldElement<F>),
// }

// pub struct Polenta<F: IsPrimeField> {
//     symbols: HashMap<String, PolentaValue<F>>,
//     rule: Rule,
// }

// pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
//     parse_rule(input, Rule::Main)
// }

// pub fn parse_rule(input: &str, rule: Rule) -> Result<Pairs<Rule>, Error<Rule>> {
//     PolentaParser::parse(rule, input)
// }

// impl<F: IsPrimeField> Polenta<F> {
//     pub fn new() -> Self {
//         Self {
//             symbols: HashMap::new(),
//             rule: Rule::Main,
//         }
//     }

//     pub fn parse_coefficient(&self, coeff: Pair<Rule>) -> FieldElement<F> {
//         assert_eq!(coeff.as_rule(), Rule::Coefficient);
//         FieldElement::<F>::from(
//             coeff
//                 .into_inner()
//                 .next()
//                 .unwrap()
//                 .as_str()
//                 .parse::<u64>()
//                 .unwrap(),
//         )
//     }

//     pub fn parse_uint(&self, uint: Pair<Rule>) -> u128 {
//         assert_eq!(uint.as_rule(), Rule::Uint);
//         uint.into_inner().next().unwrap().as_str().parse().unwrap()
//     }

//     pub fn parse_term(&self, terms: Pair<Rule>) -> (String, Polynomial<FieldElement<F>>) {
//         assert_eq!(terms.as_rule(), Rule::Term);
//         let mut coefficient = FieldElement::<F>::one();
//         let mut degree = 1usize;
//         let mut term_name = String::default();
//         for inner_term in terms.into_inner() {
//             match inner_term.as_rule() {
//                 PolentaRule::Coefficient => coefficient = self.parse_coefficient(inner_term),
//                 PolentaRule::Exponent => degree = self.parse_uint(inner_term) as usize,
//                 PolentaRule::Identifier => term_name = inner_term.as_str().to_string(),
//                 PolentaRule::Uint => {
//                     (degree, coefficient) = (
//                         0,
//                         FieldElement::<F>::from(inner_term.as_str().parse::<u64>().unwrap()),
//                     )
//                 }
//                 _ => unreachable!("{:?}", inner_term),
//             }
//         }

//         if degree != 0 && term_name.is_empty() {
//             panic!("Need term name.");
//         }

//         (term_name, Polynomial::new_monomial(coefficient, degree))
//     }

//     pub fn parse_polynomial(
//         &mut self,
//         poly_pair: Pair<PolentaRule>,
//     ) -> Polynomial<FieldElement<F>> {
//         assert_eq!(poly_pair.as_rule(), Rule::Poly);

//         let mut last_sign_positive = true;
//         let mut poly = Polynomial::<FieldElement<F>>::zero();

//         for inner_poly in poly_pair.into_inner() {
//             match inner_poly.as_rule() {
//                 PolentaRule::Term => {
//                     let (term_name, monomial) = self.parse_term(inner_poly);
//                     poly = poly
//                         + if last_sign_positive {
//                             monomial
//                         } else {
//                             -monomial
//                         };
//                 }
//                 PolentaRule::OpAdd => {
//                     last_sign_positive = match inner_poly.as_str() {
//                         "+" => true,
//                         "-" => false,
//                         _ => unreachable!(),
//                     }
//                 }
//                 _ => unreachable!("{:?}", inner_poly),
//             }
//         }

//         poly
//     }
// }
