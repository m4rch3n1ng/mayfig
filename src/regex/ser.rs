use super::Regex;
use crate::{Error, Serializer};
use serde_core::Serialize;

pub struct SerializeRegex<'a, 'id, W> {
	ser: &'a mut Serializer<'id, W>,
	regex: Option<String>,
	flags: Option<String>,
}

impl<'a, 'id, W> SerializeRegex<'a, 'id, W> {
	pub fn new(ser: &'a mut Serializer<'id, W>) -> Self {
		SerializeRegex {
			ser,
			regex: None,
			flags: None,
		}
	}
}

impl<W: std::io::Write> serde_core::ser::SerializeMap for SerializeRegex<'_, '_, W> {
	type Ok = ();
	type Error = Error;

	fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		unreachable!("regexes should only be serialized as structs")
	}

	fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		unreachable!("regexes should only be serialized as structs")
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unreachable!("regexes should only be serialized as structs")
	}
}

impl<W: std::io::Write> serde_core::ser::SerializeStruct for SerializeRegex<'_, '_, W> {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		if key == crate::regex::REGEX_FIELD {
			self.regex = Some(value.serialize(StringSerializer)?);
		} else if key == crate::regex::FLAGS_FIELD {
			self.flags = Some(value.serialize(StringSerializer)?);
		}

		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		// TODO: don't crash

		let regex = self.regex.unwrap();
		let flags = self.flags.unwrap();

		write!(self.ser.writer, "{}", Regex { regex, flags })?;
		Ok(())
	}
}

impl<W: std::io::Write> serde_core::ser::SerializeStructVariant for SerializeRegex<'_, '_, W> {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		unreachable!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}
}

struct StringSerializer;

// TODO? return Regex
// TODO? error type
impl serde_core::ser::Serializer for StringSerializer {
	type Ok = String;
	type Error = Error;
	type SerializeSeq = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeMap = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = serde_core::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeStructVariant = serde_core::ser::Impossible<Self::Ok, Self::Error>;

	fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_owned())
	}

	fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde_core::ser::Serialize + ?Sized,
	{
		unreachable!()
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}

	fn serialize_newtype_struct<T>(
		self,
		_name: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde_core::ser::Serialize + ?Sized,
	{
		unreachable!()
	}

	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde_core::ser::Serialize + ?Sized,
	{
		unreachable!()
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		unreachable!()
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		unreachable!()
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		unreachable!()
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		unreachable!()
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		unreachable!()
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		unreachable!()
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		unreachable!()
	}
}
