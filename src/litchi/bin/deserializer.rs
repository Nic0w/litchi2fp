use std::fmt::Display;

use serde::{
    de::{self, SeqAccess, Visitor},
    forward_to_deserialize_any,
};

use super::error::Error;

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

impl<'de> Deserializer<'de> {
    pub fn parse_u32(&mut self) -> u32 {
        let (bytes, remaining) = self.input.split_at(core::mem::size_of::<u32>());

        let value = u32::from_be_bytes(bytes.try_into().unwrap());

        self.input = remaining;

        value
    }

    fn parse_float(&mut self) -> f32 {
        let (bytes, remaining) = self.input.split_at(core::mem::size_of::<f32>());

        let value = f32::from_be_bytes(bytes.try_into().unwrap());

        self.input = remaining;

        value
    }

    fn parse_double(&mut self) -> f64 {
        let (bytes, remaining) = self.input.split_at(core::mem::size_of::<f64>());

        let value = f64::from_be_bytes(bytes.try_into().unwrap());

        self.input = remaining;

        value
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    forward_to_deserialize_any! {
        bool i8 i16 i64 i128 u8 u16 u64 u128
        char str string
        bytes byte_buf option unit unit_struct newtype_struct
        tuple_struct map enum identifier ignored_any
    }

    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedValue)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_u32() as i32)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse_float())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse_double())
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let size = self.parse_u32();

        let seq = SeqVisitor::new(self, size);

        visitor.visit_seq(seq)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let seq = SeqVisitor::new(self, len as u32);

        visitor.visit_seq(seq)
    }
}

struct SeqVisitor<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    size: u32,
}

impl<'a, 'de> SeqVisitor<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, size: u32) -> Self {
        SeqVisitor { de, size }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqVisitor<'a, 'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.size as usize)
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.size == 0 {
            return Ok(None);
        }

        self.size -= 1;

        seed.deserialize(&mut *self.de).map(Some)
    }
}
