use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Txt {
	#[allow(dead_code)]
	abcd: u64,
	#[allow(dead_code)]
	test: Vec<u8>,
}

const TXT: &str = r#"
abcd = 123
test [ 1 2 3 ]
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT).unwrap();
	println!("t {:?}", t);
}
