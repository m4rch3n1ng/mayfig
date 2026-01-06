use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;

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
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.t, "test");

	let t2 = mayfig::from_str::<T>(T2);
	let t2 = t2.unwrap();
	assert_eq!(t2.t, "\t\\\t");

	let t3 = mayfig::from_str::<T>(T3);
	let t3 = t3.unwrap();
	assert_eq!(t3.t, "");
}

#[derive(Debug, Deserialize)]
struct V<'a> {
	#[serde(borrow)]
	t: (&'a str, u32, &'a str),
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
struct W<'a> {
	#[serde(borrow)]
	w: Vec<&'a str>,
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
	let v1 = mayfig::from_str::<V>(V1);
	let e1 = v1.unwrap_err();
	assert!(matches!(e1.code(), ErrorCode::ExpectedDelimiter(_)));
	assert_eq!(
		e1.span(),
		Some(Span::Point(Position {
			line: 2,
			col: 13,
			index: 13
		}))
	);

	let w1 = mayfig::from_str::<W>(W1);
	let e2 = w1.unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::ExpectedDelimiter(_)));
	assert_eq!(
		e2.span(),
		Some(Span::Point(Position {
			line: 2,
			col: 12,
			index: 12
		}))
	);

	let v2 = mayfig::from_str::<V>(V2);
	let v2 = v2.unwrap();
	assert_eq!(v2.t, ("test", 20, "test"));

	let w2 = mayfig::from_str::<W>(W2);
	let w2 = w2.unwrap();
	assert_eq!(&w2.w, &["one", "two", "three"]);

	let v3 = mayfig::from_str::<V>(V3);
	let v3 = v3.unwrap();
	assert_eq!(v3.t, ("test", 20, "test"));

	let w3 = mayfig::from_str::<W>(W3);
	let w3 = w3.unwrap();
	assert_eq!(&w3.w, &["one", "two", "three"]);
}
