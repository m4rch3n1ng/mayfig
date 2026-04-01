use mayfig::error::ErrorCode;
use serde::Deserialize;

mod maytest;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Thing {
	ä: u32,
	straße: u32,
}

const T1: &str = r#"
ä = 0
"#;

const T2: &str = r#"
"ä" = 0
straße = 1
"#;

const T3: &str = r#"
"ä" = 0
"straße" = λ
"#;

const T4: &str = r#"
"ä" = 0
"straße" = 1
"#;

#[test]
fn test() {
	assert_err!(T1 as Thing, ErrorCode::ExpectedWordStart('ä'));
	assert_err!(T2 as Thing, ErrorCode::ExpectedWordContinue('ß'));
	assert_err!(T3 as Thing, ErrorCode::ExpectedNumeric('λ'));
	assert_de!(T4 as Thing, Thing { ä: 0, straße: 1 });
}
