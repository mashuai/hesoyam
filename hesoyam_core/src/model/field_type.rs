use std::str::FromStr;

use syn::export::{TokenStream2 as TokenStream, ToTokens};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
    String,

    SmallInteger,
    Integer,
    BigInteger,

    SmallUnsignedInteger,
    UnsignedInteger,
    BigUnsignedInteger,

    Boolean,

    Array(Vec<Self>),
    Enum(Vec<String>),
}

impl FieldType {
    pub fn from_type_string(type_str: &str) -> Self {
        match type_str {
            "String" => FieldType::String,

            "i8" => FieldType::SmallInteger,
            "i32" => FieldType::Integer,
            "i64" => FieldType::BigInteger,

            "u8" => FieldType::SmallUnsignedInteger,
            "u32" => FieldType::UnsignedInteger,
            "u64" => FieldType::BigUnsignedInteger,

            "bool" => FieldType::Boolean,

            unknown_type => unimplemented!("{} is unknown type", unknown_type),
        }
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let str_type = format!("{{ use hesoyam::FieldType as FFieldType; FFieldType::{:#?} }}", self);
        let ts = TokenStream::from_str(str_type.as_str()).unwrap();

        ts.to_tokens(tokens);
    }
}