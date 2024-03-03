use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Sub {
	nested: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Txt {
	abcd: u64,
	test: Vec<u8>,
	sub: Sub,
}

const TXT: &str = r#"
abcd = 123
test [ 1 2 3 ];
sub {
	nested = 123
}
"#;

fn main() {
	let t = mayfig::from_str::<Txt>(TXT).unwrap();
	println!("t {:?}", t);
}
