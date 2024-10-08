use mayfig::error::ErrorCode;
use serde_derive::Deserialize;
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
	let t1 = mayfig::from_str::<N>(N1);
	let t1 = t1.unwrap();
	assert_eq!(*t1.v, (20, -20));
	assert_eq!(*t1.t, "test")
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
	let o1 = mayfig::from_str::<O>(O1);
	let o1 = o1.unwrap();
	assert_eq!(o1.t, Some(20));

	let o2 = mayfig::from_str::<O>(O2);
	let o2 = o2.unwrap();
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
	let v1 = mayfig::from_str::<V>(V1);
	let v1 = v1.unwrap();
	assert_eq!(v1.t, 'c');

	let v2 = mayfig::from_str::<V>(V2);
	assert!(v2.is_err());

	let v3 = mayfig::from_str::<V>(V3);
	assert!(v3.is_err());
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
	let b1 = mayfig::from_str::<B>(B1);
	let b1 = b1.unwrap();
	assert_eq!(b1.b, true);

	let b2 = mayfig::from_str::<B>(B2);
	let e2 = b2.unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::InvalidBool(_)));

	let b3 = mayfig::from_str::<B>(B3);
	let b3 = b3.unwrap();
	assert_eq!(b3.b, false);

	let b4 = mayfig::from_str::<B>(B4);
	let b4 = b4.unwrap();
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
	let f1 = mayfig::from_str::<F>(F1);
	let f1 = f1.unwrap();
	assert_eq!(f1.f, 2.4);

	let f2 = mayfig::from_str::<F>(F2);
	let f2 = f2.unwrap();
	assert_eq!(f2.f, 0.2);

	let f3 = mayfig::from_str::<F>(F3);
	let e3 = f3.unwrap_err();
	assert!(matches!(e3.code(), ErrorCode::InvalidNum(_)));

	let f4 = mayfig::from_str::<F>(F4);
	let e4 = f4.unwrap_err();
	assert!(matches!(e4.code(), ErrorCode::UnsupportedNaN));

	let f5 = mayfig::from_str::<F>(F5);
	let f5 = f5.unwrap();
	assert_eq!(f5.f, f64::INFINITY);

	let f6 = mayfig::from_str::<F>(F6);
	let f6 = f6.unwrap();
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
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.t, (0, 1));

	assert_eq!(t1.m.len(), 2);
	assert_eq!(t1.m.get(&Te::T1((0, 1))), Some(&Te::T1((2, 3))));
	assert_eq!(t1.m.get(&Te::T2(0, 1)), Some(&Te::T2(2, 3)));
}
