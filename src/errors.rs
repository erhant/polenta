use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum PolentaError {
    #[error(transparent)]
    UnknownIdentifier(#[from] std::io::Error),

    #[error("Oops it blew up")]
    DivisionByZero,

    #[error(transparent)]
    #[diagnostic(transparent)]
    ParserError(#[from] ParserError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("Parser error")]
pub struct ParserError {
    #[label("here")]
    pub at: SourceSpan,
}

// #[derive(Debug)]
// pub enum PolentaError {
//     UnknownIdentifier(String),
//     DivisionByZero,
// }
