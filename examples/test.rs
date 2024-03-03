use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Sub {
	nested: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Txt {
	abcd: f64,
	test: Vec<u8>,
	sub: Sub,
}

const TXT: &str = r#"
abcd = 0.5
test [ 1 2 3 ];
sub {
	nested = -123
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT);
	match t {
		Ok(t) => println!("t: {:?}", t),
		Err(e) => println!("e: {}", e),
	}
}
