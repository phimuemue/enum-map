#![cfg(feature = "serde")]

extern crate bincode;
#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_test;

use enum_map::EnumMap;

use serde_test::{assert_de_tokens_error, assert_tokens, Compact, Configure, Token};

#[derive(Debug, EnumMap, Deserialize, Serialize)]
enum Example {
    A,
    B,
}

#[test]
fn serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_tokens(
        &map.readable(),
        &[
            Token::Map { len: Some(2) },
            Token::UnitVariant {
                name: "Example",
                variant: "A",
            },
            Token::I32(5),
            Token::UnitVariant {
                name: "Example",
                variant: "B",
            },
            Token::I32(10),
            Token::MapEnd,
        ],
    );
}

#[test]
fn compact_serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_tokens(
        &map.compact(),
        &[
            Token::Tuple { len: 2 },
            Token::I32(5),
            Token::I32(10),
            Token::TupleEnd,
        ],
    );
}

#[test]
fn invalid_compact_deserialization() {
    assert_de_tokens_error::<Compact<EnumMap<bool, bool>>>(
        &[Token::I32(4)],
        "invalid type: integer `4`, expected a sequence",
    );
}

const JSON: &str = r#"{"A":5,"B":10}"#;

#[test]
fn json_serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_eq!(serde_json::to_string(&map).unwrap(), String::from(JSON));
}

#[test]
fn json_deserialization() {
    let example: EnumMap<Example, i32> = serde_json::from_str(JSON).unwrap();
    assert_eq!(example, enum_map! { Example::A => 5, Example::B => 10 });
}

#[test]
fn json_invalid_deserialization() {
    let example: Result<EnumMap<Example, i32>, _> = serde_json::from_str(r"{}");
    assert!(example.is_err());
}

#[test]
fn json_invalid_type() {
    let example: Result<EnumMap<Example, i32>, _> = serde_json::from_str("4");
    assert!(example.is_err());
}

#[test]
fn json_invalid_key() {
    let example: Result<EnumMap<Example, i32>, _> =
        serde_json::from_str(r#"{"a": 5, "b": 10, "c": 6}"#);
    assert!(example.is_err());
}

#[test]
fn bincode_serialization() {
    let example = enum_map! { false => 3u8, true => 4u8 };
    let serialized = bincode::serialize(&example).unwrap();
    assert_eq!(example, bincode::deserialize(&serialized).unwrap());
}

#[test]
fn bincode_too_short_deserialization() {
    assert!(
        bincode::deserialize::<EnumMap<bool, bool>>(&bincode::serialize(&()).unwrap()).is_err()
    );
}
