use super::Value;
use serde_core::Serialize;

impl Serialize for Value {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde_core::Serializer,
	{
		match self {
			Value::String(s) => serializer.serialize_str(s),
			Value::Number(n) => n.serialize(serializer),
			Value::Bool(b) => serializer.serialize_bool(*b),
			Value::Seq(seq) => seq.serialize(serializer),
			Value::Map(map) => map.serialize(serializer),
			Value::Tagged(tag, value) => {
				// due to [a limitation in serde](https://github.com/serde-rs/serde/issues/2218) this
				// function call has to leak the tag string.
				serializer.serialize_newtype_variant("Value", 0, tag.clone().leak(), &value)
			}
		}
	}
}
