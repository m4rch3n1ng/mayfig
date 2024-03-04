use serde_derive::Deserialize;
use std::collections::HashMap;

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

#[test]
fn u8map() {
	let m = mayfig::from_str::<M1>(M1);
	let m = m.unwrap();

	assert_eq!(m.map.len(), 3);
	assert_eq!(m.map.get(&0), Some(&0));
	assert_eq!(m.map.get(&1), Some(&1));
	assert_eq!(m.map.get(&2), Some(&2));
}
