use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum V {
	Val(u32, u32),
	Str { val: u32 },
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum M {
	Tag,
	Key(String),
	Val(u32, u32),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Txt {
	t: HashMap<M, V>,
}

const TXT: &str = r#"
t {
	tag = "val" [ 20 40 ]
	key [ "test" ] = "str" { val = 20 }
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
