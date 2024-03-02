use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
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
