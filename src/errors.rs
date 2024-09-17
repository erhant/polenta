use miette::{Diagnostic, NamedSource, SourceSpan};
use pest::error::{Error, ErrorVariant};
use thiserror::Error;

use crate::grammar::Rule;

/// A [miette](https://crates.io/crates/miette#-in-libraries) diagnostic for Polenta errors.
///
/// You can use this to display errors in a nice way either by returning a Miette `Result` in your `main`,
/// or creating a Miette `Report` from this and debug-printing it.
#[derive(Error, Diagnostic, Debug)]
pub enum PolentaError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    InterpreterError(#[from] InterpreterError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    ParserError(#[from] ParserError),
}

/// An error that can occur during interpretation.
#[derive(Error, Debug, Diagnostic)]
pub enum InterpreterError {
    #[error("Unknown Identifier: {0}")]
    UnknownIdentifier(String),
    #[error("Division by Zero")]
    DivisionByZero,
    #[help("Asserted expression must be non-zero.")]
    #[error("Assertion Failed")]
    AssertionFailed,
}

/// An error that can occur during parsing, most likely a syntax error.
#[derive(Error, Debug, Diagnostic)]
#[error("Syntax Error")]
pub struct ParserError {
    /// Source line of the error.
    #[source_code]
    src: NamedSource<String>,
    /// The problem location.
    #[label]
    problem: SourceSpan,
    /// Helpful message to see what is expected and what has been found.
    #[help]
    help: String,
}

/// A helper to convert a `pest` error into a `miette` error.
pub(crate) fn pest_error_to_miette_error(err: Error<Rule>) -> ParserError {
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
        help,
    };
    miette_error
}
