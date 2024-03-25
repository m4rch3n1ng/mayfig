use super::Serializer;
use crate::error::Err;
use serde::Serialize;

pub struct MapKeySerializer<'a, 'id, W: std::io::Write> {
	ser: &'a mut Serializer<'id, W>,
}

impl<'a, 'id, W: std::io::Write> MapKeySerializer<'a, 'id, W> {
	pub fn new(ser: &'a mut Serializer<'id, W>) -> Self {
		MapKeySerializer { ser }
	}
}

#[allow(unused_variables)]
impl<'a, 'id, W: std::io::Write> serde::ser::Serializer for MapKeySerializer<'a, 'id, W> {
	type Ok = ();
	type Error = Err;

	type SerializeMap = &'a mut Serializer<'id, W>;
	type SerializeSeq = &'a mut Serializer<'id, W>;
	type SerializeTuple = &'a mut Serializer<'id, W>;
	type SerializeTupleStruct = &'a mut Serializer<'id, W>;
	type SerializeTupleVariant = &'a mut Serializer<'id, W>;
	type SerializeStruct = &'a mut Serializer<'id, W>;
	type SerializeStructVariant = &'a mut Serializer<'id, W>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_bool(v)
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_i8(v)
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_i16(v)
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_i32(v)
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_i64(v)
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_u8(v)
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_u16(v)
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_u32(v)
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_u64(v)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_f32(v)
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_f64(v)
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_char(v)
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		let mut vit = v.chars();
		let ident = vit.next().is_some_and(char::is_alphabetic) && vit.all(char::is_alphanumeric);
		if ident {
			self.ser.writer.write_all(v.as_bytes())?;
		} else {
			self.ser.serialize_str(v)?;
		}

		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedMapKey("bytes"))
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedNone)
	}

	fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		self.ser.serialize_some(value)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedType("unit"))
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedType(name))
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	fn serialize_newtype_struct<T: Serialize + ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: Serialize + ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(Err::UnsupportedMapKey("seq"))
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(Err::UnsupportedMapKey("tuple"))
	}

	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(Err::UnsupportedMapKey("tuple"))
	}

	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(Err::UnsupportedMapKey("tuple"))
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(Err::UnsupportedMapKey("map"))
	}

	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		Err(Err::UnsupportedMapKey("struct"))
	}

	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		Err(Err::UnsupportedMapKey("struct"))
	}
}

pub struct MapValSerializer<'a, 'id, W: std::io::Write> {
	ser: &'a mut Serializer<'id, W>,
}

impl<'a, 'id, W: std::io::Write> MapValSerializer<'a, 'id, W> {
	pub fn new(ser: &'a mut Serializer<'id, W>) -> Self {
		MapValSerializer { ser }
	}
}

#[allow(unused_variables)]
impl<'a, 'id, W: std::io::Write> serde::ser::Serializer for MapValSerializer<'a, 'id, W> {
	type Ok = ();
	type Error = Err;

	type SerializeMap = &'a mut Serializer<'id, W>;
	type SerializeSeq = &'a mut Serializer<'id, W>;
	type SerializeTuple = &'a mut Serializer<'id, W>;
	type SerializeTupleStruct = &'a mut Serializer<'id, W>;
	type SerializeTupleVariant = &'a mut Serializer<'id, W>;
	type SerializeStruct = &'a mut Serializer<'id, W>;
	type SerializeStructVariant = &'a mut Serializer<'id, W>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_bool(v)
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_i8(v)
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_i16(v)
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_i32(v)
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_i64(v)
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_u8(v)
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_u16(v)
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_u32(v)
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_u64(v)
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_f32(v)
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_f64(v)
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_char(v)
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" = ")?;
		self.ser.serialize_str(v)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedNone)
	}

	fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedType("unit"))
	}

	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(Err::UnsupportedType(name))
	}

	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	fn serialize_newtype_struct<T: Serialize + ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: Serialize + ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
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
