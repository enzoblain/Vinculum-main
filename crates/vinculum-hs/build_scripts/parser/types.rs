#[derive(Clone)]
pub(crate) struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub r#return: Type,
}

#[derive(Clone)]
pub(crate) struct Arg {
    pub name: String,
    pub r#type: Type,
}

#[derive(Clone)]
pub(crate) enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Word8,
    Word16,
    Word32,
    Word64,
    Float32,
    Float64,
    Bool,
    Char,
    String,
    Bytes,
    Maybe(Box<Type>),
    Vec(Box<Type>),
}

impl Type {
    pub(crate) fn rust_type(&self) -> String {
        match self {
            Type::Int8 => "i8".to_string(),
            Type::Int16 => "i16".to_string(),
            Type::Int32 => "i32".to_string(),
            Type::Int64 => "i64".to_string(),
            Type::Word8 => "u8".to_string(),
            Type::Word16 => "u16".to_string(),
            Type::Word32 => "u32".to_string(),
            Type::Word64 => "u64".to_string(),
            Type::Float32 => "f32".to_string(),
            Type::Float64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "String".to_string(),
            Type::Bytes => "Vec<u8>".to_string(),
            Type::Maybe(inner) => format!("Option<{}>", inner.rust_type()),
            Type::Vec(inner) => format!("Vec<{}>", inner.rust_type()),
        }
    }

    pub(crate) fn return_converter(&self) -> String {
        match self {
            Type::Int8 => "into_int8".to_string(),
            Type::Int16 => "into_int16".to_string(),
            Type::Int32 => "into_int32".to_string(),
            Type::Int64 => "into_int64".to_string(),
            Type::Word8 => "into_word8".to_string(),
            Type::Word16 => "into_word16".to_string(),
            Type::Word32 => "into_word32".to_string(),
            Type::Word64 => "into_word64".to_string(),
            Type::Float32 => "into_float32".to_string(),
            Type::Float64 => "into_float64".to_string(),
            Type::Bool => "into_bool".to_string(),
            Type::Char => "into_char".to_string(),
            Type::String => "into_string".to_string(),
            Type::Bytes => "into_bytes".to_string(),
            Type::Maybe(_) => "into_option".to_string(),
            Type::Vec(_) => "into_vec".to_string(),
        }
    }

    pub(crate) fn rust_value_ctor(&self, name: &str) -> String {
        match self {
            Type::Int8 => format!("Value::Int8({})", name),
            Type::Int16 => format!("Value::Int16({})", name),
            Type::Int32 => format!("Value::Int32({})", name),
            Type::Int64 => format!("Value::Int64({})", name),
            Type::Word8 => format!("Value::Word8({})", name),
            Type::Word16 => format!("Value::Word16({})", name),
            Type::Word32 => format!("Value::Word32({})", name),
            Type::Word64 => format!("Value::Word64({})", name),
            Type::Float32 => format!("Value::Float32({})", name),
            Type::Float64 => format!("Value::Float64({})", name),
            Type::Bool => format!("Value::Bool({})", name),
            Type::Char => format!("Value::Char({})", name),
            Type::String => format!("Value::String({})", name),
            Type::Bytes => format!("Value::Bytes({})", name),
            Type::Maybe(inner) => {
                let inner_ctor = inner.rust_value_ctor("val");

                format!(
                    "Value::Option({}.map(|val| Box::new({})))",
                    name, inner_ctor
                )
            }
            Type::Vec(inner) => {
                let inner_ctor = inner.rust_value_ctor("v");
                format!("Value::Vec({}.into_iter().map(|v| {}).collect())", name, inner_ctor)
            }
        }
    }

    pub(crate) fn rust_return_converter(&self, name: &str) -> String {
        match self {
            Type::Int8 => format!("{}.into_int8()", name),
            Type::Int16 => format!("{}.into_int16()", name),
            Type::Int32 => format!("{}.into_int32()", name),
            Type::Int64 => format!("{}.into_int64()", name),
            Type::Word8 => format!("{}.into_word8()", name),
            Type::Word16 => format!("{}.into_word16()", name),
            Type::Word32 => format!("{}.into_word32()", name),
            Type::Word64 => format!("{}.into_word64()", name),
            Type::Float32 => format!("{}.into_float32()", name),
            Type::Float64 => format!("{}.into_float64()", name),
            Type::Bool => format!("{}.into_bool()", name),
            Type::Char => format!("{}.into_char()", name),
            Type::String => format!("{}.into_string()", name),
            Type::Bytes => format!("{}.into_bytes()", name),
            Type::Maybe(inner) => format!("{}.into_option().and_then(|val| {{ if let Value::{} = *val {{ Some(x) }} else {{ None }} }})", name, inner.rust_type()),
            Type::Vec(inner) => {
                let inner_extractor = inner.rust_vec_element_extractor();
                format!("{}.into_vec().into_iter().map(|v| {}).collect()", name, inner_extractor)
            }
        }
    }

