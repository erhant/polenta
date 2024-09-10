use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

// #[derive(Error, Diagnostic, Debug)]
// pub enum PolentaError {
//     #[error(transparent)]
//     #[diagnostic(transparent)]
//     InterpreterError(#[from] InterpreterError),

//     #[error(transparent)]
//     #[diagnostic(transparent)]
//     ParserError(#[from] ParserError),
// }

pub enum ParserError {}

#[derive(Debug)]
pub enum InterpreterError {
    UnknownIdentifier(String),
    DivisionByZero,
    AssertionFailed,
}
