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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Match {
	Thing(String, String),
	Class(String),
	Title(String),
	None,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct WindowRule {
	#[serde(skip_serializing_if = "Option::is_none", default)]
	floating: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	size: Option<(u32, u32)>,
	#[serde(skip_serializing_if = "Option::is_none", default)]
	opacity: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct DeSer2 {
	binds: IndexMap<String, Action>,
	window: IndexMap<Match, WindowRule>,
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
window {
	class [ "com.system76.CosmicFiles" ] {
		floating = true
		size = [ 1000 700 ]
	}
	title [ "maym ~" ] {
		opacity = 0.6
	}
	thing [ "chromium" "Save File" ] {
		floating = true
		size = [ 700 700 ]
		opacity = 0.4
	}
	none {
		floating = false
	}
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
		window: IndexMap::from([
			(
				Match::Class("com.system76.CosmicFiles".to_owned()),
				WindowRule {
					floating: Some(true),
					size: Some((1000, 700)),
					opacity: None,
				},
			),
			(
				Match::Title("maym ~".to_owned()),
				WindowRule {
					floating: None,
					size: None,
					opacity: Some(0.6),
				},
			),
			(
				Match::Thing("chromium".to_owned(), "Save File".to_owned()),
				WindowRule {
					floating: Some(true),
					size: Some((700, 700)),
					opacity: Some(0.4),
				},
			),
			(
				Match::None,
				WindowRule {
					floating: Some(false),
					size: None,
					opacity: None,
				},
			),
		]),
	};

	let de2 = mayfig::from_str::<DeSer2>(DE_SER_2).unwrap();
	assert_eq!(de2, ref2);

	let ser2 = mayfig::to_string(&de2).unwrap();
	assert_eq!(ser2, DE_SER_2);
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct DeSer3 {
	map: IndexMap<(u8, u8), (u8, u8)>,
}

const DE_SER_3: &str = r#"map {
	[ 0 1 ] = [ 1 0 ]
	[ 1 0 ] = [ 0 1 ]
	[ 2 2 ] = [ 2 2 ]
}
"#;

#[test]
fn more() {
	let ref3 = DeSer3 {
		map: IndexMap::from([((0, 1), (1, 0)), ((1, 0), (0, 1)), ((2, 2), (2, 2))]),
	};

	let de3 = mayfig::from_str::<DeSer3>(DE_SER_3).unwrap();
	assert_eq!(de3, ref3);

	let ser3 = mayfig::to_string(&de3).unwrap();
	assert_eq!(ser3, DE_SER_3);
}
