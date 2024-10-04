const TXT: &str = r#"
map {
	test = true
	val = +1.2
}
"#;

fn main() {
	let value = mayfig::from_str::<mayfig::Value>(TXT);
	match value {
		Ok(value) => println!("{:#?}", value),
		Err(err) => println!("{}", err),
	}
}
