use mayfig::error::Error;
use serde_derive::Deserialize;

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
	assert!(matches!(t7, Error::UnexpectedNewline));

	let t8 = mayfig::from_str::<Tag>(T8);
	let t8 = t8.unwrap();
	assert_eq!(t8.t, Tagged::Un);

	let t9 = mayfig::from_str::<Tag>(T9);
	let t9 = t9.unwrap();
	assert_eq!(t9.t, Tagged::Ws(4));
}
