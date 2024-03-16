use ahash::HashMap;
use heck::ToShoutySnakeCase;
use serde::{
    ser::{Impossible, SerializeStruct},
    Serializer,
};

use super::{
    macros::{not_possible, simple_not_possible, simple_serialize_field},
    ser_map::SerializeToEnvMap,
    SerializeToEnvError,
};

pub struct SerializeToEnvStruct<'ser> {
    pub prefix: String,
    pub key: Option<String>,
    pub result: &'ser mut HashMap<String, String>,
}
impl SerializeStruct for SerializeToEnvStruct<'_> {
    type Ok = ();

    type Error = SerializeToEnvError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let key = format!("{}_{}", self.prefix, key.to_shouty_snake_case());
        if self.key.is_some() {
            return Err(SerializeToEnvError::DoubleKey);
        }
        self.key = Some(key);
        value.serialize(self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'ser> Serializer for &'ser mut SerializeToEnvStruct<'_> {
    type Ok = ();

    type Error = SerializeToEnvError;

    type SerializeSeq = Impossible<(), SerializeToEnvError>;

    type SerializeTuple = Impossible<(), SerializeToEnvError>;

    type SerializeTupleStruct = Impossible<(), SerializeToEnvError>;

    type SerializeTupleVariant = Impossible<(), SerializeToEnvError>;

    type SerializeMap = SerializeToEnvMap<'ser>;

    type SerializeStruct = SerializeToEnvStruct<'ser>;

    type SerializeStructVariant = Impossible<(), SerializeToEnvError>;
    simple_serialize_field!(basic);

    simple_not_possible!(fn serialize_bytes(self, &[u8]));
    not_possible!(basic);
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.key = None;

        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    not_possible! {
        function: fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>,
        error: "Complex Types in Complex Types are not supported"
    }
    not_possible! {
        function:  fn serialize_struct(self, _name: &'static str,_len: usize) -> Result<Self::SerializeStruct, Self::Error>,
        error: "Complex Types in Complex Types are not supported"
    }
}
