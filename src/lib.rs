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

#[test]
fn test_parse_char() {
    let parser = parse_char('b');
    let result = parser("bad".to_string()).unwrap();
    assert_eq!(result.0, "ad".to_string());
    assert_eq!(result.1, ParsedObject::Char('b'));

    let parser = parse_char('a');
    let result = parser("a".to_string()).unwrap();
    assert_eq!(result.0, "".to_string());
    assert_eq!(result.1, ParsedObject::Char('a'));

    let parser = parse_char('d');
    let result = parser("bad".to_string());
    assert!(result.is_err());

    let parser = parse_char('d');
    let result = parser("".to_string());
    assert!(result.is_err());
}
