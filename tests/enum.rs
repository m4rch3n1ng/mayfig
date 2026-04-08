use indexmap::{indexmap, IndexMap};
use mayfig::error::{ErrorCode, Position, Span};
use serde::Deserialize;
use std::collections::HashMap;

mod maytest;

#[derive(Debug, Deserialize)]
struct Tag {
	t: Tagged,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Inline {
	f: i32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Tagged {
	Un,
	Ws(usize),
	Ex(Vec<String>),
	St { code: i32 },
	Tp(u32, String, bool),
	In(Inline),
}

const T1: &str = r#"
t = "ws" [ 4 ]
"#;

const T2: &str = r#"
t = "ex" [ "one" "two" "three" ]
"#;

const T3: &str = r#"
t = "st" {
	code = 200
}
"#;

const T4: &str = r#"
t = "tp" [ 0 "two" false ]
"#;

const T5: &str = r#"
t = "in" [{
	f = -2
}]
"#;

const T6: &str = r#"
t = "in" {
	f = -4
}
"#;

const T7: &str = r#"
t = "ws"
	[ 4 ]
"#;

const T8: &str = r#"
t = "un"
"#;

const T9: &str = r#"
t = "ws" [
	4
]
"#;

#[test]
fn tagged() {
	assert_de!(T1 as Tag => t1, t1.t, Tagged::Ws(4));
	assert_de!(T2 as Tag => t2, t2.t, Tagged::Ex(vec!["one".into(), "two".into(), "three".into()]));
	assert_de!(T3 as Tag => t3, t3.t, Tagged::St { code: 200 });
	assert_de!(T4 as Tag => t4, t4.t, Tagged::Tp(0, "two".into(), false));
	assert_de!(T5 as Tag => t5, t5.t, Tagged::In(Inline { f: -2 }));
	assert_de!(T6 as Tag => t6, t6.t, Tagged::In(Inline { f: -4 }));
	assert_err!(
		T7 as Tag,
		ErrorCode::UnexpectedNewline,
		Span::new(
			Position {
				line: 2,
				col: 9,
				index: 9
			},
			Position {
				line: 3,
				col: 1,
				index: 10
			}
		)
	);

	assert_de!(T8 as Tag => t8, t8.t, Tagged::Un);
	assert_de!(T9 as Tag => t9, t9.t, Tagged::Ws(4));
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum V {
	Unit,
	New(u32),
	Val(u32, u32),
	Str { val: u32 },
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum M<'a> {
	Tag,
	#[serde(borrow)]
	Key(&'a str),
	Val(i32, u32),
	Seq(Vec<&'a str>),
}

#[derive(Debug, Deserialize)]
struct Map<'a> {
	#[serde(borrow)]
	map: HashMap<M<'a>, V>,
}

const M1: &str = r#"
map {
	tag = "val" [ 20 40 ]
	key [ "test" ] = "str" { val = 20 }
	val [ -2 2 ] = "unit"
	seq [ "one" "two" "three" ] = "new" [ 4 ]
	seq [] = "unit"
}
"#;

#[test]
fn map() {
	let m1 = mayfig::from_str::<Map>(M1).unwrap();
	assert_eq!(m1.map.len(), 5);
	assert_eq!(m1.map.get(&M::Tag), Some(&V::Val(20, 40)));
	assert_eq!(m1.map.get(&M::Key("test")), Some(&V::Str { val: 20 }));
	assert_eq!(m1.map.get(&M::Val(-2, 2)), Some(&V::Unit));
	assert_eq!(
		m1.map.get(&M::Seq(vec!["one", "two", "three"])),
		Some(&V::New(4))
	);
	assert_eq!(m1.map.get(&M::Seq(vec![])), Some(&V::Unit));
}

const IGNORED: &str = r#"
gestures {
	swipe = "workspace" [ 2 ]
	swipe [ 4 "left" 250 ] = "workspace" [ 0 ]
}
"#;

#[test]
fn ignored() {
	let _ = mayfig::from_str::<serde::de::IgnoredAny>(IGNORED).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Bind {
	Meta(Key),
	Ctrl(Key),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Key {
	Tab,
	Right,
	Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
	Cycle(Navigation),
	Move(Direction),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Navigation {
	Next,
	Prev,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Direction {
	Left,
	Right,
}

const NESTED: &str = r#"
meta [ "tab" ] = "cycle" [ "next" ]
ctrl [ "tab" ] = "cycle" [ "prev" ]
meta [ "left" ] = "move" [ "left" ]
meta [ "right" ] = "move" [ "right" ]
"#;

#[test]
fn nested() {
	assert_de!(
		NESTED as IndexMap::<Bind, Action>,
		indexmap! {
			Bind::Meta(Key::Tab) => Action::Cycle(Navigation::Next),
			Bind::Ctrl(Key::Tab) => Action::Cycle(Navigation::Prev),
			Bind::Meta(Key::Left) => Action::Move(Direction::Left),
			Bind::Meta(Key::Right) => Action::Move(Direction::Right),
		}
	);
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Movement {
	Maximize(Option<String>),
}

const OPTIONAL: &str = r#"
"one" = "maximize" [ "one" ]
"two" = "maximize" []
"#;

#[test]
fn optional() {
	assert_de!(
		OPTIONAL as IndexMap::<String, Movement>,
		indexmap! {
			"one".to_owned() => Movement::Maximize(Some("one".to_owned())),
			"two".to_owned() => Movement::Maximize(None),
		}
	);
}
