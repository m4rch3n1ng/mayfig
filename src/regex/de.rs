use super::{FIELD, FLAGS_FIELD, NAME, REGEX_FIELD};
use crate::{de::read::Ref, Error};
#[cfg(feature = "value")]
use crate::{value::de::ValueVisitor, Regex, Value};
use serde_core::{
	de::{
		value::{BorrowedStrDeserializer, StrDeserializer},
		MapAccess, Visitor,
	},
	forward_to_deserialize_any,
};

pub struct RegexMapAcc<'de, 's> {
	regex: Option<(Ref<'de, 's, str>, Ref<'de, 's, str>)>,
}

impl<'de, 's> RegexMapAcc<'de, 's> {
	pub fn new(regex: Ref<'de, 's, str>, flags: Ref<'de, 's, str>) -> Self {
		RegexMapAcc {
			regex: Some((regex, flags)),
		}
	}
}

impl<'de> serde_core::de::MapAccess<'de> for RegexMapAcc<'de, '_> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: serde_core::de::DeserializeSeed<'de>,
	{
		if self.regex.is_some() {
			seed.deserialize(BorrowedStrDeserializer::new(FIELD))
				.map(Some)
		} else {
			Ok(None)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: serde_core::de::DeserializeSeed<'de>,
	{
		if let Some((regex, flags)) = self.regex.take() {
			seed.deserialize(RegexDeserializer::new(regex, flags))
		} else {
			panic!("next_value_seed called before next_key_seed")
		}
	}
}

pub struct RegexDeserializer<'de, 's> {
	regex: Option<Ref<'de, 's, str>>,
	flags: Option<Ref<'de, 's, str>>,
}

impl<'de, 's> RegexDeserializer<'de, 's> {
	pub fn new(regex: Ref<'de, 's, str>, flags: Ref<'de, 's, str>) -> Self {
		RegexDeserializer {
			regex: Some(regex),
			flags: Some(flags),
		}
	}
}

impl<'de> serde_core::Deserializer<'de> for RegexDeserializer<'de, '_> {
	type Error = Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		Err(serde_core::de::Error::invalid_type(
			serde_core::de::Unexpected::Other("regex"),
			&visitor,
		))
	}

	fn deserialize_struct<V>(
		self,
		name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		if name == NAME {
			visitor.visit_map(self)
		} else {
			Err(serde_core::de::Error::invalid_type(
				serde_core::de::Unexpected::Other("regex"),
				&visitor,
			))
		}
	}

	forward_to_deserialize_any! {
		bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
		bytes byte_buf option unit unit_struct newtype_struct seq tuple
		tuple_struct map enum identifier ignored_any
	}
}

impl<'de> MapAccess<'de> for RegexDeserializer<'de, '_> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
	where
		K: serde_core::de::DeserializeSeed<'de>,
	{
		if self.regex.is_some() {
			seed.deserialize(BorrowedStrDeserializer::new(REGEX_FIELD))
				.map(Some)
		} else if self.flags.is_some() {
			seed.deserialize(BorrowedStrDeserializer::new(FLAGS_FIELD))
				.map(Some)
		} else {
			Ok(None)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
	where
		V: serde_core::de::DeserializeSeed<'de>,
	{
		if let Some(regex) = self.regex.take() {
			match regex {
				Ref::Borrow(regex) => seed.deserialize(BorrowedStrDeserializer::new(regex)),
				Ref::Scratch(regex) => seed.deserialize(StrDeserializer::new(regex)),
			}
		} else if let Some(flags) = self.flags.take() {
			match flags {
				Ref::Borrow(flags) => seed.deserialize(BorrowedStrDeserializer::new(flags)),
				Ref::Scratch(flags) => seed.deserialize(StrDeserializer::new(flags)),
			}
		} else {
			panic!("next_value_seed called before next_key_seed")
		}
	}
}

#[cfg(feature = "value")]
pub enum VisitMap {
	Regex(Regex),
	Key(Value),
}

#[cfg(feature = "value")]
impl<'de> VisitMap {
	/// Determine the type of the map by deserializing it
	pub fn next_key_seed<V: serde_core::de::MapAccess<'de>>(
		visitor: &mut V,
	) -> Result<Option<Self>, V::Error> {
		let mut key = None;
		let Some(()) = visitor.next_key_seed(RegexOrMapKey::new(&mut key))? else {
			return Ok(None);
		};
		let result = if let Some(key) = key {
			VisitMap::Key(key)
		} else {
			let regex: Regex = visitor.next_value()?;
			VisitMap::Regex(regex)
		};
		Ok(Some(result))
	}
}

#[cfg(feature = "value")]
struct RegexOrMapKey<'m> {
	key: &'m mut Option<Value>,
	vis: ValueVisitor,
}

#[cfg(feature = "value")]
impl<'m> RegexOrMapKey<'m> {
	fn new(key: &'m mut Option<Value>) -> Self {
		*key = None;
		Self {
			key,
			vis: ValueVisitor,
		}
	}
}

#[cfg(feature = "value")]
impl<'de> serde_core::de::DeserializeSeed<'de> for RegexOrMapKey<'_> {
	type Value = ();

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: serde_core::de::Deserializer<'de>,
	{
		deserializer.deserialize_any(self)
	}
}

#[cfg(feature = "value")]
impl<'de> serde_core::de::Visitor<'de> for RegexOrMapKey<'_> {
	type Value = ();

	fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		formatter.write_str("a string key")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		*self.key = Some(self.vis.visit_bool(v)?);
		Ok(())
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		*self.key = Some(self.vis.visit_u64(v)?);
		Ok(())
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		*self.key = Some(self.vis.visit_i64(v)?);
		Ok(())
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		*self.key = Some(self.vis.visit_f64(v)?);
		Ok(())
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		if v == FIELD {
			*self.key = None;
			Ok(())
		} else {
			*self.key = Some(self.vis.visit_str(v)?);
			Ok(())
		}
	}

	fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		if v == FIELD {
			*self.key = None;
			Ok(())
		} else {
			*self.key = Some(self.vis.visit_borrowed_str(v)?);
			Ok(())
		}
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: serde_core::de::Error,
	{
		if v == FIELD {
			*self.key = None;
			Ok(())
		} else {
			*self.key = Some(self.vis.visit_string(v)?);
			Ok(())
		}
	}

	fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
	where
		A: serde_core::de::SeqAccess<'de>,
	{
		*self.key = Some(self.vis.visit_seq(seq)?);
		Ok(())
	}

	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	where
		A: serde_core::de::EnumAccess<'de>,
	{
		*self.key = Some(self.vis.visit_enum(data)?);
		Ok(())
	}
}
