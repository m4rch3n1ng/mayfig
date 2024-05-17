use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Txt {
	m: HashMap<String, u32>,
}

const TXT: &str = r#"
m {
	t = 20
	v = 40
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
