use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Txt {
	#[allow(dead_code)]
	abc: String,
}

const TXT: &str = r#"
abcd = "efgh"
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	println!("t {:?}", t);
}
