use crate::error::Error;
use std::ops::Deref;

#[derive(Debug)]
pub enum Ref<'de, 's, T>
where
	T: ?Sized,
{
	Borrow(&'de T),
	Scratch(&'s T),
}

impl<'de, 's, T> Deref for Ref<'de, 's, T>
where
	T: ?Sized,
{
	type Target = T;
	fn deref(&self) -> &Self::Target {
		match self {
			Ref::Borrow(b) => b,
			Ref::Scratch(s) => s,
		}
	}
}

impl<'de, 's> From<Ref<'de, 's, str>> for String {
	fn from(value: Ref<'de, 's, str>) -> Self {
		match value {
			Ref::Borrow(b) => b.into(),
			Ref::Scratch(s) => s.into(),
		}
	}
}

pub trait Read<'de> {
	fn peek(&mut self) -> Option<u8>;

	fn next(&mut self) -> Option<u8>;

	fn discard(&mut self);

	fn num<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error>;

	fn word<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error>;

	fn str<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error> {
		let r#ref = self.str_bytes(scratch)?;
		match r#ref {
			Ref::Borrow(v) => std::str::from_utf8(v)
				.map(Ref::Borrow)
				.map_err(|_| Error::InvalidUtf8),
			Ref::Scratch(v) => std::str::from_utf8(v)
				.map(Ref::Scratch)
				.map_err(|_| Error::InvalidUtf8),
		}
	}

	fn str_bytes<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Error>;
}

pub struct SliceRead<'de> {
	slice: &'de [u8],
	index: usize,
}

impl<'de> SliceRead<'de> {
	pub fn new(input: &'de [u8]) -> Self {
		SliceRead {
			slice: input,
			index: 0,
		}
	}
}

impl<'de> Read<'de> for SliceRead<'de> {
	fn peek(&mut self) -> Option<u8> {
		if self.index < self.slice.len() {
			let ch = self.slice[self.index];
			Some(ch)
		} else {
			None
		}
	}

	fn next(&mut self) -> Option<u8> {
		if self.index < self.slice.len() {
			let ch = self.slice[self.index];
			self.index += 1;
			Some(ch)
		} else {
			None
		}
	}

	fn discard(&mut self) {
		self.index += 1;
	}

	fn num<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error> {
		let start = self.index;

		if let Some(b'-') = self.slice.get(self.index) {
			self.index += 1;
		}

		loop {
			let Some(peek) = self.peek() else { break };

			if let b'0'..=b'9' | b'.' = peek {
				self.index += 1;
			} else if is_delimiter(peek) {
				break;
			} else {
				return Err(Error::ExpectedNumeric(peek as char));
			}
		}

		let borrow = &self.slice[start..self.index];
		let borrow = std::str::from_utf8(borrow).expect("should never fail");

		let r#ref = Ref::Borrow(borrow);
		Ok(r#ref)
	}

	fn word<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error> {
		let start = self.index;

		loop {
			let Some(peek) = self.peek() else { break };

			if peek.is_ascii_alphanumeric() {
				self.index += 1;
			} else if is_delimiter(peek) {
				break;
			} else {
				return Err(Error::ExpectedAlphaNumeric(peek as char));
			}
		}

		let borrow = &self.slice[start..self.index];
		let borrow = std::str::from_utf8(borrow).expect("should never fail");

		let r#ref = Ref::Borrow(borrow);
		Ok(r#ref)
	}

	fn str_bytes<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Error> {
		let quote = self.next().ok_or(Error::Eof)?;
		assert!(matches!(quote, b'"' | b'\''), "is {:?}", quote as char);

		let mut start = self.index;

		let r#ref = loop {
			let peek = self.peek().ok_or(Error::Eof)?;

			if peek == quote {
				if scratch.is_empty() {
					let borrow = &self.slice[start..self.index];
					self.index += 1;
					break Ref::Borrow(borrow);
				} else {
					let slice = &self.slice[start..self.index];
					scratch.extend_from_slice(slice);
					self.index += 1;
					break Ref::Scratch(scratch);
				}
			}

			if peek.is_ascii_control() {
				return Err(Error::UnescapedControl(peek as char));
			} else if peek == b'\\' {
				let slice = &self.slice[start..self.index];
				scratch.extend_from_slice(slice);

				self.index += 1;

				parse_escape(self, scratch)?;

				start = self.index;
			} else {
				self.index += 1;
			}
		};

		if let Some(peek) = self.peek() {
			if !is_delimiter(peek) {
				return Err(Error::ExpectedDelimiter(peek as char));
			}
		}

		Ok(r#ref)
	}
}

pub struct StrRead<'de>(SliceRead<'de>);

impl<'de> StrRead<'de> {
	pub fn new(input: &'de str) -> Self {
		let slice = SliceRead::new(input.as_bytes());
		StrRead(slice)
	}
}

impl<'de> Read<'de> for StrRead<'de> {
	fn peek(&mut self) -> Option<u8> {
		self.0.peek()
	}

	fn next(&mut self) -> Option<u8> {
		self.0.next()
	}

	fn discard(&mut self) {
		self.0.discard();
	}

	fn num<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error> {
		self.0.num(scratch)
	}

	fn word<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Error> {
		self.0.word(scratch)
	}

	fn str_bytes<'s>(&mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Error> {
		self.0.str_bytes(scratch)
	}
}

fn parse_escape<'de, R: Read<'de>>(read: &mut R, scratch: &mut Vec<u8>) -> Result<(), Error> {
	let next = read.next().ok_or(Error::Eof)?;

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
		_ => return Err(Error::UnknownEscape(next as char)),
	}

	Ok(())
}

fn is_delimiter(ch: u8) -> bool {
	is_whitespace(ch)
		|| ch == b'='
		|| ch == b','
		|| ch == b'{'
		|| ch == b'}'
		|| ch == b'['
		|| ch == b']'
}

pub fn parse_bool(word: &str) -> Result<bool, Error> {
	if word.eq_ignore_ascii_case("true") {
		Ok(true)
	} else if word.eq_ignore_ascii_case("false") {
		Ok(false)
	} else {
		Err(Error::InvalidBool(word.to_owned()))
	}
}

pub fn is_whitespace(ch: u8) -> bool {
	matches!(ch, b' ' | b'\t' | b'\r' | b'\n')
}

pub fn is_whitespace_line(ch: u8) -> bool {
	matches!(ch, b' ' | b'\t' | b'\r')
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

		scratch.clear();
		let p2 = s2.str(&mut scratch).unwrap();
		assert_eq!(&*p2, r#"t"e"st"#);

		let s3 = String::from(r#""t\tt""#);
		let mut s3 = StrRead::new(&s3);

		scratch.clear();
		let p3 = s3.str(&mut scratch).unwrap();
		assert_eq!(&*p3, "t\tt");

		let s4 = String::from(r#""t\\\\\"\"\\\"t""#);
		let mut s4 = StrRead::new(&s4);

		scratch.clear();
		let p4 = s4.str(&mut scratch).unwrap();
		assert_eq!(&*p4, r#"t\\""\"t"#);
	}
}
