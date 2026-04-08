use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;

mod maytest;

#[derive(Debug, Deserialize)]
struct T {
	t: String,
}

const T1: &str = r#"
t = "test"
"#;

const T2: &str = r#"
t = "\t\\\t"
"#;

const T3: &str = r#"
t = ""
"#;

#[test]
fn string() {
	assert_de!(T1 as T => t1, t1.t, "test");
	assert_de!(T2 as T => t2, t2.t, "\t\\\t");
	assert_de!(T3 as T => t3, t3.t, "");
}

#[derive(Debug, Deserialize)]
struct V {
	t: (String, u32, String),
}

const V1: &str = r#"
t = [ "test"20 "test" ]
"#;

const V2: &str = r#"
t = ["test",20,"test"]
"#;

const V3: &str = r#"
t = [ "test" 20 "test" ]
"#;

#[derive(Debug, Deserialize)]
struct W {
	w: Vec<String>,
}

const W1: &str = r#"
w = [ "one""two""three" ]
"#;

const W2: &str = r#"
w = ["one","two","three"]
"#;

const W3: &str = r#"
w = [ "one" "two" "three" ]
"#;

#[test]
fn delim() {
	assert_err!(
		V1 as V,
		ErrorCode::ExpectedDelimiter('2'),
		Span::new(
			Position {
				line: 2,
				col: 13,
				index: 13
			},
			Position {
				line: 2,
				col: 14,
				index: 14
			}
		)
	);
	assert_err!(
		W1 as W,
		ErrorCode::ExpectedDelimiter('"'),
		Span::new(
			Position {
				line: 2,
				col: 12,
				index: 12
			},
			Position {
				line: 2,
				col: 13,
				index: 13
			}
		)
	);

	assert_de!(V2 as V => v2, v2.t, ("test".into(), 20, "test".into()));
	assert_de!(W2 as W => w2, w2.w, &["one", "two", "three"]);
	assert_de!(V3 as V => v3, v3.t, ("test".into(), 20, "test".into()));
	assert_de!(W3 as W => w3, w3.w, &["one", "two", "three"]);
}
