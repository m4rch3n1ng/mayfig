use mayfig::error::ErrorCode;
use serde::Deserialize;

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
	let e1 = mayfig::from_str::<Thing>(T1).unwrap_err();
	assert!(matches!(e1.code(), ErrorCode::ExpectedWordStart('ä')));

	let e2 = mayfig::from_str::<Thing>(T2).unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::ExpectedWordContinue('ß')));

	let e3 = mayfig::from_str::<Thing>(T3).unwrap_err();
	assert!(matches!(e3.code(), ErrorCode::ExpectedNumeric('λ')));

	let n4 = mayfig::from_str::<Thing>(T4).unwrap();
	assert_eq!(n4, Thing { ä: 0, straße: 1 });
}