    pub(crate) fn rust_vec_element_extractor(&self) -> String {
        match self {
            Type::Int8 => "if let Value::Int8(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Int16 => "if let Value::Int16(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Int32 => "if let Value::Int32(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Int64 => "if let Value::Int64(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Word8 => "if let Value::Word8(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Word16 => "if let Value::Word16(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Word32 => "if let Value::Word32(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Word64 => "if let Value::Word64(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Float32 => "if let Value::Float32(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Float64 => "if let Value::Float64(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Bool => "if let Value::Bool(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Char => "if let Value::Char(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::String => "if let Value::String(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Bytes => "if let Value::Bytes(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Maybe(_) => "if let Value::Option(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
            Type::Vec(_) => "if let Value::Vec(x) = v { x } else { panic!(\"Type mismatch\") }".to_string(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_value_function(&self) -> String {
        match self {
            Type::Int8 => "fromValueInt8".to_string(),
            Type::Int16 => "fromValueInt16".to_string(),
            Type::Int32 => "fromValueInt32".to_string(),
            Type::Int64 => "fromValueInt64".to_string(),
            Type::Word8 => "fromValueWord8".to_string(),
            Type::Word16 => "fromValueWord16".to_string(),
            Type::Word32 => "fromValueWord32".to_string(),
            Type::Word64 => "fromValueWord64".to_string(),
            Type::Float32 => "fromValueFloat32".to_string(),
            Type::Float64 => "fromValueFloat64".to_string(),
            Type::Bool => "fromValueBool".to_string(),
            Type::Char => "fromValueChar".to_string(),
            Type::String => "fromValueString".to_string(),
            Type::Bytes => "fromValueBytes".to_string(),
            Type::Maybe(_) => unreachable!("Maybe should not use from_value_function"),
            Type::Vec(_) => unreachable!("Vec should not use from_value_function"),
        }
    }

    pub(crate) fn haskell_arg_expr(&self, name: &str) -> String {
        match self {
            Type::Int8 => format!("(fromIntegral {})", name),
            Type::Int16 => format!("(fromIntegral {})", name),
            Type::Int32 => format!("(fromIntegral {})", name),
            Type::Int64 => format!("(fromIntegral {})", name),
            Type::Word8 => format!("(fromIntegral {})", name),
            Type::Word16 => format!("(fromIntegral {})", name),
            Type::Word32 => format!("(fromIntegral {})", name),
            Type::Word64 => format!("(fromIntegral {})", name),
            Type::Float32 => name.to_string(),
            Type::Float64 => name.to_string(),
            Type::Bool => name.to_string(),
            Type::Char => name.to_string(),
            Type::String => name.to_string(),
            Type::Bytes => name.to_string(),
            Type::Maybe(inner) => format!(
                "(decodeOptionWith {} {})",
                inner.from_value_function(),
                name
            ),
            Type::Vec(inner) => format!(
                "(map {} {})",
                inner.from_value_function(),
                name
            ),
        }
    }

    pub(crate) fn haskell_return_expr(&self, expr: &str) -> String {
        match self {
            Type::Int8 => format!("(fromIntegral ({}))", expr),
            Type::Int16 => format!("(fromIntegral ({}))", expr),
            Type::Int32 => format!("(fromIntegral ({}))", expr),
            Type::Int64 => format!("(fromIntegral ({}))", expr),
            Type::Word8 => format!("(fromIntegral ({}))", expr),
            Type::Word16 => format!("(fromIntegral ({}))", expr),
            Type::Word32 => format!("(fromIntegral ({}))", expr),
            Type::Word64 => format!("(fromIntegral ({}))", expr),
            Type::Float32 => expr.to_string(),
            Type::Float64 => expr.to_string(),
            Type::Bool => expr.to_string(),
            Type::Char => expr.to_string(),
            Type::String => expr.to_string(),
            Type::Bytes => expr.to_string(),
            Type::Maybe(_) => expr.to_string(),
            Type::Vec(inner) => format!("map {} ({})", inner.haskell_value_constructor(), expr),
        }
    }
}
