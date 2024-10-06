use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct Test {
	test: String,
	val: u32,
}

fn main() {
	let value = Test {
		test: "test".to_owned(),
		val: 20,
	};

	let test = mayfig::to_string(&value).unwrap();
	println!("{}", test);
}
