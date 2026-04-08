use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref};

mod maytest;

#[derive(Debug, Deserialize)]
struct N<'a> {
	v: I<(u32, i32)>,
	#[serde(borrow)]
	t: I<&'a str>,
}

#[derive(Debug, Deserialize)]
struct I<T>(T);

impl<T> Deref for I<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

const N1: &str = r#"
v = [ 20 -20 ]
t = "test"
"#;

#[test]
fn newtype() {
	let t1 = mayfig::from_str::<N>(N1).unwrap();
	assert_eq!(*t1.v, (20, -20));
	assert_eq!(*t1.t, "test");
}

#[derive(Debug, Deserialize)]
struct O {
	t: Option<u32>,
}

const O1: &str = r#"
t = 20
"#;

const O2: &str = r#""#;

#[test]
fn option() {
	assert_de!(O1 as O => o1, o1.t, Some(20));
	assert_de!(O2 as O => o2, o2.t, None);
}

#[derive(Debug, Deserialize)]
struct V {
	t: char,
}

const V1: &str = r#"
t = "c"
"#;

const V2: &str = r#"
t = ""
"#;

const V3: &str = r#"
t = "cc"
"#;

#[test]
fn char() {
	assert_de!(V1 as V => v1, v1.t, 'c');
	assert_err!(V2 as V, ErrorCode::Custom(_));
	assert_err!(
		V3 as V,
		ErrorCode::Custom(_),
		Span::new(
			Position {
				line: 2,
				col: 5,
				index: 5
			},
			Position {
				line: 2,
				col: 9,
				index: 9
			}
		)
	);
}

#[derive(Debug, Deserialize)]
struct B {
	b: bool,
}

const B1: &str = r#"
b = true
"#;

const B2: &str = r#"
b = damn
"#;

const B3: &str = r#"
b = False
"#;

const B4: &str = r#"
b = TRUE
"#;

#[test]
fn bool() {
	assert_de!(B1 as B => b1, b1.b, true);
	assert_err!(B2 as B, ErrorCode::InvalidBool(_));
	assert_de!(B3 as B => b3, b3.b, false);
	assert_de!(B4 as B => b4, b4.b, true);
}

#[derive(Debug, Deserialize)]
struct F {
	f: f64,
}

const F1: &str = r#"
f = 2.4
"#;

const F2: &str = r#"
f = .2
"#;

const F3: &str = r#"
f = .
"#;

const F4: &str = r#"
f = .nAn
"#;

const F5: &str = r#"
f = +.inf
"#;

const F6: &str = r#"
f = -.inf
"#;

#[test]
fn f64() {
	assert_de!(F1 as F => f1, f1.f, 2.4);
	assert_de!(F2 as F => f2, f2.f, 0.2);
	assert_err!(F3 as F, ErrorCode::InvalidNum(_));
	assert_err!(F4 as F, ErrorCode::UnsupportedNaN);
	assert_de!(F5 as F => f5, f5.f, f64::INFINITY);
	assert_de!(F6 as F => f6, f6.f, f64::NEG_INFINITY);
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Te {
	T1((u32, u32)),
	T2(u32, u32),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct T {
	t: (u32, u32),
	m: HashMap<Te, Te>,
}

const T1: &str = r#"
t = [ 0, 1, ]
m {
	t1 [ 0, 1, ] = "t1" [ 2, 3, ]
	t2 [ 0, 1, ] = "t2" [ 2, 3, ]
}
"#;

#[test]
fn tuple() {
	let t1 = mayfig::from_str::<T>(T1).unwrap();
	assert_eq!(t1.t, (0, 1));

	assert_eq!(t1.m.len(), 2);
	assert_eq!(t1.m.get(&Te::T1((0, 1))), Some(&Te::T1((2, 3))));
	assert_eq!(t1.m.get(&Te::T2(0, 1)), Some(&Te::T2(2, 3)));

	let t2 = mayfig::from_reader::<_, T>(std::io::Cursor::new(T1)).unwrap();
	assert_eq!(t1, t2);
}
