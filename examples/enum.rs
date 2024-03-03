use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
enum E {
	Id { id: u64 },
	St { st: u64 },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct T {
	e: E,
}

const TXT: &str = r#"
e {
	Id {
		id = 4
	}
}
"#;

fn main() {
	let t = mayfig::from_str::<T>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
