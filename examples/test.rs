use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct Txt {
	v: f64,
}

const TXT: &str = r#"
v = +.5
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
