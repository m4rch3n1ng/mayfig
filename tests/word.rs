use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};

mod maytest;

const THING: &str = r#"_test = 0
a+b+c = 1
"whä" = 2
* = 3
let* = 4
"#;

#[test]
fn word() {
	let de = assert_de!(
		THING as IndexMap<String, u8>,
		indexmap! {
			"_test".to_owned() => 0,
			"a+b+c".to_owned() => 1,
			"whä".to_owned() => 2,
			"*".to_owned() => 3,
			"let*".to_owned() => 4,
		}
	);

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
	let de = assert_de!(
		UNDERSCORES as IndexMap::<Underscores, u8>,
		indexmap! { Underscores::Variant(0) => 0, Underscores::Variant(1) => 1 }
	);

	let ser = mayfig::to_string(&de).unwrap();
	assert_eq!(ser, UNDERSCORES);
}
