use mayfig::{value::Map, Value};

mod maytest;

const V1: &str = r#"
map {
	v = .inf
	t = [ 1 2 3, ]
}
str = "string"
"#;

const TAG: &str = r#"
tag [ 0 "test" ] {
	v = 0
	t = [ "test" ]
}
[ 0 1 2 ] = "what" [ 2 ]
"#;

#[test]
fn value() {
	let v1 = Value::Map(Map::from([
		(
			Value::String("map".to_owned()),
			Value::Map(Map::from([
				(
					Value::String("v".to_owned()),
					Value::try_from(f64::INFINITY).unwrap(),
				),
				(
					Value::String("t".to_owned()),
					(Value::Seq(vec![Value::from(1), Value::from(2), Value::from(3)])),
				),
			])),
		),
		(
			Value::String("str".to_owned()),
			Value::String("string".to_owned()),
		),
	]));
	assert_de!(V1 as Value, v1);

	let t1 = Value::Map(Map::from([
		(
			Value::Tagged(
				"tag".to_owned(),
				Vec::from([Value::from(0), Value::from("test")]),
			),
			Value::Map(Map::from([
				(Value::from("v"), Value::from(0)),
				(Value::from("t"), Value::Seq(vec![Value::from("test")])),
			])),
		),
		(
			Value::Seq(vec![Value::from(0), Value::from(1), Value::from(2)]),
			(Value::Tagged("what".to_owned(), vec![Value::from(2)])),
		),
	]));
	assert_de!(TAG as Value, t1);
}

const IGNORED: &str = r#"
windowrules {
	app_id [ "org.gnome.Nautilus" ] {
		floating = true
	}
}

binds {
	"mod q" = "quit"
	"mod t" = "spawn" [ "kitty" ]
	"mod n" = "spawn" [ "firefox" "--private-window" ]
}
"#;

#[test]
fn ignored() {
	let _ = mayfig::from_str::<serde::de::IgnoredAny>(IGNORED).unwrap();
}
