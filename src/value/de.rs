use super::{Map, Number};
use crate::Value;
use serde::{
	de::{VariantAccess, Visitor},
	Deserialize,
};

struct ValueVisitor;

impl<'de> Deserialize<'de> for Value {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_any(ValueVisitor)
	}
}

impl<'de> Visitor<'de> for ValueVisitor {
	type Value = Value;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("a valid mayfig value")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Value::Bool(v))
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let num = Number::from(v);
		Ok(Value::Number(num))
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let num = Number::from(v);
		Ok(Value::Number(num))
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		let num = Number::try_from(v).expect("the Deseralizer should never return NaN");
		Ok(Value::Number(num))
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		self.visit_string(v.to_owned())
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(Value::String(v))
	}

	fn visit_seq<A>(self, mut vis: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::SeqAccess<'de>,
	{
		let mut seq = Vec::new();
		while let Some(val) = vis.next_element()? {
			seq.push(val)
		}
		Ok(Value::Seq(seq))
	}

	fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::EnumAccess<'de>,
	{
		let (variant, access) = data.variant::<String>()?;
		let values = access.newtype_variant::<Vec<Value>>()?;

		Ok(Value::Tagged(variant, values))
	}

	fn visit_map<A>(self, mut vis: A) -> Result<Self::Value, A::Error>
	where
		A: serde::de::MapAccess<'de>,
	{
		let mut map = Map::new();
		while let Some((key, val)) = vis.next_entry()? {
			map.insert(key, val);
		}
		Ok(Value::Map(map))
	}
}
