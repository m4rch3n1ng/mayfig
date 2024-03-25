use self::serializer::{MapKeySerializer, MapValSerializer};
use crate::error::Err;
use serde::Serialize;

mod serializer;

pub struct Serializer<'id, W:std::io::Write> {
	/// current indentation level
	indent_level: usize,
	indent: &'id [u8],
	writer: W,
}

impl<W: std::io::Write> Serializer<'static, W> {
	pub fn new(writer: W) -> Self {
		Serializer {
			writer,
			indent: b"\t",
			indent_level: 0,
		}
	}
}

impl<'id, W: std::io::Write> Serializer<'id, W> {
	pub fn with_indent(writer: W, indent: &'id [u8]) -> Self {
		Serializer {
			writer,
			indent,
			indent_level: 0,
		}
	}
}

impl<'id, W: std::io::Write> Serializer<'id, W> {
	fn indent(&mut self) -> Result<(), Err> {
		for _ in 0..self.indent_level {
			self.writer.write_all(self.indent)?;
		}

		Ok(())
	}
}

#[allow(unused_variables)]
impl<'id, W: std::io::Write> serde::ser::Serializer for &mut Serializer<'id, W> {
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
		let s = if v { b"true" as &[u8] } else { b"false" };
		self.writer.write_all(s)?;
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

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		let mut buf = [0; 4];
		let v = v.encode_utf8(&mut buf);
		self.serialize_str(v)
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		let v = format!("{:?}", v);
		self.writer.write_all(v.as_bytes())?;
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
		self.writer.write_all(b"[")?;
		self.writer.write_all(b" ")?;
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
			self.writer.write_all(b" {\n")?;
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
impl<'id, W: std::io::Write> serde::ser::SerializeMap for &mut Serializer<'id, W> {
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

		self.writer.write_all(b"\n")?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		serde::ser::SerializeStruct::end(self)
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeSeq for &mut Serializer<'id, W> {
	type Ok = ();
	type Error = Err;

	fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
		value.serialize(&mut **self)?;
		self.writer.write_all(b" ")?;
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.writer.write_all(b"]")?;
		Ok(())
	}
}

impl<'id, W: std::io::Write> serde::ser::SerializeStruct for &mut Serializer<'id, W> {
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

#[allow(unused_variables)]
impl<'id, W: std::io::Write> serde::ser::SerializeStructVariant for &mut Serializer<'id, W> {
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

impl<'id, W: std::io::Write> serde::ser::SerializeTuple for &mut Serializer<'id, W> {
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
impl<'id, W: std::io::Write> serde::ser::SerializeTupleStruct for &mut Serializer<'id, W> {
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
impl<'id, W: std::io::Write> serde::ser::SerializeTupleVariant for &mut Serializer<'id, W> {
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
	let mut vec = Vec::new();

	let mut serializer = Serializer::new(&mut vec);
	value.serialize(&mut serializer)?;

	let string = String::from_utf8(vec).expect("should never emit invalid utf8");
	Ok(string)
}
