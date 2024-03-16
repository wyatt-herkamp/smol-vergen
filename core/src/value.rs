use ahash::HashMap;
use chrono::FixedOffset;
use derive_more::From;
use serde::{de::Visitor, Deserialize, Serialize};
/// A value used for the environment variables
#[derive(From, Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Float(f64),
    Number(i64),
    Char(char),
    DateTime(chrono::DateTime<FixedOffset>),
}
macro_rules! from_num {
    (Number => $num:ty) => {
        impl From<$num> for Value {
            fn from(value: $num) -> Self {
                Value::Number(value as i64)
            }
        }
    };
    (Float => $num:ty) => {
        impl From<$num> for Value {
            fn from(value: $num) -> Self {
                Value::Float(value as f64)
            }
        }
    };
    (Numbers => $($num:ty),*) => {
        $(from_num!(Number => $num);)*
    };
    (Floats => $($num:ty),*) => {
        $(from_num!(Float => $num);)*
    };
}
from_num!(Numbers => i8, i16, i32, u8, u16, u32, u64);
from_num!(Floats => f32);

impl Value {
    pub fn add_to_env(&self, key: &str) {
        match self {
            Value::String(value) => crate::add_to_env(key, value),
            Value::Bool(value) => crate::add_to_env(key, &value.to_string()),
            Value::Float(float) => {
                let mut buffer = dtoa::Buffer::new();
                crate::add_to_env(key, buffer.format(*float))
            }
            Value::Number(value) => {
                let mut buffer = itoa::Buffer::new();
                crate::add_to_env(key, buffer.format(*value))
            }
            Value::Char(c) => crate::add_to_env(key, &c.to_string()),
            Value::DateTime(date_time) => crate::add_to_env(key, &date_time.to_rfc3339()),
        }
    }
    pub fn add_to_map(&self, key: &str, map: &mut HashMap<String, String>) {
        match self {
            Value::String(value) => map.insert(key.to_owned(), value.to_owned()),
            Value::Bool(value) => map.insert(key.to_owned(), value.to_string()),
            Value::Float(float) => {
                let mut buffer = dtoa::Buffer::new();
                map.insert(key.to_owned(), buffer.format(*float).to_owned())
            }
            Value::Number(value) => {
                let mut buffer = itoa::Buffer::new();
                map.insert(key.to_owned(), buffer.format(*value).to_owned())
            }
            Value::Char(c) => map.insert(key.to_owned(), c.to_string()),
            Value::DateTime(date_time) => map.insert(key.to_owned(), date_time.to_rfc3339()),
        };
    }
}
impl Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::String(value) => serializer.serialize_str(value),
            Value::Bool(value) => serializer.serialize_bool(*value),
            Value::Float(value) => serializer.serialize_f64(*value),
            Value::Number(value) => serializer.serialize_i64(*value),
            Value::Char(value) => serializer.serialize_char(*value),
            Value::DateTime(date) => serializer.serialize_str(&date.to_rfc3339()),
        }
    }
}
macro_rules! visit {
    (String => fn $name:ident<E>(self, $var_name:ident: $value:ty)) => {
        fn $name<E>(self, $var_name: $value) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::String($var_name.to_owned()))
        }
    };
    ($variant:ident => fn $name:ident<E>(self, $var_name:ident: $value:ty)) => {
        fn $name<E>(self, $var_name: $value) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Value::$variant($var_name))
        }
    };
    ($variant:ident as $as_ty:ty => [
            $(fn $name:ident<E>(self, $var_name:ident: $value:ty)),+ $(,)?
        ]) => {
        $(
            fn $name<E>(self, $var_name: $value) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::$variant($var_name as $as_ty))
            }
        )*
    };
}

struct ValueVisitor;
impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string, a boolean, a float or a number")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_owned())
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Ok(date_time) = chrono::DateTime::parse_from_rfc3339(&v) {
            return Ok(Value::DateTime(date_time));
        } else {
            Ok(Value::String(v))
        }
    }
    visit!(Char => fn visit_char<E>(self, value: char));
    visit!(Bool => fn visit_bool<E>(self, value: bool));
    visit!(Number as i64 =>
    [
        fn visit_i8<E>(self, value: i8),
        fn visit_i16<E>(self, value: i16),
        fn visit_i32<E>(self, value: i32),
        fn visit_i64<E>(self, value: i64),

        fn visit_u8<E>(self, value: u8),
        fn visit_u16<E>(self, value: u16),
        fn visit_u32<E>(self, value: u32),
        fn visit_u64<E>(self, value: u64),
    ]);

    visit!(Float as f64 =>
    [
        fn visit_f32<E>(self, value: f32),
        fn visit_f64<E>(self, value: f64),
    ]);
}
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueVisitor)
    }
}
