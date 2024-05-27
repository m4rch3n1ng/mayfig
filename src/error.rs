use std::{cmp::Ordering, fmt::Display};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Error {
	code: ErrorCode,
	span: Option<Span>,
}

impl Error {
	pub fn code(&self) -> &ErrorCode {
		&self.code
	}

	pub fn span(&self) -> Option<Span> {
		self.span
	}
}

impl Error {
	pub(crate) fn new(code: ErrorCode) -> Self {
		Error { code, span: None }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(span) = self.span {
			write!(f, "{} at {}", self.code, span)
		} else {
			Display::fmt(&self.code, f)
		}
	}
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		let msg = msg.to_string();
		let code = ErrorCode::Custom(msg);
		Error::new(code)
	}
}

#[derive(Debug, Clone, Error)]
pub enum ErrorCode {
	#[error("end of file")]
	Eof,
	#[error("invalid utf8")]
	InvalidUtf8,

	#[error("unknown escape sequence {0:?}")]
	UnknownEscape(char),
	#[error("unescaped control character {0:?}")]
	UnescapedControl(char),

	#[error("expected newline, found {0:?} first")]
	ExpectedNewline(char),
	#[error("unexpected newline")]
	UnexpectedNewline,

	#[error("invalid boolean {0:?}")]
	InvalidBool(String),
	#[error("invalid number {0:?}")]
	InvalidNum(String),

	#[error("expected quote \" or ', got {0:?}")]
	ExpectedQuote(char),
	#[error("expected value assignment '=' or '{{', got {0:?}")]
	ExpectedValue(char),
	#[error("expected map '{{', got {0:?}")]
	ExpectedMap(char),
	#[error("expected end of map '}}', got {0:?}")]
	ExpectedMapEnd(char),
	#[error("expected seq '[', got {0:?}")]
	ExpectedSeq(char),
	#[error("expected end of seq ']', got {0:?}")]
	ExpectedSeqEnd(char),
	#[error("expected quote ', \" or seq, got {0:?}")]
	ExpectedBytes(char),

	#[error("expected delimiter after string, got {0:?}")]
	ExpectedDelimiter(char),

	#[error("expected numeric, got {0:?}")]
	ExpectedNumeric(char),
	#[error("expected alphabetic, got {0:?}")]
	ExpectedAlphabetic(char),
	#[error("expected alphanumeric, got {0:?}")]
	ExpectedAlphaNumeric(char),

	#[error("unsupported unit type")]
	UnsupportedUnit,
	#[error("unsupported nan")]
	UnsupportedNaN,
	#[error("unsupported map key type {0}")]
	UnsupportedMapKey(&'static str),

	#[error("custom: {0}")]
	Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
	pub line: usize,
	pub col: usize,
	pub index: usize,
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "line {}, column {}", self.line, self.col)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
	Point(Position),
	Span(Position, Position),
}

impl Span {
	pub fn range(&self) -> std::ops::Range<usize> {
		match self {
			Span::Point(pos) => pos.index..(pos.index + 1),
			Span::Span(pos1, pos2) => pos1.index..(pos2.index),
		}
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Span::Point(pos) => Display::fmt(pos, f),
			Span::Span(pos1, pos2) => {
				if pos1.line == pos2.line {
					write!(
						f,
						"line {}, columns {} to {}",
						pos1.line, pos1.col, pos2.col
					)
				} else {
					write!(f, "{} to {}", pos1, pos2)
				}
			}
		}
	}
}

impl Ord for Span {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Span::Point(p1), Span::Point(p2)) => p1.cmp(p2),
			(Span::Span(s1_1, s1_2), Span::Span(s2_1, s2_2)) => {
				s1_1.cmp(s2_1).then_with(|| s1_2.cmp(s2_2))
			}
			(Span::Point(p), Span::Span(s1, s2)) => match p.cmp(s1) {
				Ordering::Equal => Ordering::Equal,
				Ordering::Less => Ordering::Less,
				Ordering::Greater => {
					if p > s2 {
						Ordering::Greater
					} else {
						Ordering::Equal
					}
				}
			},
			(Span::Span(s1, s2), Span::Point(p)) => match s1.cmp(p) {
				Ordering::Equal => Ordering::Equal,
				Ordering::Less => {
					if s2 < p {
						Ordering::Less
					} else {
						Ordering::Equal
					}
				}
				Ordering::Greater => Ordering::Greater,
			},
		}
	}
}

impl PartialOrd for Span {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[cfg(test)]
mod test {
	use super::{Position, Span};
	use std::cmp::Ordering;

	#[test]
	fn ord_span() {
		let span = Span::Span(
			Position {
				line: 2,
				col: 6,
				index: 15,
			},
			Position {
				line: 2,
				col: 16,
				index: 25,
			},
		);

		let p1 = Span::Point(Position {
			line: 1,
			col: 6,
			index: 5,
		});

		assert_eq!(span.cmp(&p1), Ordering::Greater);
		assert_eq!(p1.cmp(&span), Ordering::Less);

		let p2 = Span::Point(Position {
			line: 2,
			col: 6,
			index: 15,
		});

		assert_eq!(span.cmp(&p2), Ordering::Equal);
		assert_eq!(p2.cmp(&span), Ordering::Equal);

		let p3 = Span::Point(Position {
			line: 2,
			col: 11,
			index: 20,
		});

		assert_eq!(span.cmp(&p3), Ordering::Equal);
		assert_eq!(p3.cmp(&span), Ordering::Equal);

		let p4 = Span::Point(Position {
			line: 2,
			col: 16,
			index: 25,
		});

		assert_eq!(span.cmp(&p4), Ordering::Equal);
		assert_eq!(p4.cmp(&span), Ordering::Equal);

		let p5 = Span::Point(Position {
			line: 3,
			col: 6,
			index: 30,
		});

		assert_eq!(span.cmp(&p5), Ordering::Less);
		assert_eq!(p5.cmp(&span), Ordering::Greater);
	}
}
