use once_cell::sync::Lazy;
use regex::Regex;

use crate::lazy;

/**
 * (start, end)
 */
#[derive(Debug)]
pub struct Position(pub usize, pub usize);

/**
 * value?, (start, end)
 */
#[derive(Debug)]
pub enum Token {
    Identifier(String, Position),
    Flag(String, Position),
    Symbol(char, Position),
    Type(String, Position),
    Comment(String, Position),
    String(String, Position),
    Float(f64, Position),
    Int(i64, Position),
    Bool(bool, Position),
    Null(Position),
    Err(usize),
    Eof
}

#[derive(Clone)]
pub struct Tokenizer {
    pub chars: Vec<char>,
    pub index: usize,
}

const SYMBOLS: Lazy<Vec<char>> = lazy!{ "={}[]".chars().collect() };
const NULL: Lazy<Regex> = lazy!{ Regex::new(r"^(null|nil|void)").unwrap() };
const NOT_NULL: Lazy<Regex> = lazy!{ Regex::new(r"^(null|nil|void)[a-z0-9$_\-]").unwrap() };
const BOOL: Lazy<Regex> = lazy!{ Regex::new(r"^([Tt]rue|[Ff]alse|[01]b)").unwrap() };
const NOT_BOOL: Lazy<Regex> = lazy!{ Regex::new(r"^([Tt]rue|[Ff]alse|[01]b)[a-z0-9$_\-]").unwrap() };
const HEX_LITERAL: Lazy<Regex> = lazy!{ Regex::new(r"^([01])x([0-9a-fA-F]+)").unwrap() };
const OCT_LITERAL: Lazy<Regex> = lazy!{ Regex::new(r"^([01])o([0-7]+)").unwrap() };
const BIN_LITERAL: Lazy<Regex> = lazy!{ Regex::new(r"^([01])b([01]+)").unwrap() };
const FLOATING: Lazy<Regex> = lazy!{ Regex::new(r"^\-?[0-9]+\.[0-9]+").unwrap() };
const INTEGER: Lazy<Regex> = lazy!{ Regex::new(r"^\-?[0-9]+").unwrap() };
const IDENTIFIER: Lazy<Regex> = lazy!{ Regex::new(r"^([a-zA-Z$_][a-zA-Z$_\-0-9]+)").unwrap() };
const FLAG: Lazy<Regex> = lazy!{ Regex::new(r"^\-([a-zA-Z$_\-0-9]+)").unwrap() };

impl Tokenizer {
    pub fn new(doc: &str) -> Self {
        Self {
            chars: doc.chars().collect(),
            index: 0
        }
    }

    pub fn get_char(&self) -> char {
        self.chars[self.index]
    }

    pub fn is_end(&self) -> bool {
        self.index >= self.chars.len()
    }

    pub fn seek_to(&mut self, chars: Vec<char>) -> String {
        self.seek_to_esc(chars, false)
    }

    pub fn seek_to_esc(&mut self, chars: Vec<char>, escape: bool) -> String {
        let mut out = "".to_string();
        while !self.is_end() && !chars.contains(&self.get_char()) {
            let char = self.get_char();
            if escape && char == '\\' {
                self.index += 1;
                out.push(Tokenizer::get_escape_char(self.get_char()))
            } else {
                out.push(char);
            }
            self.index += 1;
        }
        out
    }

    pub fn get_escape_char(escape: char) -> char {
        match escape {
            'n' => return '\n',
            't' => return '\t',
            'b' => return 8 as char,
            'f' => return 12 as char,
            'r' => return '\r',
            _ => return escape
        }
    }

    pub fn to_triple_quote(&mut self) -> String {
        let mut out = "".to_string();

        while !self.is_end() {
            if self.to_string().starts_with("\"\"\"") {
                break;
            }
            let char = self.get_char();
            if char == '\\' {
                self.index += 1;
                let escape = Tokenizer::get_escape_char(self.get_char());
                out.push(escape);
            } else {
                out.push(char);
            }
            self.index += 1;
        }

        self.index += 3;

        out
    }

