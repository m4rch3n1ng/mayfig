use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct Thing {
	_test: u8,
	#[serde(rename = "a+b+c")]
	abc: u8,
	whä: u8,
}

const THING: &str = r#"_test = 0
a+b+c = 1
"whä" = 2
"#;

#[test]
fn word() {
	let val = Thing {
		_test: 0,
		abc: 1,
		whä: 2,
	};

	let de = mayfig::from_str::<Thing>(THING).unwrap();
	assert_eq!(de, val);

	let ser = mayfig::to_string(&de).unwrap();
	assert_eq!(ser, THING);
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum Underscores {
	#[serde(rename = "_variant")]
	Variant(u8),
}

const UNDERSCORES: &str = r#"_variant [ 0 ] = 0
_variant [ 1 ] = 1
"#;

#[test]
fn underscores() {
	let val = indexmap! { Underscores::Variant(0) => 0, Underscores::Variant(1) => 1 };

	let de = mayfig::from_str::<IndexMap<Underscores, u8>>(UNDERSCORES).unwrap();
	assert_eq!(de, val);

	let ser = mayfig::to_string(&de).unwrap();
	assert_eq!(ser, UNDERSCORES);
}
