use annotate_snippets::{Level, Renderer, Snippet};
use serde::{de::Visitor, Deserialize};
use serde_derive::Deserialize;

#[derive(Debug)]
#[expect(dead_code)]
struct Color([u8; 3]);

impl<'de> Deserialize<'de> for Color {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(ColorVis)
	}
}

struct ColorVis;

impl Visitor<'_> for ColorVis {
	type Value = Color;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("a mayfig color")
	}

	fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
		let hex = hex_color(v)
			.ok_or_else(|| serde::de::Error::custom(format_args!("invalid hex color {:?}", v)))?;
		Ok(Color(hex))
	}
}

fn hex_digit(c: u8) -> Option<u8> {
	match c {
		b'0'..=b'9' => Some(c - b'0'),
		b'A'..=b'F' => Some(c - b'A' + 10),
		b'a'..=b'f' => Some(c - b'a' + 10),
		_ => None,
	}
}

fn hex_color(s: &str) -> Option<[u8; 3]> {
	let hex = s.strip_prefix("#")?;
	if let [r1, r2, g1, g2, b1, b2] = hex.as_bytes() {
		let color = [
			hex_digit(*r1)? * 16 + hex_digit(*r2)?,
			hex_digit(*g1)? * 16 + hex_digit(*g2)?,
			hex_digit(*b1)? * 16 + hex_digit(*b2)?,
		];
		Some(color)
	} else {
		None
	}
}

#[derive(Debug)]
#[expect(dead_code)]
struct Number(u8);

impl<'de> Deserialize<'de> for Number {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_u8(NumberVis)
	}
}

struct NumberVis;

impl Visitor<'_> for NumberVis {
	type Value = Number;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("a number that is mod 5")
	}

	fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
		if v % 5 != 0 {
			Err(serde::de::Error::custom("number has to be mod 5"))
		} else {
			Ok(Number(v))
		}
	}
}

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct AnnotatedError {
	v: Number,
	t: Color,
}

const WITH_ERROR: &str = r##"
t = "#0080xx"
v = 21
"##;

fn main() {
	// blocked on https://github.com/rust-lang/annotate-snippets-rs/issues/25
	let with_error = WITH_ERROR.replace('\t', "    ");

	let thing = mayfig::from_str::<AnnotatedError>(&with_error);
	let thing = match thing {
		Ok(thing) => thing,
		Err(err) => {
			let code = err.code().to_string();
			let message = if let Some(span) = err.span() {
				Level::Error.title(code.as_str()).snippet(
					Snippet::source(&with_error)
						.origin("test/test.mf")
						.fold(true)
						.annotation(Level::Error.span(span.range())),
				)
			} else {
				Level::Error.title(code.as_str())
			};

			let renderer = Renderer::styled();
			anstream::println!("{}", renderer.render(message));

			return;
		}
	};

	println!("thing {:?}", thing);
}
