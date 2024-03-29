use serde_derive::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Sub {
	nested: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Txt {
	abcd: Option<f64>,
	test: Vec<u8>,
	sub: Sub,
	map: HashMap<String, u8>,
	str: String,
}

const TXT: &str = r#"
abcd = 2.0
test = [ 1 2 3 ]
str = "t\\\"est"

sub {
	nested = -123
}

map {
	k0 = 0
	k1 = 1
	k2 = 0
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
