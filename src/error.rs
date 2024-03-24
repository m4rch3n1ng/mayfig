use thiserror::Error;

// todo better char conversion
#[derive(Debug, Error)]
pub enum Err {
	#[error("io error")]
	Io(#[from] std::io::Error),

	#[error("unknown escape sequence {0:?}")]
	UnknownEscape(char),
	#[error("unescaped control character {0:?}")]
	UnescapedControl(char),

	#[error("invalid boolean {0:?}")]
	InvalidBool(String),
	#[error("invalid number {0:?}")]
	InvalidNum(String),
	#[error("invalid type")]
	InvalidType,
	#[error("unexpected word {0:?}")]
	UnexpectedWord(String),

	#[error("unexpected char {0:?}, expected numeric")]
	ExpectedNumeric(char),
	#[error("unexpected char {0:?}, expected alphanumeric")]
	ExpectedAlphaNumeric(char),
	#[error("unexpected char {0:?}, expected enum")]
	ExpectedEnum(char),
	#[error("unexpected char {0:?}, expected alphabetic")]
	ExpectedAlphabetic(char),

	#[error("expected quote \" or ', got {0:?}")]
	ExpectedQuote(char),
	#[error("expected value assignment '=', '{{' or '[', got {0:?}")]
	ExpectedValue(char),
	#[error("expected map end '}}', got {0:?}")]
	ExpectedMapEnd(char),
	#[error("expected map '{{', got {0:?}")]
	ExpectedMap(char),
	#[error("expected seq '[', got {0:?}")]
	ExpectedSeq(char),
	#[error("expected end of sequence ']', got {0:?}")]
	ExpectedSeqEnd(char),

	#[error("expected delimiter after string, got {0:?}")]
	ExpectedDelimiter(char),

	#[error("unsupported none")]
	UnsupportedNone,
	#[error("unsupported type {0}")]
	UnsupportedType(&'static str),
	#[error("unsupportet map key type {0}")]
	UnsupportedMapKey(&'static str),

	#[error("invalid utf8")]
	InvalidUtf8,
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
