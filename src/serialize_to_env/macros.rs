macro_rules! simple_serialize_field {
    (string => fn $name:ident(self, $var_name:ident: $var_type:ty)) =>{
        fn $name( self, $var_name: $var_type) -> Result<Self::Ok, Self::Error> {
            let key = self.key.take().ok_or(SerializeToEnvError::MissingKey)?;
            self.result.insert(key, $var_name.to_owned());
            Ok(())
        }
    };
    (num => fn $name:ident(self, $var_name:ident: $var_type:ty)) =>{
        fn $name(self, $var_name: $var_type) -> Result<Self::Ok, Self::Error> {
            let key = self.key.take().ok_or(SerializeToEnvError::MissingKey)?;
            let mut buffer = itoa::Buffer::new();
            self.result.insert(key, buffer.format($var_name).to_owned());
            Ok(())
        }
    };
    (float => fn $name:ident(self, $var_name:ident: $var_type:ty)) =>{
        fn $name(self, $var_name: $var_type) -> Result<Self::Ok, Self::Error> {
            let key = self.key.take().ok_or(SerializeToEnvError::MissingKey)?;
            let mut buffer = dtoa::Buffer::new();
            self.result.insert(key, buffer.format($var_name).to_owned());
            Ok(())
        }
    };
    (other => fn $name:ident(self, $var_name:ident: $var_type:ty)) =>{
        fn $name(self, $var_name: $var_type) -> Result<Self::Ok, Self::Error> {
            let key = self.key.take().ok_or(SerializeToEnvError::MissingKey)?;
            self.result.insert(key, $var_name.to_string());
            Ok(())
        }
    };

    ($($serialize_type:ident => fn $name:ident(self, $var_name:ident: $var_type:ty)),*) => {
        $(
            simple_serialize_field!($serialize_type => fn $name(self, $var_name: $var_type));
        )*
    };
    (basic) => {
        simple_serialize_field!(
        other => fn serialize_bool(self, v: bool),
        other => fn serialize_char(self, v: char),
        num => fn serialize_i8(self, v: i8),
        num => fn serialize_i16(self, v: i16),
        num => fn serialize_i32(self, v: i32),
        num => fn serialize_i64(self, v: i64),
        num => fn serialize_u8(self, v: u8),
        num => fn serialize_u16(self, v: u16),
        num => fn serialize_u32(self, v: u32),
        num => fn serialize_u64(self, v: u64),
        float => fn serialize_f32(self, v: f32),
        float =>fn serialize_f64(self, v: f64),
        string => fn serialize_str(self, v: &str)
    );

    }
}
pub(crate) use simple_serialize_field;
macro_rules! simple_not_possible {
    (fn $name:ident(self, $var_type:ty)) => {
        fn $name(self, _value: $var_type) -> Result<Self::Ok, Self::Error> {
            Err(SerializeToEnvError::Custom(format!(
                "Cannot serialize {} to environment variable",
                stringify!($var_type)
            )))
        }
    };
    ($(fn $name:ident(self, $var_type:ty)),*) => {
        $(
            fn $name(self, _value: $var_type) -> Result<Self::Ok, Self::Error> {
                Err(SerializeToEnvError::Custom(format!(
                    "Cannot serialize {} to environment variable",
                    stringify!($var_type)
                )))
            }
        )*
    };
}
pub(crate) use simple_not_possible;
macro_rules! not_possible {
    {
        function: fn $name:ident(self, $($var_name:ident: $var_type:ty),*) -> Result<$ok:path, Self::Error>,
        error: $error:tt
    } => {
        fn $name(self, $($var_name: $var_type),*) -> Result<$ok, Self::Error> {
            Err(SerializeToEnvError::from($error))
        }
    };
    {
        function: fn $name:ident<T: ?Sized>(self, $($var_name:ident: $var_type:ty),*) -> Result<$ok:path, Self::Error>,
        error: $error:tt
    } => {
        fn $name<T: ?Sized>(self, $($var_name: $var_type),*) -> Result<$ok, Self::Error> {
            Err(SerializeToEnvError::from($error))
        }
    };

    (basic) =>{
            not_possible! {
                function: fn serialize_unit(self,) -> Result<Self::Ok, Self::Error>,
                error: "Cannot serialize Unit to environment variable"
            }
    not_possible! {
        function: fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error>,
        error:  "Cannot serialize Unit Struct to environment variable"
    }
    not_possible! {
        function: fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error>,
        error: "Cannot serialize Unit Variant to environment variable"
    }
    not_possible! {
        function: fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>,
        error: "Cannot serialize Newtype Struct to environment variable"
    }
    not_possible! {
        function: fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>,
        error: "Cannot serialize Newtype Variant to environment variable"
    }
    not_possible! {
        function: fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>,
        error: "Cannot serialize Sequence to environment variable"
    }
    not_possible! {
        function: fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error>,
        error: "Cannot serialize Tuple to environment variable"
    }
    not_possible! {
        function: fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error>,
        error: "Cannot serialize Tuple Struct to environment variable"
    }
    not_possible! {
        function: fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error>,
        error: "Cannot serialize Tuple Variant to environment variable"
    }
    not_possible! {
        function: fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error>,
        error: "Cannot serialize Struct Variant to environment variable"
    }
    }
}

pub(crate) use not_possible;
