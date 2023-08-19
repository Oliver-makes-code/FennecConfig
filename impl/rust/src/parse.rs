use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{token::{Token, Tokenizer}, lazy};

#[derive(Debug)]
pub enum FennecType {
    Object(HashMap<String, FennecType>),
    Array(Vec<FennecType>),
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Null
}

impl ToString for FennecType {
    fn to_string(&self) -> String {
        self.to_string_internal(0, true)
    }
}

const INDENT: usize = 4;
const IDENTIFIER: Lazy<Regex> = lazy!{ Regex::new(r"^([a-zA-Z$_][a-zA-Z$_\-0-9]+)$").unwrap() };

impl FennecType {
    fn replace_escapes(str: &str) -> String {
        str
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace(8 as char, "\\b")
        .replace(12 as char, "\\f")
    }

    fn to_string_internal(&self, indent: usize, first: bool) -> String {
        match self {
            Self::String(str) => return format_args!("\"{}\"", &FennecType::replace_escapes(str)).to_string(),
            Self::Float(num) => return num.to_string(),
            Self::Int(num) => return num.to_string(),
            Self::Bool(bool) => return bool.to_string(),
            Self::Null => return "null".to_string(),

            Self::Object(obj) => {

                let mut out = "".to_string();

                if !first {
                    out.push('{');
                    out.push('\n');
                }

                let idt = if first { indent } else { indent + 1 };
                
                for key in obj.keys() {
                    out.push_str(&" ".repeat(idt*INDENT));
                    if IDENTIFIER.is_match(key) {
                        out.push_str(key);
                    } else {
                        out.push('"');
                        out.push_str(&FennecType::replace_escapes(key));
                        out.push('"');
                    }
                    let val = &obj[key];
                    match val {
                        Self::Object(_) | Self::Array(_) => {
                            out.push(' ');
                            out.push_str(&val.to_string_internal(idt, false))
                        }
                        _ => {
                            out.push_str(" = ");
                            out.push_str(&val.to_string_internal(idt, false));
                        }
                    }
                    out.push('\n');
                }

                if !first {
                    out.push_str(&" ".repeat(indent*INDENT));
                    out.push('}');
                }

                return out;
            }, 
            Self::Array(arr) => {
                let mut out = "[\n".to_string();

                let idt = indent + 1;

                for val in arr {
                    out.push_str(&" ".repeat(idt*INDENT));
                    out.push_str(&val.to_string_internal(idt, false));
                    out.push('\n');
                }

                out.push_str(&" ".repeat(indent*INDENT));
                out.push(']');
                return out;
            }
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, FennecType>> {
        if let Self::Object(var) = self {
            return Some(var);
        }
        None
    }
    pub fn get_key(&self, key: &str) -> Option<&FennecType> {
        if let Self::Object(var) = self {
            return var.get(key);
        }
        None
    }
    pub fn as_array(&self) -> Option<&Vec<FennecType>> {
        if let Self::Array(var) = self {
            return Some(var);
        }
        None
    }
    pub fn get_index(&self, index: usize) -> Option<&FennecType> {
        if let Self::Array(var) = self {
            return var.get(index);
        }
        None
    }
    pub fn as_string(&self) -> Option<String> {
        if let Self::String(var) = self {
            return Some(var.to_string());
        }
        None
    }
    pub fn as_float(&self) -> Option<f64> {
        if let Self::Float(var) = self {
            return Some(*var);
        }
        None
    }
    pub fn as_int(&self) -> Option<i64> {
        if let Self::Int(var) = self {
            return Some(*var);
        }
        None
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(var) = self {
            return Some(*var);
        }
        None
    }
    pub fn as_null(&self) -> Option<()> {
        if let Self::Null = self {
            return Some(());
        }
        None
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token)
}

pub struct Parser {
    tokenizer: Tokenizer
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer
        }
    }

