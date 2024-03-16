use ahash::{HashMap, HashMapExt};
use serde::{ser::Impossible, Serialize, Serializer};
mod macros;
mod ser_map;
pub(crate) mod ser_map_key;
mod ser_struct;
use thiserror::Error;
#[cfg(test)]
mod tests;
use self::{
    macros::{not_possible, simple_not_possible},
    ser_map::SerializeToEnvMap,
    ser_struct::SerializeToEnvStruct,
};

pub(crate) fn serialize_to_map<T: Serialize>(
    prefix: impl Into<String>,
    serialize: &T,
) -> Result<HashMap<String, String>, SerializeToEnvError> {
    let prefix = prefix.into();
    let mut result = HashMap::new();
    let mut ser = SerializeToEnv {
        prefix,
        result: &mut result,
    };
    serialize.serialize(&mut ser)?;
    Ok(result)
}

#[derive(Debug, Error)]
pub enum SerializeToEnvError {
    #[error("Failed to serialize to environment variable {0}")]
    Custom(String),
    #[error("Missing Key")]
    MissingKey,
    #[error("Double Key")]
    DoubleKey,
}
impl From<&str> for SerializeToEnvError {
    fn from(s: &str) -> Self {
        SerializeToEnvError::Custom(s.to_owned())
    }
}
impl From<String> for SerializeToEnvError {
    fn from(s: String) -> Self {
        SerializeToEnvError::Custom(s)
    }
}

impl serde::ser::Error for SerializeToEnvError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SerializeToEnvError::Custom(msg.to_string())
    }
}
pub struct SerializeToEnv<'ser> {
    pub prefix: String,
    pub result: &'ser mut HashMap<String, String>,
}

impl<'ser> Serializer for &'ser mut SerializeToEnv<'ser> {
    type Ok = ();

    type Error = SerializeToEnvError;

    type SerializeSeq = Impossible<(), SerializeToEnvError>;

    type SerializeTuple = Impossible<(), SerializeToEnvError>;

    type SerializeTupleStruct = Impossible<(), SerializeToEnvError>;

    type SerializeTupleVariant = Impossible<(), SerializeToEnvError>;

    type SerializeMap = SerializeToEnvMap<'ser>;

    type SerializeStruct = SerializeToEnvStruct<'ser>;

    type SerializeStructVariant = Impossible<(), SerializeToEnvError>;

    simple_not_possible!(
        fn serialize_bool(self, bool),
        fn serialize_i8(self, i8),
        fn serialize_i16(self, i16),
        fn serialize_i32(self, i32),
        fn serialize_i64(self,  i64),
        fn serialize_u8(self, u8),
        fn serialize_u16(self,  u16),
        fn serialize_u32(self,  u32),
        fn serialize_u64(self,  u64),
        fn serialize_f32(self,  f32),
        fn serialize_f64(self,  f64),
        fn serialize_char(self,  char),
        fn serialize_str(self,  &str),
        fn serialize_bytes(self,  &[u8])
    );

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(SerializeToEnvError::Custom(
            "Cannot serialize Option<T> to environment variable".to_owned(),
        ))
    }
    not_possible!(basic);

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeToEnvMap {
            prefix: self.prefix.clone(),
            key: None,
            result: &mut self.result,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeToEnvStruct {
            prefix: self.prefix.clone(),
            key: None,
            result: &mut self.result,
        })
    }

    not_possible! {
        function: fn serialize_none(self,) -> Result<Self::Ok, Self::Error>,
        error: "Cannot serialize Tuple Variant to environment variable"
    }
}
