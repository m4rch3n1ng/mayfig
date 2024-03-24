use serde::{de::DeserializeOwned, ser::Serialize};
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug};

fn twoway<T: DeserializeOwned + Serialize + Debug + PartialEq>(t: T, s: &'static str) {
	// string ser
	let ser = mayfig::to_string(&t);
	let ser = ser.unwrap();
	assert_eq!(s, ser);

	// string de
	let de = mayfig::from_str::<T>(&ser);
	let de = de.unwrap();
	assert_eq!(t, de);

	// reader de
	let de_r = mayfig::from_reader::<_, T>(s.as_bytes());
	let de_r = de_r.unwrap();
	assert_eq!(t, de_r);
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

#[derive(Debug, Serialize)]
struct S3 {
	m: BTreeMap<u64, u64>,
}

const S3: &str = r#"m {
  0 = 0
  1 = 1
  2 = 2
}
"#;

#[test]
fn indent() {
	let m = BTreeMap::from([(0, 0), (1, 1), (2, 2)]);
	let s3 = S3 { m };

	let mut st = String::new();
	let mut ser = mayfig::Serializer::with_indent(&mut st, "  ");
	s3.serialize(&mut ser).unwrap();

	assert_eq!(S3, st)
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct S4 {
	s: String,
	t: T4,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct T4 {
	v: (u64, u64),
	m: BTreeMap<u64, u64>,
}

const S4: &str = r#"s = "test"
t {
	v [ 1 2 ]
	m {
		0 = 0
		1 = 1
	}
}
"#;

#[test]
fn twolevel() {
	let m = BTreeMap::from([(0, 0), (1, 1)]);
	let t = T4 { v: (1, 2), m };
	let s = S4 {
		s: "test".to_owned(),
		t,
	};

	twoway(s, S4);
}
