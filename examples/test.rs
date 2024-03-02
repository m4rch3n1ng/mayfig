use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Txt {
	#[allow(dead_code)]
	abcd: u64,
	#[allow(dead_code)]
	test: u8,
}

const TXT: &str = r#"
abcd = 123
test = 123
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT).unwrap();
	println!("t {:?}", t);
}
