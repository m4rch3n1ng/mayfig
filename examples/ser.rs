use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct T {
	v: Vec<u64>,
	s: String,
}

fn main() {
	let t = T {
		v: vec![1, 2, 4],
		s: "test".into(),
	};

	let t = mayfig::to_string(&t);
	match t {
		Ok(t) => println!("{}", t),
		Err(e) => println!("{}", e),
	}
}
