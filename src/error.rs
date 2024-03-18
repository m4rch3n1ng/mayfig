use thiserror::Error;

// todo better char conversion
#[derive(Debug, Error)]
pub enum Err {
	#[error("io error")]
	Io(#[source] std::io::Error),
	#[error("unknown escape sequence {0:?}")]
	UnknownEscape(char),
	#[error("unescaped control character {0:?}")]
	UnescapedControl(char),
	#[error("invalid boolean {0:?}")]
	InvalidBool(String),
	#[error("invalid number {0:?}")]
	InvalidNum(String),
	#[error("unexpected word {0:?}")]
	UnexpectedWord(String),
	#[error("invalid utf8")]
	InvalidUtf8,
	#[error("expected {0:?}, got {1:?}")]
	Expected(char, char),
	#[error("unexpected char {0:?}, expected {1}")]
	UnexpectedChar(char, &'static str),
	#[error("unsupported none")]
	UnsupportedNone,
	#[error("expected end of sequence")]
	ExpectedSeqEnd,
	#[error("expected delimiter after string, got {0:?}")]
	ExpectedDelimiter(char),
	#[error("end of file")]
	Eof,
	#[error("custom: {0}")]
	Custom(String),
}

impl serde::de::Error for Err {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		let msg = msg.to_string();
		Err::Custom(msg)
	}
}

impl serde::ser::Error for Err {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		let msg = msg.to_string();
		Err::Custom(msg)
	}
}
