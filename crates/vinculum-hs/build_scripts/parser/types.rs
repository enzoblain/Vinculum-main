use std::convert::TryFrom;

use super::errors::ParseError;

#[derive(Debug)]
pub(crate) enum HaskellType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
}

impl TryFrom<&str> for HaskellType {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Int8" => Ok(Self::I8),
            "Int16" => Ok(Self::I16),
            "Int32" => Ok(Self::I32),
            "Int64" => Ok(Self::I64),
            "Word8" => Ok(Self::U8),
            "Word16" => Ok(Self::U16),
            "Word32" => Ok(Self::U32),
            "Word64" => Ok(Self::U64),
            "Float" => Ok(Self::F32),
            "Double" => Ok(Self::F64),
            "Bool" => Ok(Self::Bool),
            "Char" => Ok(Self::Char),
            _ => Err(ParseError::UnsupportedHaskellType(s.to_owned())),
        }
    }
}

impl HaskellType {
    pub(crate) fn haskell_name(&self) -> &'static str {
        match self {
            HaskellType::I8 => "Int8",
            HaskellType::I16 => "Int16",
            HaskellType::I32 => "Int32",
            HaskellType::I64 => "Int64",
            HaskellType::U8 => "Word8",
            HaskellType::U16 => "Word16",
            HaskellType::U32 => "Word32",
            HaskellType::U64 => "Word64",
            HaskellType::F32 => "Float",
            HaskellType::F64 => "Double",
            HaskellType::Bool => "Bool",
            HaskellType::Char => "Char",
        }
    }

    pub(crate) fn rust_name(&self) -> &'static str {
        match self {
            HaskellType::I8 => "i8",
            HaskellType::I16 => "i16",
            HaskellType::I32 => "i32",
            HaskellType::I64 => "i64",
            HaskellType::U8 => "u8",
            HaskellType::U16 => "u16",
            HaskellType::U32 => "u32",
            HaskellType::U64 => "u64",
            HaskellType::F32 => "f32",
            HaskellType::F64 => "f64",
            HaskellType::Bool => "bool",
            HaskellType::Char => "char",
        }
    }

    pub(crate) fn to_rust_value(&self, name: &str) -> String {
        format!("Value::{}({})", self.haskell_name(), name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_rust_value(&self) -> String {
        format!("result.into_{}()", self.haskell_name().to_lowercase())
    }

    pub(crate) fn to_haskell_value(&self, name: &str) -> String {
        match self {
            HaskellType::I8
            | HaskellType::I16
            | HaskellType::I32
            | HaskellType::I64
            | HaskellType::U8
            | HaskellType::U16
            | HaskellType::U32
            | HaskellType::U64 => format!("(fromIntegral {})", name),
            _ => name.to_string(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_haskell_value(&self, name: &str) -> String {
        match self {
            HaskellType::I8
            | HaskellType::I16
            | HaskellType::I32
            | HaskellType::I64
            | HaskellType::U8
            | HaskellType::U16
            | HaskellType::U32
            | HaskellType::U64 => format!("(fromIntegral ({}))", name),
            _ => name.to_string(),
        }
    }

    pub(crate) fn haskell_pattern(&self, name: &str) -> String {
        match self {
            HaskellType::I8 => format!("VInt8 {}", name),
            HaskellType::I16 => format!("VInt16 {}", name),
            HaskellType::I32 => format!("VInt32 {}", name),
            HaskellType::I64 => format!("VInt64 {}", name),
            HaskellType::U8 => format!("VWord8 {}", name),
            HaskellType::U16 => format!("VWord16 {}", name),
            HaskellType::U32 => format!("VWord32 {}", name),
            HaskellType::U64 => format!("VWord64 {}", name),
            HaskellType::F32 => format!("VFloat32 {}", name),
            HaskellType::F64 => format!("VFloat64 {}", name),
            HaskellType::Bool => format!("VBool {}", name),
            HaskellType::Char => format!("VChar {}", name),
        }
    }

    pub(crate) fn haskell_encode_fn(&self) -> &'static str {
        match self {
            HaskellType::I8 => "encodeInt8",
            HaskellType::I16 => "encodeInt16",
            HaskellType::I32 => "encodeInt32",
            HaskellType::I64 => "encodeInt64",
            HaskellType::U8 => "encodeWord8",
            HaskellType::U16 => "encodeWord16",
            HaskellType::U32 => "encodeWord32",
            HaskellType::U64 => "encodeWord64",
            HaskellType::F32 => "encodeFloat32",
            HaskellType::F64 => "encodeFloat64",
            HaskellType::Bool => "encodeBool",
            HaskellType::Char => "encodeChar",
        }
    }
}