    pub fn parse_root(&mut self) -> Result<FennecType, ParseError> {
        let token = self.tokenizer.next();
        match &token {
            Token::Identifier(_, pos) | Token::Flag(_, pos) => {
                self.tokenizer.index = pos.0;
                return self.parse_object(true);
            }

            Token::String(val, pos) => {
                if let Token::Eof = self.tokenizer.next() {
                    return Ok(FennecType::String(val.to_string()))
                }
                self.tokenizer.index = pos.0;
                return self.parse_object(true);
            }

            Token::Symbol(_, _) |
            Token::Float(_, _) |
            Token::Bool(_, _) |
            Token::Int(_, _) |
            Token::Null(_) 
                => return self.parse_value(token),

            _ => return Err(ParseError::UnexpectedToken(token))
        };
        
    }

    fn parse_value(&mut self, token: Token) -> Result<FennecType, ParseError> {
        match &token {
            Token::String(_, _) |
            Token::Float(_, _) |
            Token::Bool(_, _) |
            Token::Int(_, _) |
            Token::Null(_) 
                => return self.parse_primitive(token),

            Token::Symbol(char, _) => match char {
                '{' => return self.parse_object(false),
                '[' => return self.parse_array(),
                _ => return Err(ParseError::UnexpectedToken(token))
            }

            _ => {
                return Err(ParseError::UnexpectedToken(token))
            }
        };
    }

    fn parse_primitive(&mut self, token: Token) -> Result<FennecType, ParseError> {
        match &token {
            Token::String(str, _) => return Ok(FennecType::String(str.to_string())),
            Token::Float(val, _) => return Ok(FennecType::Float(*val)),
            Token::Int(val, _) => return Ok(FennecType::Int(*val)),
            Token::Bool(val, _) => return Ok(FennecType::Bool(*val)),
            Token::Null(_) => return Ok(FennecType::Null),

            _ => {
                return Err(ParseError::UnexpectedToken(token))
            }
        };
    }

    fn parse_object(&mut self, expect_eof: bool) -> Result<FennecType, ParseError> {
        let mut out = HashMap::new();

        loop {
            let token = self.tokenizer.next();
            match &token {
                Token::Eof => if expect_eof {
                    return Ok(FennecType::Object(out))
                } else {
                    return Err(ParseError::UnexpectedToken(token))
                }

                Token::Symbol(char, _) => if *char == '}' && !expect_eof {
                    return Ok(FennecType::Object(out))
                } else {
                    return Err(ParseError::UnexpectedToken(token))
                }

                Token::Flag(name, _) => {
                    out.insert(name.to_string(), FennecType::Bool(true));
                }

                Token::Identifier(name, _) | Token::String(name, _) => {
                    let next = self.tokenizer.next();

                    if let Token::Symbol(symbol, _) = next {
                        match symbol {
                            '=' => {
                                let prim_token = self.tokenizer.next();
                                let primitive = self.parse_primitive(prim_token);
                                if primitive.is_err() {
                                    return primitive;
                                }
                                out.insert(name.to_string(), primitive.expect("We just checked! This shouldn't be Err."));
                            }
                            '[' => {
                                let arr = self.parse_array();
                                if arr.is_err() {
                                    return arr;
                                }
                                out.insert(name.to_string(), arr.expect("We just checked! This shouldn't be Err."));
                            }
                            '{' => {
                                let obj = self.parse_object(false);
                                if obj.is_err() {
                                    return obj;
                                }
                                out.insert(name.to_string(), obj.expect("We just checked! This shouldn't be Err."));
                            }
                            _ => return Err(ParseError::UnexpectedToken(next))
                        }
                    } else {
                        return Err(ParseError::UnexpectedToken(next));
                    }
                }

                _ => return Err(ParseError::UnexpectedToken(token))
            }
        }
    }

    fn parse_array(&mut self) -> Result<FennecType, ParseError> {
        let mut out = Vec::new();

        loop {
            let token = self.tokenizer.next();
            if let Token::Symbol(char, _) = token && char == ']' {
                return Ok(FennecType::Array(out));
            }
            let val = self.parse_value(token);
            if let Err(err) = val {
                return Err(err);
            }
            out.push(val.expect("We just checked! This shouldn't be Err."));
        }
    }
}
