use std::ops::Deref;

use serde_derive::Deserialize;

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
