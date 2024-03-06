use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct N {
	n: NT,
}

#[derive(Debug, Deserialize)]
struct NT(u32);

const N1: &str = r#"
n = 4
"#;

#[test]
fn newtype() {
	let n1 = mayfig::from_str::<N>(N1).unwrap();
	assert_eq!(n1.n.0, 4);
}

#[derive(Debug, Deserialize)]
struct O {
	o: Option<u64>,
}

const O1: &str = r#""#;

const O2: &str = r#"
o = 4
"#;

#[test]
fn option() {
	let o1 = mayfig::from_str::<O>(O1);
	let o1 = o1.unwrap();
	assert_eq!(o1.o, None);

	let o2 = mayfig::from_str::<O>(O2);
	let o2 = o2.unwrap();
	assert_eq!(o2.o, Some(4));
}
