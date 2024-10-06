use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct DeSer1 {
	test: String,
	val: u32,
	nested: Nested,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Nested {
	thing: Vec<u8>,
}

const DE_SER_1: &str = r#"test = "string"
val = 0
nested {
	thing = [ 0 1 2 3 ]
}
"#;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Action {
	Close,
	Spawn(String),
	Thing { thing: String, val: usize },
	Nest(Nested),
	Move(String, usize),
	Seq(Vec<char>),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct DeSer2 {
	binds: IndexMap<String, Action>,
}

const DE_SER_2: &str = r#"binds {
	"mod q" = "close"
	"mod t" = "spawn" [ "kitty" ]
	"mod w" = "thing" {
		thing = "thing"
		val = 4
	}
	"mod n" = "nest" {
		thing = [ 4 4 4 4 ]
	}
	"mod s" = "seq" [ "a" "c" ]
	"mod 0" = "move" [ "workspace" 0 ]
}
"#;

#[test]
fn ser() {
	let ref1 = DeSer1 {
		test: "string".to_owned(),
		val: 0,
		nested: Nested {
			thing: vec![0, 1, 2, 3],
		},
	};

	let de1 = mayfig::from_str::<DeSer1>(DE_SER_1).unwrap();
	assert_eq!(de1, ref1);

	let ser1 = mayfig::to_string(&de1).unwrap();
	assert_eq!(ser1, DE_SER_1);

	let ref2 = DeSer2 {
		binds: IndexMap::from([
			("mod q".to_owned(), Action::Close),
			("mod t".to_owned(), Action::Spawn("kitty".to_owned())),
			(
				"mod w".to_owned(),
				Action::Thing {
					thing: "thing".to_owned(),
					val: 4,
				},
			),
			(
				"mod n".to_owned(),
				Action::Nest(Nested {
					thing: vec![4, 4, 4, 4],
				}),
			),
			("mod s".to_owned(), Action::Seq(vec!['a', 'c'])),
			("mod 0".to_owned(), Action::Move("workspace".to_owned(), 0)),
		]),
	};

	let de2 = mayfig::from_str::<DeSer2>(DE_SER_2).unwrap();
	assert_eq!(de2, ref2);

	let ser2 = mayfig::to_string(&de2).unwrap();
	assert_eq!(ser2, DE_SER_2);
}
