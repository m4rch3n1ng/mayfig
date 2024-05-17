use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct T {
	n: N,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct N {
	t: u32,
}

const T1: &str = r#"
n {
	t = 20
}
"#;

const T2: &str = r#"{
	n { t = 20
	}
}"#;

const T3: &str = r#"
n = {
	t = 20
}
"#;

#[test]
fn nested_struct() {
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.n, N { t: 20 });

	let t2 = mayfig::from_str::<T>(T2);
	let t2 = t2.unwrap();
	assert_eq!(t2.n, N { t: 20 });

	let t3 = mayfig::from_str::<T>(T3);
	let t3 = t3.unwrap();
	assert_eq!(t3.n, N { t: 20 });
}

#[derive(Debug, Deserialize)]
struct M<'a, T> {
	#[serde(borrow)]
	m: HashMap<&'a str, T>,
}

const M1: &str = r#"
m {
	v = 20
	"t" = 40
	f = 0
}
"#;

const M2: &str = r#"
m = {
	v = "one"
	t = "two"
	"f" = "three"
}
"#;

#[test]
fn map() {
	let m1 = mayfig::from_str::<M<u32>>(M1);
	let m1 = m1.unwrap();
	assert_eq!(m1.m.len(), 3);
	assert_eq!(m1.m.get("v"), Some(&20));
	assert_eq!(m1.m.get("t"), Some(&40));
	assert_eq!(m1.m.get("f"), Some(&0));

	let m2 = mayfig::from_str::<M<&str>>(M2);
	let m2 = m2.unwrap();
	assert_eq!(m2.m.len(), 3);
	assert_eq!(m2.m.get("v"), Some(&"one"));
	assert_eq!(m2.m.get("t"), Some(&"two"));
	assert_eq!(m2.m.get("f"), Some(&"three"));
}
