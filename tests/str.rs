use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct S {
	s: String,
	t: u32,
}

const S1: &str = r#"
s = "test"t = 20
"#;

const S2: &str = r#"
"s"="test";t=20
"#;

#[derive(Debug, Deserialize)]
struct St {
	v: Vec<String>,
	t: u32,
}

const S3: &str = r#"
"v"["one","two"] t=20
"#;

#[test]
fn str() {
	let s1 = mayfig::from_str::<S>(S1);
	assert!(s1.is_err());

	let s2 = mayfig::from_str::<S>(S2);
	let s2 = s2.unwrap();
	assert_eq!(s2.s, "test");
	assert_eq!(s2.t, 20);

	let s3 = mayfig::from_str::<St>(S3);
	let s3 = s3.unwrap();
	assert_eq!(&s3.v, &["one", "two"]);
	assert_eq!(s3.t, 20);
}
