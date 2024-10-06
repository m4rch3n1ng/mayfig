use serde_derive::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
struct Test {
	test: String,
	val: u32,
	nested: Nested,
	map: BTreeMap<usize, String>,
}

#[derive(Debug, Serialize)]
struct Nested {
	thing: Vec<u8>,
}

fn main() {
	let value = Test {
		test: "test".to_owned(),
		val: 20,
		nested: Nested {
			thing: vec![0, 1, 2, 3, 4],
		},
		map: BTreeMap::from([
			(0, "zero".to_owned()),
			(2, "two".to_owned()),
			(4, "four".to_owned()),
		]),
	};

	let test = mayfig::to_string(&value).unwrap();
	print!("{}", test);
}
