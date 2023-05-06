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

pub type Parser = Box<dyn Fn(String) -> ParseResult>;

pub fn parse_whitespace() -> Parser {
    let parser = move |source: String| {
        let space = parse_char(' ');
        let tab = parse_char('\t');
        let new_line = parse_char('\n');
        let whitespace = parse_any_of(vec![space, tab, new_line]);
        whitespace(source)
    };
    Box::new(parser)
}

pub fn parse_any_of(parsers: Vec<Parser>) -> Parser {
    let parser = move |source: String| {
        for parser in &parsers {
            match parser(source.clone()) {
                Ok(parsed_result) => return Ok(parsed_result),
                Err(_) => continue,
            }
        }
        Err(ParserError::Unknown(
            "could not parse anything from <any_of>".to_string(),
        ))
    };
    Box::new(parser)
}

pub fn parse_char(target_char: char) -> Parser {
    let parser = move |source: String| match source.chars().nth(0) {
        Some(char) => match target_char == char {
            true => ParseResult::Ok((source[1..].to_string(), ParsedObject::Char(char))),
            false => ParseResult::Err(ParserError::Unexpected(
                target_char.to_string(),
                char.to_string(),
                0,
            )),
        },
        None => ParseResult::Err(ParserError::Unexpected(target_char.to_string(), source, 0)),
    };
    Box::new(parser)
}
