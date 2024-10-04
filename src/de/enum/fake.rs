use crate::Error;
use serde::{forward_to_deserialize_any, Deserializer};

pub struct FakeStringDeserializer {
	string: String,
}

impl FakeStringDeserializer {
	pub fn new(string: String) -> Self {
		FakeStringDeserializer { string }
	}
}

impl<'de> Deserializer<'de> for FakeStringDeserializer {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_string(self.string)
	}

	forward_to_deserialize_any! {
		bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64
		char str string bytes byte_buf option unit
		unit_struct newtype_struct seq tuple tuple_struct
		map struct enum identifier ignored_any
	}
}
