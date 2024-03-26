use self::serializer::{MapKeySerializer, MapValSerializer};
use crate::error::Err;
use serde::Serialize;

mod serializer;

pub struct Serializer<'a> {
	/// current indentation level
	indent_level: usize,
	indent: &'static str,
	writer: &'a mut String,
}

impl<'a> Serializer<'a> {
	pub fn new(writer: &'a mut String) -> Self {
		Serializer {
			writer,
			indent: "\t",
			indent_level: 0,
		}
	}

	pub fn with_indent(writer: &'a mut String, indent: &'static str) -> Self {
		Serializer {
			writer,
			indent,
			indent_level: 0,
		}
	}
}

impl<'ser> Serializer<'ser> {
	fn indent(&mut self) -> Result<(), Err> {
		for _ in 0..self.indent_level {
			self.writer.push_str(self.indent);
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
		let s = if v { "true" } else { "false" };
		self.writer.push_str(s);
		Ok(())
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		self.serialize_i64(i64::from(v))
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		let v = v.to_string();
		self.writer.push_str(&v);
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.serialize_u64(u64::from(v))
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		let v = v.to_string();
		self.writer.push_str(&v);
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		self.serialize_f64(f64::from(v))
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		let v = v.to_string();
		self.writer.push_str(&v);
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		let mut buf = [0; 4];
		let v = v.encode_utf8(&mut buf);
		self.serialize_str(v)
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
		self.writer.push('[');
		self.writer.push(' ');
		Ok(self)
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
		if self.indent_level == 0 {
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

	fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
		self.indent()?;

		let mapk = MapKeySerializer::new(self);
		key.serialize(mapk)
	}

	fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		let mapv = MapValSerializer::new(self);
		value.serialize(mapv)?;

		self.writer.push('\n');
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		serde::ser::SerializeStruct::end(self)
	}
}

impl<'a> serde::ser::SerializeSeq for &mut Serializer<'a> {
	type Ok = ();
	type Error = Err;

	fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		value.serialize(&mut **self)?;
		self.writer.push(' ');
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.push(']');
		Ok(())
	}
}

impl<'ser> serde::ser::SerializeStruct for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: Serialize + ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error> {
		self.indent()?;

		let mapk = MapKeySerializer::new(self);
		key.serialize(mapk)?;

		let mapv = MapValSerializer::new(self);
		value.serialize(mapv)?;

		self.writer.push('\n');
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		if let Some(indent) = self.indent_level.checked_sub(1) {
			self.indent_level = indent;
			self.indent()?;
			self.writer.push('}');
		}

		Ok(())
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeStructVariant for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: Serialize + ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error> {
		todo!()
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}

impl<'ser> serde::ser::SerializeTuple for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		serde::ser::SerializeSeq::serialize_element(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		serde::ser::SerializeSeq::end(self)
	}
}

#[allow(unused_variables)]
impl<'ser> serde::ser::SerializeTupleStruct for &mut Serializer<'ser> {
	type Ok = ();
	type Error = Err;

	fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
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

	fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
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
