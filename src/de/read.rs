use crate::error::Err;
use std::ops::Deref;

#[derive(Debug)]
pub enum Ref<'de, 's> {
	Borrow(&'de str),
	Scratch(&'s str),
}

impl<'de, 's> Deref for Ref<'de, 's> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		match self {
			Ref::Borrow(b) => b,
			Ref::Scratch(s) => s,
		}
	}
}

pub trait Read<'de> {
	fn peek(&self) -> Result<u8, Err>;

	fn next(&mut self) -> Result<u8, Err>;

	fn discard(&mut self);

	fn num(&mut self) -> Result<&'de str, Err>;

	fn word(&mut self) -> Result<&'de str, Err>;

	fn str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's>, Err>;
}

pub struct StrRead<'de> {
	slice: &'de [u8],
	index: usize,
}

impl<'de> StrRead<'de> {
	pub fn new(input: &'de str) -> Self {
		StrRead {
			slice: input.as_bytes(),
			index: 0,
		}
	}
}

impl<'de> Read<'de> for StrRead<'de> {
	fn peek(&self) -> Result<u8, Err> {
		self.slice.get(self.index).copied().ok_or(Err::Eof)
	}

	fn next(&mut self) -> Result<u8, Err> {
		let ch = self.slice.get(self.index).copied().ok_or(Err::Eof)?;
		self.index += 1;
		Ok(ch)
	}

	fn discard(&mut self) {
		self.index += 1;
	}

	fn num(&mut self) -> Result<&'de str, Err> {
		let start = self.index;

		if let Some(b'-') = self.slice.get(start) {
			self.index += 1;
		}

		loop {
			let Ok(next) = self.peek() else { break };

			if let b'0'..=b'9' | b'.' = next {
				self.index += 1;
			} else if is_delimiter(next) {
				break;
			} else {
				return Err(Err::UnexpectedChar(char::from(next), "[num] numeric"));
			}
		}

		let borrow = &self.slice[start..self.index];
		let num = std::str::from_utf8(borrow).expect("should never fail");

		Ok(num)
	}

	fn word(&mut self) -> Result<&'de str, Err> {
		let start = self.index;

		loop {
			let Ok(nxt) = self.peek() else {
				break;
			};

			if nxt.is_ascii_alphanumeric() || nxt == b'_' {
				self.index += 1;
			} else if is_delimiter(nxt) {
				break;
			} else {
				return Err(Err::UnexpectedChar(char::from(nxt), "[word] alphanumeric"));
			}
		}

		let borrow = &self.slice[start..self.index];
		let ident = std::str::from_utf8(borrow).expect("should never fail");

		Ok(ident)
	}

	fn str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's>, Err> {
		let quote = self.next()?;
		assert!(matches!(quote, b'"' | b'\''), "is {:?}", quote);

		scratch.clear();

		let mut start = self.index;

		let r#ref = loop {
			let nxt = self.peek()?;

			if nxt == quote {
				if scratch.is_empty() {
					let borrow = &self.slice[start..self.index];
					let borrow = std::str::from_utf8(borrow).expect("should never fail");

					self.index += 1;

					break Ref::Borrow(borrow);
				} else {
					let slice = &self.slice[start..self.index];
					scratch.extend_from_slice(slice);
					let scratch = std::str::from_utf8(scratch).expect("should never fail");

					self.index += 1;

					break Ref::Scratch(scratch);
				}
			}

			if nxt.is_ascii_control() {
				return Err(Err::UnescapedControl(char::from(nxt)));
			} else if nxt == b'\\' {
				let slice = &self.slice[start..self.index];
				scratch.extend_from_slice(slice);
				self.index += 1;

				parse_escape(self, scratch)?;

				start = self.index;
			} else {
				self.index += 1;
			}
		};

		if let Ok(peek) = self.peek() {
			if !is_delimiter(peek) {
				return Err(Err::ExpectedDelimiter(char::from(peek)));
			}
		}

		Ok(r#ref)
	}
}

fn parse_escape<'de, 's, R: Read<'de>>(
	read: &'s mut R,
	scratch: &'s mut Vec<u8>,
) -> Result<(), Err> {
	let next = read.next()?;

	match next {
		b'"' => scratch.push(b'"'),
		b'\'' => scratch.push(b'\''),
		b'\\' => scratch.push(b'\\'),
		b'/' => scratch.push(b'/'),
		b'n' => scratch.push(b'\n'),
		b'r' => scratch.push(b'\r'),
		b't' => scratch.push(b'\t'),
		b'b' => scratch.push(b'\x08'),
		b'f' => scratch.push(b'\x0c'),
		b'u' => todo!("unicode escape"),
		_ => return Err(Err::UnknownEscape(char::from(next))),
	}

	Ok(())
}

fn is_delimiter(ch: u8) -> bool {
	is_whitespace(ch)
		|| ch == b'='
		|| ch == b','
		|| ch == b';'
		|| ch == b'{'
		|| ch == b'}'
		|| ch == b'['
		|| ch == b']'
}

pub fn is_whitespace(ch: u8) -> bool {
	matches!(ch, b' ' | b'\t' | b'\r' | b'\n')
}

#[cfg(test)]
mod test {
	use super::{Read, StrRead};

	#[test]
	fn str() {
		let mut scratch = Vec::new();

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
