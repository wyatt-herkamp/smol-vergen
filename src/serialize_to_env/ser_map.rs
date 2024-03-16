use super::{
    macros::{not_possible, simple_not_possible, simple_serialize_field},
    ser_map_key::KeyToStringSerializer,
    ser_struct::SerializeToEnvStruct,
    SerializeToEnvError,
};
use ahash::HashMap;
use serde::{
    ser::{Impossible, SerializeMap},
    Serialize, Serializer,
};
pub struct SerializeToEnvMap<'ser> {
    pub prefix: String,
    pub key: Option<String>,
    pub result: &'ser mut HashMap<String, String>,
}
impl SerializeMap for SerializeToEnvMap<'_> {
    type Ok = ();

    type Error = SerializeToEnvError;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let must_be_string = KeyToStringSerializer {
            prefix: &self.prefix,
            key: &mut self.key,
        };
        key.serialize(must_be_string)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.key.is_none() {
            return Err(SerializeToEnvError::MissingKey);
        }
        let key = self.key.as_ref().unwrap();
        let mut ser = SerializeToEnvStruct {
            prefix: key.clone(),
            key: None,
            result: self.result,
        };
        value.serialize(&mut ser)
    }
}
impl<'ser> Serializer for &'ser mut SerializeToEnvMap<'_> {
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
        function: fn serialize_map(self, _size: Option<usize>) -> Result<Self::SerializeMap, Self::Error>,
        error: "Complex Types in Complex Types are not supported"
    }
    not_possible! {
        function:  fn serialize_struct(self, _name: &'static str,_len: usize) -> Result<Self::SerializeStruct, Self::Error>,
        error: "Complex Types in Complex Types are not supported"
    }
}
