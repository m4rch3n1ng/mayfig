use crate::error::Err;

pub struct StrRead<'de> {
	input: &'de str,
}

impl<'de> StrRead<'de> {
	pub fn new(input: &'de str) -> Self {
		StrRead { input }
	}

	pub fn peek(&self) -> Result<char, Err> {
		self.input.chars().next().ok_or(Err::Eof)
	}

	pub fn next(&mut self) -> Result<char, Err> {
		let char = self.input.chars().next().ok_or(Err::Eof)?;
		let len = char.len_utf8();
		self.input = &self.input[len..];
		Ok(char)
	}

	pub fn discard(&mut self) {
		let _ = self.next();
	}

	pub fn num(&mut self) -> Result<&'de str, Err> {
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

	pub fn word(&mut self) -> Result<&'de str, Err> {
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
}
