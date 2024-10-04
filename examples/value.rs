const TXT: &str = r#"
map {
	test [ "test" 20 ] = true
	val = "tag" [ +1.2 ]
}
vec = [ "test" -24 ]
"#;

fn main() {
	let value = mayfig::from_str::<mayfig::Value>(TXT);
	match value {
		Ok(value) => println!("{:#?}", value),
		Err(err) => println!("{}", err),
	}
}
