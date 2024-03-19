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

const R2: &str = r#"
s = "str"t = 1 v [ -1 0 1 ]
"#;

#[derive(Debug, Deserialize)]
struct N1 {
	t: i32,
	n: N2,
}

#[derive(Debug, Deserialize)]
struct N2 {
	v_v: Vec<u8>,
	w: bool,
	s: String,
}

const R3: &str = r#"
t = -4
n {
	v_v [ 1 2 3 ]
	w = no
	s = "s\t\"r"
}
"#;

#[test]
#[allow(clippy::bool_assert_comparison)]
fn reader() {
	let b1 = R1.as_bytes();
	let r1 = mayfig::from_reader::<_, R>(b1);
	let r1 = r1.unwrap();

	assert_eq!(r1.t, 1);
	assert_eq!(r1.v, (-1, 0, 1));
	assert_eq!(r1.s, "str");

	let b2 = R2.as_bytes();
	let r2 = mayfig::from_reader::<_, R>(b2);
	assert!(r2.is_err());

	let b3 = R3.as_bytes();
	let r3 = mayfig::from_reader::<_, N1>(b3);
	let r3 = r3.unwrap();

	assert_eq!(r3.t, -4);
	assert_eq!(r3.n.v_v, vec![1, 2, 3]);
	assert_eq!(r3.n.w, false);
	assert_eq!(r3.n.s, "s\t\"r");
}
