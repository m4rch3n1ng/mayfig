use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

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
	n { t = 20 }
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
	t_k = 21
	mod+a = 41
	alt-a = 22
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
	assert_eq!(m1.m.len(), 6);
	assert_eq!(m1.m.get("v"), Some(&20));
	assert_eq!(m1.m.get("t"), Some(&40));
	assert_eq!(m1.m.get("f"), Some(&0));
	assert_eq!(m1.m.get("t_k"), Some(&21));
	assert_eq!(m1.m.get("mod+a"), Some(&41));
	assert_eq!(m1.m.get("alt-a"), Some(&22));

	let m2 = mayfig::from_str::<M<&str>>(M2);
	let m2 = m2.unwrap();
	assert_eq!(m2.m.len(), 3);
	assert_eq!(m2.m.get("v"), Some(&"one"));
	assert_eq!(m2.m.get("t"), Some(&"two"));
	assert_eq!(m2.m.get("f"), Some(&"three"));
}

#[derive(Debug, Deserialize)]
struct S<'a> {
	t: HashMap<(u8, u8), (u8, u8)>,
	#[serde(borrow)]
	v: BTreeMap<Vec<&'a str>, Vec<&'a str>>,
}

const S: &str = r#"
t {
	[ 0 0 ] = [ 1 1 ]
	[ 0 1 ] = [ 1 0 ]
}

v {
	[ "ctrl" "tab" ] = [ "switch" ]
	[ "ctrl" "shift" "t" ] = [ "exec" "terminal" ]
}
"#;

#[test]
fn weird_keys() {
	let m2 = mayfig::from_str::<S>(S);
	let m2 = m2.unwrap();

	assert_eq!(m2.t.len(), 2);
	assert_eq!(m2.t.get(&(0, 0)), Some(&(1, 1)));
	assert_eq!(m2.t.get(&(0, 1)), Some(&(1, 0)));

	assert_eq!(m2.v.len(), 2);
	assert_eq!(m2.v.get(&["ctrl", "tab"] as &[_]), Some(&vec!["switch"]));
	assert_eq!(
		m2.v.get(&["ctrl", "shift", "t"] as &[_]),
		Some(&vec!["exec", "terminal"])
	);
}
