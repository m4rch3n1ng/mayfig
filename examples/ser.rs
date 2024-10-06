use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Action {
	Close,
	Spawn(String),
	Thing { thing: String, val: usize },
	Nest(Nested),
	Seq(Vec<char>),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Test {
	test: String,
	val: u32,
	nested: Nested,
	map: BTreeMap<usize, String>,
	bind: HashMap<String, Action>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
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
		bind: HashMap::from([
			("mod q".to_owned(), Action::Close),
			("mod t".to_owned(), Action::Spawn("kitty".to_owned())),
			(
				"mod w".to_owned(),
				Action::Thing {
					thing: "t".to_owned(),
					val: 2,
				},
			),
			(
				"mod n".to_owned(),
				Action::Nest(Nested { thing: vec![0, 1] }),
			),
			("mod s".to_owned(), Action::Seq(vec!['a', 'c'])),
		]),
	};

	let test = mayfig::to_string(&value).unwrap();
	print!("{}", test);

	let back = mayfig::from_str::<Test>(&test).unwrap();
	assert_eq!(value, back);
}
