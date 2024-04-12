use serde_derive::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)]
struct M {
	map: HashMap<String, u8>,
}

const M: &str = r#"
map {
	k0 = 0
	k1 = 1
	k2 = 2
}
"#;

#[test]
fn map() {
	let m = mayfig::from_str::<M>(M);
	let m = m.unwrap();

	assert_eq!(m.map.len(), 3);
	assert_eq!(m.map.get("k0"), Some(&0));
	assert_eq!(m.map.get("k1"), Some(&1));
	assert_eq!(m.map.get("k2"), Some(&2));
}

#[derive(Debug, Deserialize)]
struct M1 {
	map: HashMap<u8, u8>,
}

const M1: &str = r#"
map {
	0 = 0
	1 = 1
	2 = 2
}
"#;

#[derive(Debug, Deserialize)]
struct M2<'a> {
	t: HashMap<( u8, u8 ), ( u8, u8)>,
	#[serde(borrow)]
	v: BTreeMap<Vec<&'a str>, Vec<&'a str>>
}

const M2: &str = r#"
t {
	[ 0 0 ] [ 1 1 ]
	[ 0 1 ] = [ 1 0 ]
}

v {
	[ "ctrl" "tab" ] [ "switch" ]
	[ "ctrl" "shift" "t" ] [ "exec" "terminal" ]
}
"#;

#[test]
fn weird_keys() {
	let m1 = mayfig::from_str::<M1>(M1);
	let m1 = m1.unwrap();

	assert_eq!(m1.map.len(), 3);
	assert_eq!(m1.map.get(&0), Some(&0));
	assert_eq!(m1.map.get(&1), Some(&1));
	assert_eq!(m1.map.get(&2), Some(&2));

	let m2 = mayfig::from_str::<M2>(M2);
	let m2 = m2.unwrap();

	assert_eq!(m2.t.len(), 2);
	assert_eq!(m2.t.get(&(0, 0)), Some(&(1, 1)));
	assert_eq!(m2.t.get(&(0, 1)), Some(&(1, 0)));

	assert_eq!(m2.v.len(), 2);
	assert_eq!(m2.v.get(&["ctrl", "tab"] as &[_]), Some(&vec!["switch"]));
	assert_eq!(m2.v.get(&["ctrl", "shift", "t"] as &[_]), Some(&vec!["exec", "terminal"]));
}
