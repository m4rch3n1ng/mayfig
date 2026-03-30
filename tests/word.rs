use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};

const THING: &str = r#"_test = 0
a+b+c = 1
"whä" = 2
"#;

#[test]
fn word() {
	let val = indexmap! {
		"_test".to_owned() => 0,
		"a+b+c".to_owned() => 1,
		"whä".to_owned() => 2,
	};

	let de = mayfig::from_str::<IndexMap<String, u8>>(THING).unwrap();
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
