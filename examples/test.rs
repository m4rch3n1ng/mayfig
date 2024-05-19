use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum T {
	Val(u32, u32),
	Str { val: u32 },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Txt {
	t: T,
}

const TXT: &str = r#"
t = "val" [ 20 40 ]
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
