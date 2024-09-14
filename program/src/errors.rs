use miette::{Diagnostic, NamedSource, SourceSpan};
use pest::error::{Error, ErrorVariant};
use thiserror::Error;

use crate::grammar::Rule;

/// A [miette](https://crates.io/crates/miette#-in-libraries) diagnostic for Polenta errors.
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

#[derive(Error, Debug, Diagnostic)]
#[error("Compiler Error")]
// #[diagnostic(code(oops::my::bad))]
pub struct ParserError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label]
    problem: SourceSpan,
    #[help]
    help: String,
}

pub fn pest_error_to_miette_error(err: Error<Rule>) -> ParserError {
    let (start, length) = match err.line_col {
        pest::error::LineColLocation::Pos((_, col)) => (col - 1, 1),
        pest::error::LineColLocation::Span((_, col_s), (_, col_e)) => {
            (col_s - 1, col_e - col_s + 1)
        }
    };

    let help = match &err.variant {
        ErrorVariant::CustomError { message } => message.into(),
        ErrorVariant::ParsingError {
            positives,
            negatives,
        } => {
            format!("Expected one of {:?}, got {:?}", positives, negatives)
        }
    };

    let miette_error = ParserError {
        src: NamedSource::new("input", err.line().to_string()).with_language("Rust"),
        problem: SourceSpan::new(start.into(), length),
        help: help,
    };
    miette_error
}
