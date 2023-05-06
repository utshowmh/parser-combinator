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
fn test_any_of() {
    use crate::{any_of, parse_char, ParsedObject};

    let parser = any_of(vec![parse_char('b')]);
    let result = parser("bad".to_string()).unwrap();
    assert_eq!(result.0, "ad".to_string());
    assert_eq!(result.1, ParsedObject::Char('b'));

    let parser = any_of(vec![parse_char('a')]);
    let result = parser("bad".to_string());
    assert!(result.is_err());
}