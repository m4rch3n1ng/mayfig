use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
enum E {
	Id(u64),
	St,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct T {
	e: E,
}

const TXT: &str = r#"
e {
	Id = 4
}
"#;

fn main() {
	let t = mayfig::from_str::<T>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
