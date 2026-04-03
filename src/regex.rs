//! TODO: docs regex stuff etc
//!
//! this is mostly inspired from the rust [toml crate](https://github.com/toml-rs/toml),
//! specifically from the following files:
//!
//! - <https://github.com/toml-rs/toml/blob/main/crates/toml_datetime/src/de.rs>
//! - <https://github.com/toml-rs/toml/blob/main/crates/toml/src/value.rs>
//!
//! as well as a few other spots.

use serde_core::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use std::fmt::Display;

pub mod de;
pub mod ser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Regex {
	pub regex: String,
	pub flags: String,
}

impl Display for Regex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "/{}/{}", self.regex, self.flags)
	}
}

impl<'de> Deserialize<'de> for Regex {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde_core::Deserializer<'de>,
	{
		struct Vis;

		impl<'v> Visitor<'v> for Vis {
			type Value = Regex;

			fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				f.write_str("a regex")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: serde_core::de::MapAccess<'v>,
			{
				let (k, regex) = map.next_entry::<&str, String>()?.unwrap();
				if k != REGEX_FIELD {
					return Err(serde_core::de::Error::invalid_type(
						serde_core::de::Unexpected::Other("map"),
						&self,
					));
				}

				let (k, flags) = map.next_entry::<&str, String>()?.unwrap();
				if k != FLAGS_FIELD {
					return Err(serde_core::de::Error::invalid_type(
						serde_core::de::Unexpected::Other("map"),
						&self,
					));
				}

				let regex = Regex { regex, flags };
				Ok(regex)
			}
		}

		static FIELDS: [&str; 2] = [REGEX_FIELD, FLAGS_FIELD];
		deserializer.deserialize_struct(NAME, &FIELDS, Vis)
	}
}

impl Serialize for Regex {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde_core::Serializer,
	{
		let mut ser = serializer.serialize_struct(NAME, 2)?;
		ser.serialize_field(REGEX_FIELD, &self.regex)?;
		ser.serialize_field(FLAGS_FIELD, &self.flags)?;
		ser.end()
	}
}

pub const NAME: &str = "$__mayfig_private_Regex";
pub const FIELD: &str = "$__mayfig_private_regex";
pub const REGEX_FIELD: &str = "$__mayfig_private_regex_regex";
pub const FLAGS_FIELD: &str = "$__mayfig_private_regex_flags";

pub fn is_regex(name: &str) -> bool {
	name == NAME
}
