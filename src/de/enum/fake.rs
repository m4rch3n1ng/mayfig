use crate::Error;
use serde_core::{forward_to_deserialize_any, Deserializer};
use std::borrow::Cow;

pub struct FakeStringDeserializer<'a> {
	string: Cow<'a, str>,
}

impl<'a> FakeStringDeserializer<'a> {
	pub fn new(string: Cow<'a, str>) -> Self {
		FakeStringDeserializer { string }
	}
}

impl<'de> Deserializer<'de> for FakeStringDeserializer<'de> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::Visitor<'de>,
	{
		match self.string {
			Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
			Cow::Owned(s) => visitor.visit_string(s),
		}
	}

	forward_to_deserialize_any! {
		bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64
		char str string bytes byte_buf option unit
		unit_struct newtype_struct seq tuple tuple_struct
		map struct enum identifier ignored_any
	}
}
