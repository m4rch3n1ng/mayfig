use serde_derive::Deserialize;
use std::{borrow::Cow, collections::HashMap};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Sub {
	nested: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Txt<'a> {
	abcd: Option<f64>,
	#[serde(with = "serde_bytes")]
	test: Vec<u8>,
	sub: Sub,
	map: HashMap<String, u8>,
	#[serde(with = "serde_bytes")]
	#[serde(borrow)]
	str: Cow<'a, [u8]>,
}

const TXT: &str = r#"
abcd = 2.0
test = [ 1 2 3 ]
str = "test"

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
