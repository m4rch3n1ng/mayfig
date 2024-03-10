use std::ops::Deref;

use crate::error::Err;

#[derive(Debug)]
pub enum Ref<'de, 's> {
	Borrow(&'de str),
	Scratch(&'s str),
}

impl<'de, 's> Deref for Ref<'de, 's> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		match self {
			Ref::Borrow(s) => s,
			Ref::Scratch(s) => s,
		}
	}
}

pub trait Read<'de> {
	fn peek(&self) -> Result<char, Err>;

	fn next(&mut self) -> Result<char, Err>;

	fn discard(&mut self);

	fn num(&mut self) -> Result<&'de str, Err>;

	fn word(&mut self) -> Result<&'de str, Err>;

	fn str<'s>(&'s mut self, scratch: &'s mut String) -> Result<Ref<'de, 's>, Err>;
}

pub struct StrRead<'de> {
	input: &'de str,
}

impl<'de> StrRead<'de> {
	pub fn new(input: &'de str) -> Self {
		StrRead { input }
	}
}

impl<'de> Read<'de> for StrRead<'de> {
	fn peek(&self) -> Result<char, Err> {
		self.input.chars().next().ok_or(Err::Eof)
	}

	fn next(&mut self) -> Result<char, Err> {
		let char = self.input.chars().next().ok_or(Err::Eof)?;
		let len = char.len_utf8();
		self.input = &self.input[len..];
		Ok(char)
	}

	fn discard(&mut self) {
		let _ = self.next();
	}

	fn num(&mut self) -> Result<&'de str, Err> {
		let mut one = &self.input[0..0];
		let mut two = self.input;

		if let Some('-') = two.chars().next() {
			let len = '-'.len_utf8();

			one = &self.input[..len];
			two = &self.input[len..];
		}

		loop {
			let Some(nxt) = two.chars().next() else {
				break;
			};

			if let '0'..='9' | '.' = nxt {
				let len = one.len() + nxt.len_utf8();

				one = &self.input[..len];
				two = &self.input[len..];
			} else if nxt.is_ascii_whitespace() || nxt.is_ascii_punctuation() {
				break;
			} else {
				return Err(Err::UnexpectedChar(nxt, "[num] numeric"));
			}
		}

		self.input = two;
		Ok(one)
	}

	fn word(&mut self) -> Result<&'de str, Err> {
		let mut one = &self.input[0..0];
		let mut two = self.input;

		loop {
			let Some(nxt) = two.chars().next() else {
				break;
			};

			if nxt.is_alphanumeric() || nxt == '_' {
				let len = one.len() + nxt.len_utf8();

				one = &self.input[..len];
				two = &self.input[len..];
			} else if nxt.is_ascii_whitespace() || nxt.is_ascii_punctuation() {
				break;
			} else {
				return Err(Err::UnexpectedChar(nxt, "[word] alphanumeric"));
			}
		}

		self.input = two;
		Ok(one)
	}

	fn str<'s>(&'s mut self, scratch: &'s mut String) -> Result<Ref<'de, 's>, Err> {
		let quote = self.next()?;
		assert!(matches!(quote, '"' | '\''), "is {:?}", quote);

		scratch.clear();

		let mut one = &self.input[0..0];
		let mut two = self.input;

		loop {
			let Some(nxt) = two.chars().next() else {
				break;
			};

			if nxt == quote {
				self.input = two;
				self.discard();

				break;
			}

			if nxt.is_control() {
				return Err(Err::UnescapedControl(nxt));
			} else if nxt == '\\' {
				scratch.push_str(one);

				self.input = two;
				self.discard();

				parse_escape(self, scratch)?;

				one = &self.input[0..0];
				two = self.input;
			} else {
				let len = one.len() + nxt.len_utf8();

				one = &self.input[..len];
				two = &self.input[len..];
			}
		}

		if let Ok(peek) = self.peek() {
			if !is_delimiter(peek) {
				return Err(Err::ExpectedDelimiter(peek));
			}
		}

		if scratch.is_empty() {
			Ok(Ref::Borrow(one))
		} else {
			scratch.push_str(one);
			Ok(Ref::Scratch(scratch))
		}
	}
}

fn parse_escape<'de, 's, R: Read<'de>>(
	read: &'s mut R,
	scratch: &'s mut String,
) -> Result<(), Err> {
	let next = read.next()?;

	match next {
		'"' => scratch.push('"'),
		'\'' => scratch.push('\''),
		'\\' => scratch.push('\\'),
		'/' => scratch.push('/'),
		'n' => scratch.push('\n'),
		'r' => scratch.push('\r'),
		't' => scratch.push('\t'),
		'b' => scratch.push('\x08'),
		'f' => scratch.push('\x0c'),
		'u' => todo!("unicode escape"),
		_ => return Err(Err::UnknownEscape(next)),
	}

	Ok(())
}

fn is_delimiter(ch: char) -> bool {
	ch.is_ascii_whitespace()
		|| ch == '='
		|| ch == ','
		|| ch == ';'
		|| ch == '{'
		|| ch == '}'
		|| ch == '['
		|| ch == ']'
}

#[cfg(test)]
mod test {
	use super::{Read, StrRead};

	#[test]
	fn str() {
		let mut scratch = String::new();

		let s1 = String::from(r#""test""#);
		let mut s1 = StrRead::new(&s1);

		let p1 = s1.str(&mut scratch).unwrap();
		assert_eq!(&*p1, "test");

		let s2 = String::from(r#""t\"e\"st""#);
		let mut s2 = StrRead::new(&s2);

		let p2 = s2.str(&mut scratch).unwrap();
		assert_eq!(&*p2, r#"t"e"st"#);

		let s3 = String::from(r#""t\tt""#);
		let mut s3 = StrRead::new(&s3);

		let p3 = s3.str(&mut scratch).unwrap();
		assert_eq!(&*p3, "t\tt");

		let s4 = String::from(r#""t\\\\\"\"\\\"t""#);
		let mut s4 = StrRead::new(&s4);

		let p4 = s4.str(&mut scratch).unwrap();
		assert_eq!(&*p4, r#"t\\""\"t"#);
	}
}
