use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Action {
	Close,
	Spawn(String),
	Move(String, usize),
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
struct Test {
	bind: IndexMap<String, Action>,
	window: IndexMap<Match, WindowRule>,
}

fn main() {
	let value = Test {
		bind: IndexMap::from([
			("mod q".to_owned(), Action::Close),
			("mod t".to_owned(), Action::Spawn("kitty".to_owned())),
			("mod w".to_owned(), Action::Move("workspace".to_owned(), 0)),
		]),
		window: IndexMap::from([
			(
				Match::Thing("chromium".to_owned(), "Save File".to_owned()),
				WindowRule {
					floating: Some(true),
					size: Some((700, 700)),
					opacity: Some(0.4),
				},
			),
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
				Match::None,
				WindowRule {
					floating: Some(false),
					size: None,
					opacity: None,
				},
			),
		]),
	};

	let test = mayfig::to_string(&value).unwrap();
	print!("{}", test);

	let back = mayfig::from_str::<Test>(&test).unwrap();
	assert_eq!(value, back);
}
