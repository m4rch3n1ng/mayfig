use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct T {
	t: u64,
	v: Vec<u64>,
}

fn main() {
	let t = T { t: 20, v: vec![1, 2, 4] };
	let t = mayfig::ser::to_string(&t);
	match t {
		Ok(t) => println!("{}", t),
		Err(e) => println!("{}", e),
	}
}