use super::Serializer;
use crate::error::Err;

pub struct MapKeySerializer<'a, 'ser> {
	ser: &'a mut Serializer<'ser>,
}

impl<'a, 'ser> MapKeySerializer<'a, 'ser> {
	pub fn new(ser: &'a mut Serializer<'ser>) -> Self {
		MapKeySerializer { ser }
	}
}

#[allow(unused_variables)]
impl<'a, 'ser> serde::ser::Serializer for MapKeySerializer<'a, 'ser> {
	type Ok = ();
	type Error = Err;

	type SerializeMap = &'a mut Serializer<'ser>;
	type SerializeSeq = &'a mut Serializer<'ser>;
	type SerializeTuple = &'a mut Serializer<'ser>;
	type SerializeTupleStruct = &'a mut Serializer<'ser>;
	type SerializeTupleVariant = &'a mut Serializer<'ser>;
	type SerializeStruct = &'a mut Serializer<'ser>;
	type SerializeStructVariant = &'a mut Serializer<'ser>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_u64(v)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		let vv = v.chars().any(|ch| ch.is_alphanumeric())
			&& v.chars().next().is_some_and(|ch| ch.is_alphabetic());
		if vv {
			let v = v.to_string();
			self.ser.writer.push_str(&v);
		} else {
			self.ser.serialize_str(v)?;
		}

		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		todo!()
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		todo!()
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		todo!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		todo!()
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		todo!()
	}

	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		todo!()
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		todo!()
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		todo!()
	}

	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		todo!()
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		todo!()
	}
}

pub struct MapValSerializer<'a, 'ser> {
	ser: &'a mut Serializer<'ser>,
}

impl<'a, 'ser> MapValSerializer<'a, 'ser> {
	pub fn new(ser: &'a mut Serializer<'ser>) -> Self {
		MapValSerializer { ser }
	}
}

#[allow(unused_variables)]
impl<'a, 'ser> serde::ser::Serializer for MapValSerializer<'a, 'ser> {
	type Ok = ();
	type Error = Err;

	type SerializeMap = &'a mut Serializer<'ser>;
	type SerializeSeq = &'a mut Serializer<'ser>;
	type SerializeTuple = &'a mut Serializer<'ser>;
	type SerializeTupleStruct = &'a mut Serializer<'ser>;
	type SerializeTupleVariant = &'a mut Serializer<'ser>;
	type SerializeStruct = &'a mut Serializer<'ser>;
	type SerializeStructVariant = &'a mut Serializer<'ser>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_bool(v)
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_i8(v)
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_i16(v)
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_i32(v)
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_i64(v)
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_u8(v)
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_u16(v)
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_u32(v)
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_u64(v)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.push_str(" = ");
		self.ser.serialize_str(v)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedNone)
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		todo!()
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		todo!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.ser.writer.push_str(" [ ");
		Ok(self.ser)
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		todo!()
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		self.ser.indent_level += 1;
		self.ser.serialize_map(len)
	}

	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		self.ser.indent_level += 1;
		self.ser.serialize_struct(name, len)
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		todo!()
	}
}
