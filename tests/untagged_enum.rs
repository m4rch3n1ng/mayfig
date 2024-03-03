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
