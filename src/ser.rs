use self::map::{MapKeySerializer, MapValSerializer};
use crate::{error::ErrorCode, Error};
use serde::Serialize;

mod map;

pub struct Serializer<'id, W> {
	/// current level of indentation
	indent_level: usize,
	indent: &'id [u8],
	writer: W,
}

impl<W: std::io::Write> Serializer<'static, W> {
	pub fn new(writer: W) -> Self {
		Serializer {
			indent_level: 0,
			indent: b"\t",
			writer,
		}
	}
}

impl<'id, W: std::io::Write> Serializer<'id, W> {
	pub fn with_indent(writer: W, indent: &'id [u8]) -> Self {
		Serializer {
			indent_level: 0,
			indent,
			writer,
		}
	}
}

impl<'id, W: std::io::Write> Serializer<'id, W> {
	fn indent(&mut self) -> Result<(), Error> {
		for _ in 0..self.indent_level {
			self.writer.write_all(self.indent)?;
		}

		Ok(())
	}
}

impl<'id, W: std::io::Write> serde::ser::Serializer for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	type SerializeMap = Self;
	type SerializeSeq = Self;
	type SerializeStruct = Self;
	type SerializeStructVariant = Self;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Self;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		let s = if v { b"true" as &[u8] } else { b"false" };
		self.writer.write_all(s)?;
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		let mut buffer = itoa::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		let mut buffer = ryu::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		let mut buffer = ryu::Buffer::new();
		let s = buffer.format(v);
		self.writer.write_all(s.as_bytes())?;
		Ok(())
	}

	#[expect(unused_variables)]
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!();
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		let v = format!("{:?}", v);
		self.writer.write_all(v.as_bytes())?;
		Ok(())
	}

	#[expect(unused_variables)]
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedNone))
	}

	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedUnit))
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(Error::new(ErrorCode::UnsupportedUnit))
	}

	#[expect(unused_variables)]
	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		todo!();
	}

	fn serialize_newtype_struct<T>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
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

	#[expect(unused_variables)]
	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		todo!();
	}

	#[expect(unused_variables)]
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		todo!();
	}

	#[expect(unused_variables)]
	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		todo!();
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

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		if self.indent_level == 0 {
			Ok(self)
		} else {
			self.writer.write_all(b" {")?;
			Ok(self)
		}
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
		todo!();
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeMap for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	#[expect(unused_variables)]
	fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeSeq for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!();
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeStruct for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		self.indent()?;

		let map_key = MapKeySerializer::new(self);
		key.serialize(map_key)?;

		let map_val = MapValSerializer::new(self);
		value.serialize(map_val)?;

		self.writer.write_all(b"\n")?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		if let Some(indent) = self.indent_level.checked_sub(1) {
			self.indent_level = indent;
			self.indent()?;
			self.writer.write_all(b"}")?;
		}

		Ok(())
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeStructVariant for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!();
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeTuple for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!();
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeTupleStruct for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!();
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeTupleVariant for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Error;

	#[expect(unused_variables)]
	fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		todo!();
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!();
	}
}

pub fn to_string<T: ?Sized + Serialize>(value: &T) -> Result<String, Error> {
	let mut buf = Vec::with_capacity(128);

	let mut serializer = Serializer::new(&mut buf);
	value.serialize(&mut serializer)?;

	let string = String::from_utf8(buf).expect("should never emit invalid utf8");
	Ok(string)
}
