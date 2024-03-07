use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum T {
	Id { id: u64 },
	St { st: u64 },
}

const T1: &str = r#"
id = 4
"#;

const T2: &str = r#"
st = 4
"#;

const T3: &str = r#"
ex = 4
"#;

#[test]
fn top() {
	let t1 = mayfig::from_str::<T>(T1);
	assert!(matches!(t1, Ok(T::Id { id: 4 })));

	let t2 = mayfig::from_str::<T>(T2);
	assert!(matches!(t2, Ok(T::St { st: 4 })));

	let t3 = mayfig::from_str::<T>(T3);
	assert!(t3.is_err());
}

const T4: &str = r#"{
	id = 4
}"#;

const T5: &str = r#"{
	st = 4
}"#;

const T6: &str = r#"{
	ex = 4
}"#;

#[test]
fn braces() {
	let t4 = mayfig::from_str::<T>(T4);
	assert!(matches!(t4, Ok(T::Id { id: 4 })));

	let t5 = mayfig::from_str::<T>(T5);
	assert!(matches!(t5, Ok(T::St { st: 4 })));

	let t6 = mayfig::from_str::<T>(T6);
	assert!(t6.is_err())
}

#[derive(Debug, Deserialize)]
struct E {
	t: T,
}

const T7: &str = r#"
t {
	id = 4
}
"#;

const T8: &str = r#"
t {
	st = 4
}
"#;

const T9: &str = r#"
t {
	ex = 4
}
"#;

#[test]
fn nested() {
	let t7 = mayfig::from_str::<E>(T7).unwrap();
	assert!(matches!(t7.t, T::Id { id: 4 }));

	let t8 = mayfig::from_str::<E>(T8).unwrap();
	assert!(matches!(t8.t, T::St { st: 4 }));

	let t9 = mayfig::from_str::<E>(T9);
	assert!(t9.is_err())
}

#[derive(Debug, Deserialize)]
struct S {
	v: V,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum V {
	Seq(u64, u64),
	Sin(u64),
}

const T10: &str = r#"
v = 4
"#;

const T11: &str = r#"
v [ 4, 5 ]
"#;

const T12: &str = r#"
v [ 5 ]
"#;

#[test]
fn tuple() {
	let t10 = mayfig::from_str::<S>(T10);
	let t10 = t10.unwrap();
	assert!(matches!(t10.v, V::Sin(4)));

	let t11 = mayfig::from_str::<S>(T11);
	let t11 = t11.unwrap();
	assert!(matches!(t11.v, V::Seq(4, 5)));

	let t12 = mayfig::from_str::<S>(T12);
	assert!(t12.is_err());
}
