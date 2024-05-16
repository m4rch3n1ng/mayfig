use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct T {
	t: String,
}

const T1: &str = r#"
t = "test"
"#;

const T2: &str = r#"
t = "\t\\\t"
"#;

const T3: &str = r#"
t = ""
"#;

#[test]
fn string() {
	let t1 = mayfig::from_str::<T>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.t, "test");

	let t2 = mayfig::from_str::<T>(T2);
	let t2 = t2.unwrap();
	assert_eq!(t2.t, "\t\\\t");

	let t3 = mayfig::from_str::<T>(T3);
	let t3 = t3.unwrap();
	assert_eq!(t3.t, "");
}
