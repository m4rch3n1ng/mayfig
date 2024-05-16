use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Txt {
	test: u32,
}

const TXT: &str = r#"
test = 20
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
