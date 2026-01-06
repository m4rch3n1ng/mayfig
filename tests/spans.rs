use mayfig::error::{ErrorCode, Position, Span};
use serde::{de::Visitor, Deserialize};

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct N {
	n: Num,
}

#[derive(Debug)]
#[expect(dead_code)]
struct Num(u8);

impl<'de> Deserialize<'de> for Num {
	fn deserialize<D: serde_core::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_u8(NumVis)
	}
}

struct NumVis;

impl Visitor<'_> for NumVis {
	type Value = Num;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("a number that is mod 5")
	}

	fn visit_u8<E: serde_core::de::Error>(self, v: u8) -> Result<Self::Value, E> {
		if v.is_multiple_of(5) {
			Ok(Num(v))
		} else {
			Err(serde_core::de::Error::custom(
				"number has to be a multiple of 5",
			))
		}
	}
}

const N1: &str = r#"
n = 21
"#;

const N2: &str = r#"
n = 20
"#;

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct C {
	c: Col,
}

#[derive(Debug)]
#[expect(dead_code)]
struct Col(String);

impl<'de> Deserialize<'de> for Col {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde_core::Deserializer<'de>,
	{
		deserializer.deserialize_str(ColVis)
	}
}

struct ColVis;

impl Visitor<'_> for ColVis {
	type Value = Col;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("a color")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		if !v.starts_with('#') {
			Err(serde_core::de::Error::custom("a color must start with a #"))
		} else {
			Ok(Col(v.to_owned()))
		}
	}
}

const C1: &str = r##"
c = "#008080"
"##;

const C2: &str = r##"
c = "008080"
"##;

#[test]
fn custom_spans() {
	let n1 = mayfig::from_str::<N>(N1);
	let e1 = n1.unwrap_err();
	assert!(matches!(e1.code(), ErrorCode::Custom(_)));
	assert_eq!(
		e1.span(),
		Some(Span::Span(
			Position {
				line: 2,
				col: 5,
				index: 5,
			},
			Position {
				line: 2,
				col: 7,
				index: 7
			}
		))
	);

	let n2 = mayfig::from_str::<N>(N2);
	let _ = n2.unwrap();

	let c1 = mayfig::from_str::<C>(C1);
	let _ = c1.unwrap();

	let c2 = mayfig::from_str::<C>(C2);
	let e2 = c2.unwrap_err();
	assert!(matches!(e2.code(), ErrorCode::Custom(_)));
	assert_eq!(
		e2.span(),
		Some(Span::Span(
			Position {
				line: 2,
				col: 5,
				index: 5
			},
			Position {
				line: 2,
				col: 13,
				index: 13
			}
		))
	);
}
