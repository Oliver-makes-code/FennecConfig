#![feature(decl_macro, let_chains)]
pub mod token;
pub mod parse;

use once_cell::sync::Lazy;
use parse::{FennecType, Parser, ParseError};

pub macro lazy($t:expr) {
    Lazy::new(|| $t)
}

pub fn parse(str: &str) -> Result<FennecType, ParseError> {
    Parser::new(token::Tokenizer::new(str)).parse_root()
}
