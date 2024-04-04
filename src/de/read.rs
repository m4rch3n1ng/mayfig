use crate::error::Err;
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
			Ref::Borrow(b) => b.to_owned(),
			Ref::Scratch(s) => s.to_owned(),
		}
	}
}

pub trait Read<'de> {
	fn peek(&mut self) -> Result<Option<u8>, Err>;

	fn next(&mut self) -> Result<Option<u8>, Err>;

	fn discard(&mut self);

	fn num<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err>;

	fn word<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err>;

	fn str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		let r#ref = self.str_bytes(scratch)?;
		match r#ref {
			Ref::Borrow(v) => std::str::from_utf8(v)
				.map(Ref::Borrow)
				.map_err(|_| Err::InvalidUtf8),
			Ref::Scratch(v) => std::str::from_utf8(v)
				.map(Ref::Scratch)
				.map_err(|_| Err::InvalidUtf8),
		}
	}

	fn str_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Err>;
}

pub struct IoRead<R: std::io::Read> {
	read: std::io::Bytes<R>,
	peek: Option<u8>,
}

impl<R: std::io::Read> IoRead<R> {
	pub fn new(read: R) -> Self {
		IoRead {
			read: read.bytes(),
			peek: None,
		}
	}
}

impl<'de, R> Read<'de> for IoRead<R>
where
	R: std::io::Read,
{
	fn peek(&mut self) -> Result<Option<u8>, Err> {
		match self.peek {
			peek @ Some(_) => Ok(peek),
			None => match self.read.next() {
				Some(Err(err)) => Err(Err::Io(err)),
				Some(Ok(peek)) => {
					self.peek = Some(peek);
					Ok(Some(peek))
				}
				None => Ok(None),
			},
		}
	}

	fn next(&mut self) -> Result<Option<u8>, Err> {
		match self.peek.take() {
			next @ Some(_) => Ok(next),
			None => match self.read.next() {
				Some(Err(err)) => Err(Err::Io(err)),
				Some(Ok(next)) => Ok(Some(next)),
				None => Ok(None),
			},
		}
	}

	fn discard(&mut self) {
		if self.peek.take().is_none() {
			unreachable!()
		}
	}

	fn num<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		if let Some(peek @ b'-') = self.peek()? {
			scratch.push(peek);
			self.discard();
		}

		loop {
			let Some(next) = self.next()? else { break };

			if let b'0'..=b'9' | b'.' = next {
				scratch.push(next);
			} else if is_delimiter(next) {
				break;
			} else {
				return Err(Err::UnexpectedChar(char::from(next), "[num] numeric"));
			}
		}

		let scratch = std::str::from_utf8(scratch).map_err(|_| Err::InvalidUtf8)?;
		let r#ref = Ref::Scratch(scratch);
		Ok(r#ref)
	}

	fn word<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		loop {
			let Some(next) = self.next()? else { break };

			if next.is_ascii_alphanumeric() || next == b'_' {
				scratch.push(next);
			} else if is_delimiter(next) {
				break;
			} else {
				return Err(Err::UnexpectedChar(char::from(next), "[word] alphanumeric"));
			}
		}

		let scratch = std::str::from_utf8(scratch).map_err(|_| Err::InvalidUtf8)?;
		let r#ref = Ref::Scratch(scratch);
		Ok(r#ref)
	}

	fn str_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Err> {
		let quote = self.next()?.ok_or(Err::Eof)?;
		assert!(matches!(quote, b'"' | b'\''), "is {:?}", quote);

		let r#ref: Ref<'de, 's, [u8]> = loop {
			let next = self.next()?.ok_or(Err::Eof)?;

			if next == quote {
				break Ref::Scratch(scratch);
			}

			if next.is_ascii_control() {
				return Err(Err::UnescapedControl(char::from(next)));
			} else if next == b'\\' {
				parse_escape(self, scratch)?;
			} else {
				scratch.push(next);
			}
		};

		if let Some(peek) = self.peek()? {
			if !is_delimiter(peek) {
				return Err(Err::ExpectedDelimiter(char::from(peek)));
			}
		}

		Ok(r#ref)
	}
}

pub struct SliceRead<'de> {
	slice: &'de [u8],
	index: usize,
}

impl<'de> SliceRead<'de> {
	pub fn new(slice: &'de [u8]) -> Self {
		SliceRead { slice, index: 0 }
	}
}

impl<'de> Read<'de> for SliceRead<'de> {
	fn peek(&mut self) -> Result<Option<u8>, Err> {
		if self.index < self.slice.len() {
			let ch = self.slice[self.index];
			Ok(Some(ch))
		} else {
			Ok(None)
		}
	}

	fn next(&mut self) -> Result<Option<u8>, Err> {
		if self.index < self.slice.len() {
			let ch = self.slice[self.index];
			self.index += 1;
			Ok(Some(ch))
		} else {
			Ok(None)
		}
	}

	fn discard(&mut self) {
		self.index += 1;
	}

	fn num<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		let start = self.index;

		if let Some(b'-') = self.slice.get(start) {
			self.index += 1;
		}

		loop {
			let Some(next) = self.peek()? else { break };

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

		let r#ref = Ref::Borrow(num);
		Ok(r#ref)
	}

	fn word<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		let start = self.index;

		loop {
			let Some(next) = self.peek()? else { break };

			if next.is_ascii_alphanumeric() || next == b'_' {
				self.index += 1;
			} else if is_delimiter(next) {
				break;
			} else {
				return Err(Err::UnexpectedChar(char::from(next), "[word] alphanumeric"));
			}
		}

		let borrow = &self.slice[start..self.index];
		let ident = std::str::from_utf8(borrow).expect("should never fail");

		let r#ref = Ref::Borrow(ident);
		Ok(r#ref)
	}

	fn str_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Err> {
		let quote = self.next()?.ok_or(Err::Eof)?;
		assert!(matches!(quote, b'"' | b'\''), "is {:?}", quote as char);

		let mut start = self.index;

		let r#ref = loop {
			let next = self.peek()?.ok_or(Err::Eof)?;

			if next == quote {
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

			if next.is_ascii_control() {
				return Err(Err::UnescapedControl(char::from(next)));
			} else if next == b'\\' {
				let slice = &self.slice[start..self.index];
				scratch.extend_from_slice(slice);
				self.index += 1;

				parse_escape(self, scratch)?;

				start = self.index;
			} else {
				self.index += 1;
			}
		};

		if let Some(peek) = self.peek()? {
			if !is_delimiter(peek) {
				return Err(Err::ExpectedDelimiter(char::from(peek)));
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
	fn peek(&mut self) -> Result<Option<u8>, Err> {
		self.0.peek()
	}

	fn next(&mut self) -> Result<Option<u8>, Err> {
		self.0.next()
	}

	fn discard(&mut self) {
		self.0.discard()
	}

	fn num<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		self.0.num(scratch)
	}

	fn word<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, str>, Err> {
		self.0.word(scratch)
	}

	fn str_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Ref<'de, 's, [u8]>, Err> {
		self.0.str_bytes(scratch)
	}
}

fn parse_escape<'de, 's, R: Read<'de>>(
	read: &'s mut R,
	scratch: &'s mut Vec<u8>,
) -> Result<(), Err> {
	let next = read.next()?.ok_or(Err::Eof)?;

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
