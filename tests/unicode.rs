use mayfig::error::ErrorCode;
use serde_derive::Deserialize;

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
	let t1 = mayfig::from_str::<Thing>(T1);
	let e1 = t1.unwrap_err();
	assert!(matches!(e1.code(), ErrorCode::ExpectedAsciiAlphabetic('ä')));

	let t2 = mayfig::from_str::<Thing>(T2);
	let e2 = t2.unwrap_err();
	assert!(matches!(
		e2.code(),
		ErrorCode::ExpectedAsciiAlphanumeric('ß')
	));

	let t3 = mayfig::from_str::<Thing>(T3);
	let e3 = t3.unwrap_err();
	assert!(matches!(e3.code(), ErrorCode::ExpectedNumeric('λ')));

	let t4 = mayfig::from_str::<Thing>(T4);
	let n4 = t4.unwrap();
	assert_eq!(n4, Thing { ä: 0, straße: 1 });
}
