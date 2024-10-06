use super::Serializer;
use crate::{error::ErrorCode, Error};
use serde::Serialize;

pub struct NewtypeVariantSerializer<'a, 'id, W: std::io::Write> {
	ser: &'a mut Serializer<'id, W>,
}

impl<'a, 'id, W: std::io::Write> NewtypeVariantSerializer<'a, 'id, W> {
	pub fn new(ser: &'a mut Serializer<'id, W>) -> Self {
		NewtypeVariantSerializer { ser }
	}
}

impl<'a, 'id, W: std::io::Write> serde::ser::Serializer for NewtypeVariantSerializer<'a, 'id, W> {
	type Ok = ();
	type Error = Error;

	type SerializeMap = &'a mut Serializer<'id, W>;
	type SerializeSeq = &'a mut Serializer<'id, W>;
	type SerializeTuple = &'a mut Serializer<'id, W>;
	type SerializeTupleStruct = &'a mut Serializer<'id, W>;
	type SerializeTupleVariant = &'a mut Serializer<'id, W>;
	type SerializeStruct = &'a mut Serializer<'id, W>;
	type SerializeStructVariant = &'a mut Serializer<'id, W>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_bool(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_i8(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_i16(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_i32(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_i64(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_u8(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_u16(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_u32(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_u64(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_f32(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_f64(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		let mut buf = [0; 4];
		let v = v.encode_utf8(&mut buf);
		self.serialize_str(v)
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_str(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		self.ser.serialize_bytes(v)?;
		self.ser.writer.write_all(b" ]")?;
		Ok(())
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedNone))
	}

	fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedUnit))
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedUnit))
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	fn serialize_newtype_struct<T: Serialize + ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error> {
		value.serialize(self)
	}

	#[expect(unused_variables)]
	fn serialize_newtype_variant<T>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.ser.writer.write_all(b" [ ")?;
		Ok(self.ser)
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		self.serialize_seq(Some(len))
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		self.serialize_tuple(len)
	}

	#[expect(unused_variables)]
	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		todo!();
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		self.ser.indent_level += 1;
		self.ser.serialize_map(len)
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(Some(len))
	}

	#[expect(unused_variables)]
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
