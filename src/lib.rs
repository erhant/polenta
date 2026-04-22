#![doc = include_str!("../README.md")]

mod errors;
mod instance;
mod interpreter;
mod parser;
mod utils;

pub use errors::{InterpreterError, PolentaError};
pub use instance::{IsPolentaInstance, PolentaInstance};
pub use interpreter::Polenta;
pub use utils::PolentaUtilExt;
