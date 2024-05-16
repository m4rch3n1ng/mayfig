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

pub fn is_whitespace(ch: u8) -> bool {
	matches!(ch, b' ' | b'\t' | b'\r' | b'\n')
}

pub fn is_whitespace_line(ch: u8) -> bool {
	matches!(ch, b' ' | b'\t' | b'\r')
}
