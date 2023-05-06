mod tests;

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedObject {
    Char(char),
    String(String),
    Digit(char),
    Number(String),
}

#[derive(Debug, Clone)]
pub enum ParserError {
    Unexpected(String, String, usize),
    Unknown(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Unexpected(expected, unexpected, position) => {
                write!(f, "expected {expected}, got {unexpected},  in {position}",)
            }
            ParserError::Unknown(message) => write!(f, "{message}"),
        }
    }
}

pub type ParseResult = Result<(String, ParsedObject), ParserError>;

pub fn any_of(parsers: Vec<impl Fn(String) -> ParseResult>) -> impl Fn(String) -> ParseResult {
    move |source: String| {
        for parser in &parsers {
            match parser(source.clone()) {
                Ok(parsed_result) => return Ok(parsed_result),
                Err(_) => continue,
            }
        }
        Err(ParserError::Unknown(
            "could not parse anything from 'any_of'".to_string(),
        ))
    }
}

pub fn parse_char(target_char: char) -> impl Fn(String) -> ParseResult {
    move |source: String| match source.chars().nth(0) {
        Some(char) => match target_char == char {
            true => ParseResult::Ok((source[1..].to_string(), ParsedObject::Char(char))),
            false => ParseResult::Err(ParserError::Unexpected(
                target_char.to_string(),
                char.to_string(),
                0,
            )),
        },
        None => ParseResult::Err(ParserError::Unexpected(target_char.to_string(), source, 0)),
    }
}
