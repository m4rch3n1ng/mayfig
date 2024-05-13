use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("custom: {0}")]
	Custom(String),
}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		let msg = msg.to_string();
		Error::Custom(msg)
	}
}
