//! the internal module for regex support
//!
//! the top-level module holds the actual [`Regex`] struct and a few helpers,
//! while the more specialized helper functions for serialization and
//! deserialization ! are held in the [`de`] and [`ser`] modules respectively.
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

/// the regex type in mayfig
///
/// the actual regex syntax is similar to that of ecmascript, with the pattern
/// inside of two slashes and optional flags after that:
///
/// ```text
/// /pattern/f
///  ^^^^^^^ ^ flags
///  | pattern
/// ```
///
/// due to limitations of the serde traits, it is not possible to have
/// non-standard types in the format without some hacky workarounds by encoding
/// regex types via other types, that are actually supported by serde.
///
/// that is why you cannot implement deserialization for regex yourself and
/// have to go through this struct first, before using the output from this for
/// constructing your own types.
///
/// # Example
///
/// ```
/// use regex::Regex;
/// use serde::{Deserialize, Deserializer};
///
/// struct Match(Regex);
///
/// impl<'de> Deserialize<'de> for Match {
///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
///     where
///         D: Deserializer<'de>,
///     {
///         // use the Regex struct in mayfig for deserialization
///         let regex = mayfig::Regex::deserialize(deserializer)?;
///         // i'm ignoring the flags for simplicity in this example, but you
///         // can use them however you like.
///         let regex = Regex::new(&regex.pattern).map_err(serde::de::Error::custom)?;
///         Ok(Match(regex))
///     }
/// }
///
/// # let input = r"/regex/i";
/// # let thing = mayfig::from_str::<Match>(input).unwrap();
/// # assert_eq!(thing.0.as_str(), "regex");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Regex {
	/// the pattern part of the regex.
	///
	/// ```text
	/// /pattern/f
	///  ^^^^^^^
	///  | pattern
	/// ```
	///
	/// if you want to have a literal `/` in your pattern, you will have to escape
	/// it like `\/`, as that will otherwise close the regex literal.
	pub pattern: String,
	/// the flags part of the regex.
	///
	/// ```text
	/// /pattern/f
	///          ^ flags
	/// ```
	///
	/// flags are allowed to be multile characters, however they can only be ascii
	/// alphabetic characters. they are not otherwise validated and can contain any
	/// ascii letters.
	///
	/// if no flags are given this will be the empty string.
	pub flags: String,
}

impl Display for Regex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "/{}/{}", self.pattern, self.flags)
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
				// TODO: don't crash
				let (k, pattern) = map.next_entry::<&str, String>()?.unwrap();
				if k != PATTERN_FIELD {
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

				let regex = Regex { pattern, flags };
				Ok(regex)
			}
		}

		static FIELDS: [&str; 2] = [PATTERN_FIELD, FLAGS_FIELD];
		deserializer.deserialize_struct(NAME, &FIELDS, Vis)
	}
}

impl Serialize for Regex {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde_core::Serializer,
	{
		let mut ser = serializer.serialize_struct(NAME, 2)?;
		ser.serialize_field(PATTERN_FIELD, &self.pattern)?;
		ser.serialize_field(FLAGS_FIELD, &self.flags)?;
		ser.end()
	}
}

pub const NAME: &str = "$__mayfig_private_Regex";
pub const PATTERN_FIELD: &str = "$__mayfig_private_regex_pattern";
pub const FLAGS_FIELD: &str = "$__mayfig_private_regex_flags";

pub fn is_regex(name: &str) -> bool {
	name == NAME
}
