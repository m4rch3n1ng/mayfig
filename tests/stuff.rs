use mayfig::error::Error;
use serde_derive::Deserialize;
use std::ops::Deref;

#[derive(Debug, Deserialize)]
struct T<'a> {
	v: N<(u32, i32)>,
	#[serde(borrow)]
	t: N<&'a str>,
}

#[derive(Debug, Deserialize)]
struct N<T>(T);

impl<T> Deref for N<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

const T1: &str = r#"
v = [ 20 -20 ]
t = "test"
"#;

#[test]
fn newtype() {
	let t1 = mayfig::from_str::<T>(T1);
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
#[allow(clippy::bool_assert_comparison)]
fn bool() {
	let b1 = mayfig::from_str::<B>(B1);
	let b1 = b1.unwrap();
	assert_eq!(b1.b, true);

	let b2 = mayfig::from_str::<B>(B2);
	let e2 = b2.unwrap_err();
	assert!(matches!(e2, Error::InvalidBool(_)));

	let b3 = mayfig::from_str::<B>(B3);
	let b3 = b3.unwrap();
	assert_eq!(b3.b, false);

	let b4 = mayfig::from_str::<B>(B4);
	let b4 = b4.unwrap();
	assert_eq!(b4.b, true);
}
