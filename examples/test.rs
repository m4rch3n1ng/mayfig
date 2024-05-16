use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Txt {}

const TXT: &str = r#""#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