    pub fn to_string(&self) -> String {
        self.chars.split_at(self.index).1.iter().collect()
    }

    pub fn next(&mut self) -> Token {
        loop {
            let token = self.next_token();
            if let Token::Comment(_, _) = token {
                continue;
            }
            if let Token::Type(_, _) = token {
                continue;
            }
            return token;
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.is_end() {
            return Token::Eof
        }

        let mut char = self.get_char();

        while char.is_whitespace() {
            self.index += 1;
            char = self.get_char();
        }

        let start_idx = self.index;

        if SYMBOLS.contains(&char) {
            self.index += 1;
            return Token::Symbol(char, Position(start_idx, self.index));
        }

        if char == '#' {
            self.index += 1;
            return Token::Comment(self.seek_to(vec!['\n']).trim().to_string(), Position(start_idx, self.index));
        }

        if char == ':' {
            self.index += 1;
            return Token::Type(self.seek_to(SYMBOLS.to_vec()).trim().to_string(), Position(start_idx, self.index));
        }

        let str = self.to_string();

        if str.starts_with("-\"\"\"") {
            self.index += 4;
            let quote = self.to_triple_quote();
            let lines = quote.lines();
            let mut str = "".to_string();
            for line in lines {
                str.push_str(line.trim_start());
                str.push('\n');
            }
            return Token::String(str.trim().to_string(), Position(start_idx, self.index))
        }

        if str.starts_with("\"\"\"") {
            self.index += 3;
            return Token::String(self.to_triple_quote().trim().to_string(), Position(start_idx, self.index))
        }

        if char == '"' {
            self.index += 1;
            let str = self.seek_to(vec!['"']);
            self.index += 1;
            return Token::String(str, Position(start_idx, self.index));
        }

        let null_match = NULL.captures(&str);
        let not_null_match = NOT_NULL.captures(&str);

        if not_null_match.is_none() && let Some(capture) = null_match {
            let str = &capture[0];
            self.index += str.len();
            return Token::Null(Position(start_idx, self.index));
        }

        let bool_match = BOOL.captures(&str);
        let not_bool_match = NOT_BOOL.captures(&str);

        if not_bool_match.is_none() && let Some(capture) = bool_match {
            let str = &capture[0];
            self.index += str.len();
            return Token::Bool(str.to_lowercase() == "true" || str == "1b", Position(start_idx, self.index));
        }

        if let Some(capture) = HEX_LITERAL.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            let sign = if &capture[1] == "1" {-1} else {1};
            return Token::Int(sign * i64::from_str_radix(&capture[2], 16).unwrap(), Position(start_idx, self.index))
        }

        if let Some(capture) = OCT_LITERAL.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            let sign = if &capture[1] == "1" {-1} else {1};
            return Token::Int(sign * i64::from_str_radix(&capture[2], 8).unwrap(), Position(start_idx, self.index))
        }

        if let Some(capture) = BIN_LITERAL.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            let sign = if &capture[1] == "1" {-1} else {1};
            return Token::Int(sign * i64::from_str_radix(&capture[2], 2).unwrap(), Position(start_idx, self.index))
        }

        if let Some(capture) = FLOATING.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            return Token::Float(str.parse::<f64>().unwrap(), Position(start_idx, self.index))
        }

        if let Some(capture) = INTEGER.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            return Token::Int(str.parse::<i64>().unwrap(), Position(start_idx, self.index))
        }

        if let Some(capture) = FLAG.captures(&str) {
            let str = &capture[0];
            self.index += str.len()+1;
            return Token::Flag(str.to_string(), Position(start_idx, self.index))
        }

        if let Some(capture) = IDENTIFIER.captures(&str) {
            let str = &capture[0];
            self.index += str.len();
            return Token::Identifier(str.to_string(), Position(start_idx, self.index))
        }

        Token::Err(start_idx)
    }
}
