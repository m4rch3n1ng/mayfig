use indexmap::{indexmap, IndexMap};
use mayfig::{error::ErrorCode, Regex};
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
			regex: "test".to_owned(),
			flags: "".to_owned(),
		}
	);

	assert_de!(
		R2 as Thing => r,
		r.x,
		Regex {
			regex: r"\d+".to_owned(),
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
				regex: r"\d".to_owned(),
				flags: "ui".to_owned(),
			}) => Action::Remove(Regex {
				regex: r"[0-3]+".to_owned(),
				flags: String::new(),
			}),
			Match::Value(Regex {
				regex: r"^#".to_owned(),
				flags: "v".to_owned(),
			}) => Action::Replace(
				Regex {
					regex: r"[xyz]".to_owned(),
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
			regex: r"[^/\\]+".to_owned(),
			flags: String::new()
		}
	);
}
