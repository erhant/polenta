use miette::{Diagnostic, NamedSource, SourceSpan};
use pest::{error::Error, iterators::Pair};
use thiserror::Error;

use crate::grammar::Rule;

#[derive(Error, Diagnostic, Debug)]
pub enum PolentaError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    InterpreterError(#[from] InterpreterError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ParserError(#[from] ParserError),
}

#[derive(Error, Debug, Diagnostic)]
#[error("Compiler error")]
#[diagnostic(code(oops::my::bad))]
pub struct ParserError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
    #[help]
    help: String,
}

pub fn pest_error_to_miette_error(source: &str, err: Error<Rule>) -> ParserError {
    let (location, length) = match err.location {
        pest::error::InputLocation::Pos(pos) => (pos, 1),
        pest::error::InputLocation::Span((location, length)) => (location, length),
    };

    let miette_error = ParserError {
        src: NamedSource::new("input", source.to_string()).with_language("Rust"),
        bad_bit: SourceSpan::new(location.into(), length),
        help: "This is a label".to_string(),
    };
    miette_error
}

#[derive(Error, Debug, Diagnostic)]
#[diagnostic()]
pub enum InterpreterError {
    #[help("try doing it better next time?")]
    #[error("Unknown Identifier: {0}")]
    UnknownIdentifier(String),
    #[help("im zeroooo?")]
    #[error("Division by Zero")]
    DivisionByZero,
    #[help("dont do it")]
    #[error("Assertion Failed")]
    AssertionFailed,
}
