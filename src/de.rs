use self::read::{Read, StrRead};
use crate::error::Error;

mod read;

pub struct Deserializer<R> {
	read: R,
	indent: usize,
	scratch: Vec<u8>,
}

impl<'de, R: Read<'de>> Deserializer<R> {
	fn new(read: R) -> Self {
		Deserializer {
			read,
			indent: 0,
			scratch: Vec::new(),
		}
	}
}

impl<'de> Deserializer<StrRead<'de>> {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(input: &'de str) -> Self {
		let read = StrRead::new(input);
		Deserializer::new(read)
	}
}

pub fn from_str<'a, T>(input: &'a str) -> Result<T, Error>
where
	T: serde::de::Deserialize<'a>,
{
	let mut deserializer = Deserializer::from_str(input);
	let t = T::deserialize(&mut deserializer);
	t
}
