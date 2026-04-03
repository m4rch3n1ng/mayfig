use indexmap::{indexmap, IndexMap};
use mayfig::{
	error::{ErrorCode, Position, Span},
	Regex,
};
use serde::Deserialize;

mod maytest;

#[derive(Debug, Deserialize)]
struct Thing {
	x: Regex,
}

const R1: &str = r#"
x = /test/
"#;

const R2: &str = r#"
x = /\d+/v
"#;

#[test]
fn regex() {
	assert_de!(
		R1 as Thing => r,
		r.x,
		Regex {
			pattern: "test".to_owned(),
			flags: "".to_owned(),
		}
	);

	assert_de!(
		R2 as Thing => r,
		r.x,
		Regex {
			pattern: r"\d+".to_owned(),
			flags: "v".to_owned()
		}
	);
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Match {
	Key(Regex),
	Value(Regex),
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Action {
	Remove(Regex),
	Replace(Regex, String),
}

const E1: &str = r#"
key [ /\d/ui ] = "remove" [ /[0-3]+/ ]
value [ /^#/v ] = "replace" [ /[xyz]/i "a" ]
"#;

#[test]
fn r#enum() {
	assert_de!(
		E1 as IndexMap<Match, Action>,
		indexmap! {
			Match::Key(Regex {
				pattern: r"\d".to_owned(),
				flags: "ui".to_owned(),
			}) => Action::Remove(Regex {
				pattern: r"[0-3]+".to_owned(),
				flags: String::new(),
			}),
			Match::Value(Regex {
				pattern: r"^#".to_owned(),
				flags: "v".to_owned(),
			}) => Action::Replace(
				Regex {
					pattern: r"[xyz]".to_owned(),
					flags: "i".to_owned(),
				},
				"a".to_owned()
			)
		}
	);
}

const K1: &str = r#"
/test/x = 0
"#;

#[test]
fn key() {
	assert_err!(
		K1 as IndexMap<Regex, u64>,
		ErrorCode::UnsupportedMapKey("regex")
	);
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Untagged {
	String(String),
	Regex(Regex),
}

const UNTAGGED: &str = r#"
one = /test/
two = "test"
thr = /wow/i
fou = "nope"
"#;

#[test]
fn untagged() {
	assert_de!(
		UNTAGGED as IndexMap<String, Untagged>,
		indexmap! {
			"one".to_owned() => Untagged::Regex(Regex { pattern: "test".to_owned(), flags: String::new() }),
			"two".to_owned() => Untagged::String("test".to_owned()),
			"thr".to_owned() => Untagged::Regex(Regex { pattern: "wow".to_owned(), flags: "i".to_owned() }),
			"fou".to_owned() => Untagged::String("nope".to_owned())
		}
	);
}

const DELIM1: &str = r#"
x = /regex/a0
"#;

const DELIM2: &str = r#"
x = /regex/ä
"#;

const DELIM3: &str = r#"
y = /regex/v#comment
"#;

const DELIM4: &str = r#"z=/regex/i"#;

#[test]
fn delims() {
	assert_err!(
		DELIM1 as IndexMap<String, Regex>,
		ErrorCode::ExpectedRegexFlag('0'),
		Span::new(
			Position {
				line: 2,
				col: 13,
				index: 13
			},
			Position {
				line: 2,
				col: 14,
				index: 14
			}
		)
	);

	assert_err!(
		DELIM2 as IndexMap<String, Regex>,
		ErrorCode::ExpectedRegexFlag('ä'),
		Span::new(
			Position {
				line: 2,
				col: 12,
				index: 12
			},
			Position {
				line: 2,
				col: 13,
				index: 14
			}
		)
	);

	assert_de!(
		DELIM3 as IndexMap<String, Regex>,
		indexmap! {
			"y".to_owned() => Regex { pattern: "regex".to_owned(), flags: "v".to_owned() }
		}
	);

	assert_de!(
		DELIM4 as IndexMap<String, Regex>,
		indexmap! {
			"z".to_owned() => Regex { pattern: "regex".to_owned(), flags: "i".to_owned() }
		}
	);
}

#[derive(Debug, Deserialize)]
struct Alt {
	hm: Regex,
}

const ALT: &str = r#"
hm = /[^/\\]+/
"#;

#[test]
#[ignore = "not yet implemented"]
fn alternation() {
	assert_de!(
		ALT as Alt => alt,
		alt.hm,
		Regex {
			pattern: r"[^/\\]+".to_owned(),
			flags: String::new()
		}
	);
}
