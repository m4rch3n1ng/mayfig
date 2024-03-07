use crate::error::Err;
use serde::Serialize;

use self::serializer::{MapKeySerializer, MapValSerializer};

mod serializer;

pub struct Serializer<'a> {
	indent: usize,
	writer: &'a mut String,
}

impl<'a> Serializer<'a> {
	fn new(writer: &'a mut String) -> Self {
		Serializer { writer, indent: 0 }
	}
}

impl<'ser> Serializer<'ser> {
	fn indent(&mut self) -> Result<(), Err> {
		for _ in 0..self.indent {
			self.writer.push('\t');
		}

		Ok(())
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::Serializer for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	type SerializeMap = Self;
	type SerializeSeq = Self;
	type SerializeStruct = Self;
	type SerializeStructVariant = Self;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Self;

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
		let v = v.to_string();
		self.writer.push_str(&v);
		Ok(())
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
		let v = format!("{:?}", v);
		self.writer.push_str(&v);
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
		T: Serialize,
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
		T: Serialize,
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
		T: Serialize,
	{
		todo!()
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		self.writer.push('[');
		self.writer.push(' ');
		Ok(self)
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
		if self.indent == 0 {
			Ok(self)
		} else {
			self.writer.push_str(" {\n");
			Ok(self)
		}
	}

	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(Some(len))
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

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeMap for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl<'a> serde::ser::SerializeSeq for &mut Serializer<'a> {
	type Ok = ();
	type Error = Err;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		value.serialize(&mut **self)?;
		self.writer.push(' ');
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.push(']');
		Ok(())
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeStruct for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
			self.indent()?;

			let mapk = MapKeySerializer::new(self);
			key.serialize(mapk)?;

			let mapv = MapValSerializer::new(self);
			value.serialize(mapv)?;

			self.writer.push('\n');
			Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		if let Some(indent) = self.indent.checked_sub(1) {
			self.indent = indent;
			self.indent()?;
			self.writer.push_str("}\n");
		}

		Ok(())
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeStructVariant for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeTuple for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeTupleStruct for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeTupleVariant for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

pub fn to_string<T: ?Sized + Serialize>(value: &T) -> Result<String, Err> {
	let mut string = String::new();
	let mut serializer = Serializer::new(&mut string);
	value.serialize(&mut serializer)?;
	Ok(string)
}
