use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct R {
	t: u64,
	v: (i64, i64, i64),
	s: String,
}

const R1: &str = r#"
t = 1
v [ -1 0 1 ]
s = "str"
"#;

#[test]
fn reader() {
	let b1 = R1.as_bytes();
	let r1: Result<R, _> = mayfig::from_reader(b1);
	let r1 = r1.unwrap();

	assert_eq!(r1.t, 1);
	assert_eq!(r1.v, (-1, 0, 1));
	assert_eq!(r1.s, "str");
}
