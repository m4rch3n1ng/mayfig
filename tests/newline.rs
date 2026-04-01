use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;

mod maytest;

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
	assert_de!(IS1 as Tst, Tst { one: 20, two: 4.4 });
	assert_de!(IS2 as Tst, Tst { one: -2, two: 4.0 });
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
	assert_err!(
		NO1 as Tst,
		ErrorCode::ExpectedNewline('t'),
		Span::Point(Position {
			line: 2,
			col: 10,
			index: 10
		})
	);

	assert_err!(
		NO2 as Tst,
		ErrorCode::UnexpectedNewline,
		Span::Point(Position {
			line: 3,
			col: 6,
			index: 14
		})
	);
}
