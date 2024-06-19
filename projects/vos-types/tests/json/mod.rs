use std::fmt::{Debug, Display, Formatter};
use vos_types::{
    encoder::{DictionaryEncoder, Encoder, EnumerateEncoder, SequenceEncoder},
    error::Report,
};

pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

pub struct JsonError {}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Report for JsonError {
    fn custom<T>(error: T) -> Self
    where
        T: 'static + Send + Sync + Display + Debug,
    {
        todo!()
    }

    fn message<T>(message: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

pub struct JsonEncoder {}

pub struct JsonValueEncoder<'a> {
    config: &'a JsonEncoder,
}

pub struct JsonArrayEncoder<'a> {
    config: &'a JsonEncoder,
}

pub struct JsonObjectEncoder<'a> {
    config: &'a JsonEncoder,
}

pub struct JsonVariantEncoder<'a> {
    config: &'a JsonEncoder,
}

impl<'a> Encoder for JsonValueEncoder<'a> {
    type Value = Json;
    type Error = JsonError;
    type Some = JsonValueEncoder<'a>;
    type Pack = JsonArrayEncoder<'a>;
    type List = JsonArrayEncoder<'a>;
    type Dict = JsonObjectEncoder<'a>;
    type Tuple = JsonArrayEncoder<'a>;
    type Class = JsonObjectEncoder<'a>;
    type Union = JsonObjectEncoder<'a>;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> SequenceEncoder for JsonArrayEncoder<'a> {
    type Ok = Json;
    type Error = JsonError;
    type Encoder<'this> = JsonValueEncoder<'a>;

    fn next(&mut self) -> Result<Self::Encoder<'_>, Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> DictionaryEncoder for JsonObjectEncoder<'a> {
    type Ok = Json;
    type Error = JsonError;
    type Encoder<'this>
    where
        Self: 'this,
    = ();

    fn next(&mut self) -> Result<Self::Encoder<'_>, Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> EnumerateEncoder for JsonObjectEncoder<'a> {
    type Ok = Json;
    type Error = JsonError;
    type Tag<'this>
    where
        Self: 'this,
    = ();
    type Variant<'this>
    where
        Self: 'this,
    = ();

    fn tag(&mut self) -> Result<Self::Tag<'_>, Self::Error> {
        todo!()
    }

    fn variant(&mut self) -> Result<Self::Variant<'_>, Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
