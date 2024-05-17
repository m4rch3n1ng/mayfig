use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Txt {
	n: Nested,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Nested {
	t: u32,
}

const TXT: &str = r#"
n {
	t = 20
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
