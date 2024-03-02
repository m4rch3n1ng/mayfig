use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
	#[error("invalid number")]
	InvalidNum,
	#[error("expected {0:?}, got {1:?}")]
	Expected(char, char),
	#[error("unexpected char {0:?}")]
	UnexpectedChar(char),
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
