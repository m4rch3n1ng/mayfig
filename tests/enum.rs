use indexmap::IndexMap;
use mayfig::error::{ErrorCode, Position, Span};
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Tag<'a> {
	#[serde(borrow)]
	t: Tagged<'a>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Inline {
	f: i32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Tagged<'a> {
	Un,
	Ws(usize),
	#[serde(borrow)]
	Ex(Vec<&'a str>),
	St {
		code: i32,
	},
	Tp(u32, &'a str, bool),
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
	let t1 = mayfig::from_str::<Tag>(T1);
	let t1 = t1.unwrap();
	assert_eq!(t1.t, Tagged::Ws(4));

	let t2 = mayfig::from_str::<Tag>(T2);
	let t2 = t2.unwrap();
	assert_eq!(t2.t, Tagged::Ex(vec!["one", "two", "three"]));

	let t3 = mayfig::from_str::<Tag>(T3);
	let t3 = t3.unwrap();
	assert_eq!(t3.t, Tagged::St { code: 200 });

	let t4 = mayfig::from_str::<Tag>(T4);
	let t4 = t4.unwrap();
	assert!(matches!(t4.t, Tagged::Tp(0, "two", false)));

	let t5 = mayfig::from_str::<Tag>(T5);
	let t5 = t5.unwrap();
	assert_eq!(t5.t, Tagged::In(Inline { f: -2 }));

	let t6 = mayfig::from_str::<Tag>(T6);
	let t6 = t6.unwrap();
	assert_eq!(t6.t, Tagged::In(Inline { f: -4 }));

	let t7 = mayfig::from_str::<Tag>(T7);
	let t7 = t7.unwrap_err();
	assert!(matches!(t7.code(), ErrorCode::UnexpectedNewline));
	assert_eq!(
		t7.span(),
		Some(Span::Point(Position {
			line: 2,
			col: 9,
			index: 9
		}))
	);

	let t8 = mayfig::from_str::<Tag>(T8);
	let t8 = t8.unwrap();
	assert_eq!(t8.t, Tagged::Un);

	let t9 = mayfig::from_str::<Tag>(T9);
	let t9 = t9.unwrap();
	assert_eq!(t9.t, Tagged::Ws(4));
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
	let m1 = mayfig::from_str::<Map>(M1);
	let m1 = m1.unwrap();
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
	let nested = mayfig::from_str::<IndexMap<Bind, Action>>(NESTED).unwrap();
	let nested = nested.into_iter().collect::<Vec<_>>();
	assert_eq!(
		&nested,
		&[
			(Bind::Meta(Key::Tab), Action::Cycle(Navigation::Next)),
			(Bind::Ctrl(Key::Tab), Action::Cycle(Navigation::Prev)),
			(Bind::Meta(Key::Left), Action::Move(Direction::Left)),
			(Bind::Meta(Key::Right), Action::Move(Direction::Right)),
		]
	)
}
