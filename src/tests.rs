#[test]
fn test_parse_char() {
    use crate::{parse_char, ParsedObject};

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

#[test]
fn test_parse_any_of() {
    use crate::{parse_any_of, parse_char, parse_whitespace, ParsedObject};

    let parser = parse_any_of(vec![parse_char('b'), parse_whitespace()]);
    let result = parser(" bad".to_string()).unwrap();
    assert_eq!(result.0, "bad".to_string());
    assert_eq!(result.1, ParsedObject::Char(' '));

    let parser = parse_any_of(vec![parse_char('a')]);
    let result = parser("bad".to_string());
    assert!(result.is_err());
}

#[test]
fn test_parse_whitespace() {
    use crate::{parse_whitespace, ParsedObject};

    let parser = parse_whitespace();
    let result = parser(" ok".to_string()).unwrap();
    assert_eq!(result.0, "ok".to_string());
    assert_eq!(result.1, ParsedObject::Char(' '));

    let parser = parse_whitespace();
    let result = parser("\tok".to_string()).unwrap();
    assert_eq!(result.0, "ok".to_string());
    assert_eq!(result.1, ParsedObject::Char('\t'));

    let parser = parse_whitespace();
    let result = parser("\nok".to_string()).unwrap();
    assert_eq!(result.0, "ok".to_string());
    assert_eq!(result.1, ParsedObject::Char('\n'));

    let parser = parse_whitespace();
    let result = parser("bad".to_string());
    assert!(result.is_err());

    let parser = parse_whitespace();
    let result = parser("".to_string());
    assert!(result.is_err());
}
