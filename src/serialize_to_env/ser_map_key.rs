use heck::ToShoutySnekCase;
use serde::{ser::Impossible, Serializer};

use super::SerializeToEnvError;
const MUST_BE_STRING: &str = "Key must be able to serialize to string";

pub struct KeyToStringSerializer<'ser> {
    pub prefix: &'ser str,
    pub key: &'ser mut Option<String>,
}
macro_rules! not_possible_as_key {
    (fn $name:ident(self, $($var_name:ident: $var_type:ty),*) -> Result<$ok:path, Self::Error>) => {
        super::macros::not_possible! {
            function: fn $name(self, $($var_name: $var_type),*) -> Result<$ok, Self::Error>,
            error: MUST_BE_STRING
        }
    };
    ($(fn $name:ident(self, $var_name:ident: $var_type:ty)),*) => {
        $(
            super::macros::not_possible! {
                function: fn $name(self, $var_name: $var_type) -> Result<Self::Ok, Self::Error>,
                error: MUST_BE_STRING
            }
        )*
    };
    (fn $name:ident<T: ?Sized>(self, $($var_name:ident: $var_type:ty),*) -> Result<$ok:path, Self::Error>) => {
        super::macros::not_possible! {
            function: fn $name<T: ?Sized>(self, $($var_name: $var_type),*) -> Result<$ok, Self::Error>,
            error: MUST_BE_STRING
        }
    };
}
impl Serializer for KeyToStringSerializer<'_> {
    type Ok = ();

    type Error = SerializeToEnvError;

    type SerializeSeq = Impossible<(), SerializeToEnvError>;

    type SerializeTuple = Impossible<(), SerializeToEnvError>;

    type SerializeTupleStruct = Impossible<(), SerializeToEnvError>;

    type SerializeTupleVariant = Impossible<(), SerializeToEnvError>;

    type SerializeMap = Impossible<(), SerializeToEnvError>;

    type SerializeStruct = Impossible<(), SerializeToEnvError>;

    type SerializeStructVariant = Impossible<(), SerializeToEnvError>;

    not_possible_as_key!(fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>);
    not_possible_as_key!(fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error>);

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let key = format!("{}_{}", self.prefix, v.TO_SHOUTY_SNEK_CASE());
        if self.key.is_some() {
            return Err(SerializeToEnvError::DoubleKey);
        }
        *self.key = Some(key);
        Ok(())
    }
    not_possible_as_key!(
        fn serialize_bool(self, _v: bool),
        fn serialize_i8(self, _v: i8),
        fn serialize_i16(self, _v: i16),
        fn serialize_i32(self, _v: i32),
        fn serialize_i64(self, _v: i64),
        fn serialize_u8(self, _v: u8),
        fn serialize_u16(self, _v: u16),
        fn serialize_u32(self, _v: u32),
        fn serialize_u64(self, _v: u64),
        fn serialize_f32(self, _v: f32),
        fn serialize_f64(self, _v: f64),
        fn serialize_char(self, _v: char),
        fn serialize_bytes(self, _v: &[u8])
    );
    not_possible_as_key!(fn serialize_none(self,) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_unit(self,) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>);
    not_possible_as_key!(fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>);
    not_possible_as_key!(fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error>);
    not_possible_as_key!(fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error>);
    not_possible_as_key!(fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error>);
    not_possible_as_key!(fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error>);
    not_possible_as_key!(fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>);
}
