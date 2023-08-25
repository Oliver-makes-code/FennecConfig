#![feature(decl_macro, let_chains, ascii_char, vec_into_raw_parts)]
pub mod extern_c;
pub mod parse;
pub mod token;

use once_cell::sync::Lazy;
use parse::{FennecType, ParseError, Parser};

pub macro lazy($t:expr) {
    Lazy::new(|| $t)
}

pub fn parse(str: &str) -> Result<FennecType, ParseError> {
    Parser::new(token::Tokenizer::new(str)).parse_root()
}
