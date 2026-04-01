use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref};

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
	let o1 = mayfig::from_str::<O>(O1).unwrap();
	assert_eq!(o1.t, Some(20));

	let o2 = mayfig::from_str::<O>(O2).unwrap();
	assert_eq!(o2.t, None);
}

#[derive(Debug, Deserialize)]
struct V {
	t: char,
}

const V1: &str = r#"
t = "c"
"#;

const V2: &str = r#"
t = "cc"
"#;

const V3: &str = r#"
t = ""
"#;

#[test]
fn char() {
	let v1 = mayfig::from_str::<V>(V1).unwrap();
	assert_eq!(v1.t, 'c');

	let e2 = mayfig::from_str::<V>(V2).unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::Custom(_)));
	assert_eq!(
		e2.span(),
		Some(Span::Span(
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
		))
	);

	let e3 = mayfig::from_str::<V>(V3).unwrap_err();
	assert!(matches!(e3.code(), ErrorCode::Custom(_)));
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
#[expect(clippy::bool_assert_comparison)]
fn bool() {
	let b1 = mayfig::from_str::<B>(B1).unwrap();
	assert_eq!(b1.b, true);

	let e2 = mayfig::from_str::<B>(B2).unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::InvalidBool(_)));

	let b3 = mayfig::from_str::<B>(B3).unwrap();
	assert_eq!(b3.b, false);

	let b4 = mayfig::from_str::<B>(B4).unwrap();
	assert_eq!(b4.b, true);
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
	let f1 = mayfig::from_str::<F>(F1).unwrap();
	assert_eq!(f1.f, 2.4);

	let f2 = mayfig::from_str::<F>(F2).unwrap();
	assert_eq!(f2.f, 0.2);

	let e3 = mayfig::from_str::<F>(F3).unwrap_err();
	assert!(matches!(e3.code(), ErrorCode::InvalidNum(_)));

	let e4 = mayfig::from_str::<F>(F4).unwrap_err();
	assert!(matches!(e4.code(), ErrorCode::UnsupportedNaN));

	let f5 = mayfig::from_str::<F>(F5).unwrap();
	assert_eq!(f5.f, f64::INFINITY);

	let f6 = mayfig::from_str::<F>(F6).unwrap();
	assert_eq!(f6.f, f64::NEG_INFINITY);
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Te {
	T1((u32, u32)),
	T2(u32, u32),
}

#[derive(Debug, Deserialize)]
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
}
