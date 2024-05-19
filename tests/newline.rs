use mayfig::error::Error;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Tst {
	one: i32,
	two: f32,
}

const IS1: &str = r#"
one = 20
two = 4.4
"#;

const IS2: &str = r#"one = -2
two = 4
"#;

#[test]
fn is_newline() {
	let t1 = mayfig::from_str::<Tst>(IS1);
	let t1 = t1.unwrap();
	assert_eq!(t1, Tst { one: 20, two: 4.4 });

	let t2 = mayfig::from_str::<Tst>(IS2);
	let t2 = t2.unwrap();
	assert_eq!(t2, Tst { one: -2, two: 4.0 });
}

const NO1: &str = r#"
one = 20 two = 40
"#;

const NO2: &str = r#"
one = 0
two =
	0
"#;

#[test]
fn is_not_newline() {
	let t1 = mayfig::from_str::<Tst>(NO1);
	let e2 = t1.unwrap_err();
	assert!(matches!(e2, Error::ExpectedNewline(_)));

	let t2 = mayfig::from_str::<Tst>(NO2);
	let e2 = t2.unwrap_err();
	assert!(matches!(e2, Error::UnexpectedNewline));
}
