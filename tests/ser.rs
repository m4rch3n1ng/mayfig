use serde::{de::DeserializeOwned, ser::Serialize};
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug};

fn twoway<T: DeserializeOwned + Serialize + Debug + PartialEq>(t: T, s: &'static str) {
	let ser = mayfig::ser::to_string(&t);
	let ser = ser.unwrap();
	assert_eq!(s, ser);

	let de = mayfig::from_str::<T>(&ser);
	let de = de.unwrap();
	assert_eq!(t, de);
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct S1 {
	v: Vec<u64>,
	t: u64,
	s: String,
}

const S1: &str = r#"v [ 1 2 3 4 ]
t = 5
s = "test"
"#;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct S2 {
	v: (i8, u8, u8),
	m: BTreeMap<String, u64>,
}

const S2: &str = r#"v [ -1 0 1 ]
m {
	k0 = 0
	k1 = 1
	k2 = 2
}

"#;

#[test]
fn test() {
	let s1 = S1 {
		v: vec![1, 2, 3, 4],
		t: 5,
		s: "test".into(),
	};
	twoway(s1, S1);

	let m = BTreeMap::from([("k0".into(), 0), ("k1".into(), 1), ("k2".into(), 2)]);
	let s2 = S2 { v: (-1, 0, 1), m };
	twoway(s2, S2);
}
