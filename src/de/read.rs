use crate::error::{Error, ErrorCode, Position};
use std::{iter::Peekable, ops::Deref, str::CharIndices};

impl Position {
	fn new() -> Self {
		Position {
			line: 1,
			col: 1,
			index: 0,
		}
	}

	fn next(&mut self, ch: char) {
		self.index += ch.len_utf8();
		if ch == '\n' {
			self.line += 1;
			self.col = 1;
		} else {
			self.col += 1;
		}
	}
}

#[derive(Debug)]
pub enum Ref<'de, 's, T>
where
	T: ?Sized,
{
	Borrow(&'de T),
	Scratch(&'s T),
}

impl<T> Deref for Ref<'_, '_, T>
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
	fn peek(&mut self) -> Option<char>;

	fn next(&mut self) -> Option<char>;

	fn discard(&mut self);

	fn position(&mut self) -> Position;

	fn num<'s>(&mut self, scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error>;

	fn word<'s>(&mut self, scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error>;

	fn str<'s>(&mut self, scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error>;
}

pub struct StrRead<'de> {
	input: &'de str,
	iter: Peekable<CharIndices<'de>>,
	pos: Position,
}

impl<'de> StrRead<'de> {
	pub fn new(input: &'de str) -> Self {
		StrRead {
			input,
			iter: input.char_indices().peekable(),
			pos: Position::new(),
		}
	}

	/// slice from the given start to the current index
	fn slice(&mut self, start: usize) -> &'de str {
		let index = self.position().index;
		&self.input[start..index]
	}
}

impl<'de> Read<'de> for StrRead<'de> {
	fn peek(&mut self) -> Option<char> {
		self.iter.peek().map(|x| x.1)
	}

	fn next(&mut self) -> Option<char> {
		self.iter.next().map(|x| x.1).inspect(|x| self.pos.next(*x))
	}

	fn discard(&mut self) {
		let _ = self.next();
	}

	fn position(&mut self) -> Position {
		self.pos
	}

	fn num<'s>(&mut self, _scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error> {
		let start = self.position().index;

		if let Some('+' | '-') = self.peek() {
			self.discard();
		}

		if let Some('.') = self.peek() {
			self.discard();

			if let Some('a'..='z' | 'A'..='Z') = self.peek() {
				while let Some('a'..='z' | 'A'..='Z') = self.peek() {
					self.discard();
				}

				let borrow = self.slice(start);
				let r#ref = Ref::Borrow(borrow);
				return Ok(r#ref);
			}
		}

		while let Some(peek) = self.peek() {
			if let '0'..='9' | '.' | 'e' | '-' | '+' = peek {
				self.discard();
			} else if is_delimiter(peek) {
				break;
			} else {
				let point = self.position();
				let code = ErrorCode::ExpectedNumeric(peek);
				return Err(Error::with_point(code, point));
			}
		}

		let borrow = self.slice(start);
		let r#ref = Ref::Borrow(borrow);
		Ok(r#ref)
	}

	fn word<'s>(&mut self, _scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error> {
		let start = self.position().index;

		while let Some(peek) = self.peek() {
			if is_word(peek) {
				self.discard();
			} else if is_delimiter(peek) {
				break;
			} else {
				let point = self.position();
				let code = ErrorCode::ExpectedWordContinue(peek);
				return Err(Error::with_point(code, point));
			}
		}

		let borrow = self.slice(start);
		let r#ref = Ref::Borrow(borrow);
		Ok(r#ref)
	}

	fn str<'s>(&mut self, scratch: &'s mut String) -> Result<Ref<'de, 's, str>, Error> {
		let quote = self.next().ok_or(Error::EOF)?;
		debug_assert!(matches!(quote, '"' | '\''), "is {:?}", quote);

		let mut start = self.position().index;
		let r#ref = loop {
			let peek = self.peek().ok_or(Error::EOF)?;

			if peek == quote {
				if scratch.is_empty() {
					let borrow = self.slice(start);
					self.discard();

					break Ref::Borrow(borrow);
				} else {
					let slice = self.slice(start);

					scratch.push_str(slice);
					self.discard();
					break Ref::Scratch(scratch);
				}
			}

			if peek.is_ascii_control() {
				let point = self.position();
				let code = ErrorCode::UnescapedControl(peek);
				return Err(Error::with_point(code, point));
			} else if peek == '\\' {
				let slice = self.slice(start);
				scratch.push_str(slice);

				self.discard();
				parse_escape(self, scratch)?;

				start = self.position().index;
			} else {
				self.discard();
			}
		};

		if let Some(peek) = self.peek() {
			if !is_delimiter(peek) {
				let point = self.position();
				let code = ErrorCode::ExpectedDelimiter(peek);
				return Err(Error::with_point(code, point));
			}
		}

		Ok(r#ref)
	}
}

fn parse_escape<'de, R: Read<'de>>(read: &mut R, scratch: &mut String) -> Result<(), Error> {
	let peek = read.peek().ok_or(Error::EOF)?;
	match peek {
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
		_ => {
			let point = read.position();
			let code = ErrorCode::UnknownEscape(peek);
			return Err(Error::with_point(code, point));
		}
	}

	read.discard();
	Ok(())
}

fn is_delimiter(ch: char) -> bool {
	is_whitespace(ch)
		|| ch == '='
		|| ch == ','
		|| ch == '{'
		|| ch == '}'
		|| ch == '['
		|| ch == ']'
		|| ch == '#'
}

pub fn is_word(ch: char) -> bool {
	is_word_start(ch) || ch.is_ascii_digit() || ch == '+' || ch == '-'
}

pub fn is_word_start(ch: char) -> bool {
	ch.is_ascii_alphabetic() || ch == '_'
}

pub fn is_whitespace(ch: char) -> bool {
	matches!(ch, ' ' | '\t' | '\r' | '\n')
}

pub fn is_whitespace_line(ch: char) -> bool {
	matches!(ch, ' ' | '\t' | '\r')
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
