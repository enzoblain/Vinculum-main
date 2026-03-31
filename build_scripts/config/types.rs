use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(rename = "function")]
    pub functions: Vec<Function>,
}

#[derive(Deserialize)]
pub(crate) struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub r#return: Type,
}

#[derive(Deserialize)]
pub(crate) struct Arg {
    pub name: String,
    pub r#type: Type,
}

#[derive(Deserialize)]
pub(crate) enum Type {
    Int,
    Float,
    Bool,
    String,
    Bytes,
}

impl Type {
    pub(crate) fn rust_type(&self) -> &'static str {
        match self {
            Type::Int => "i64",
            Type::Float => "f64",
            Type::Bool => "bool",
            Type::String => "String",
            Type::Bytes => "Vec<u8>",
        }
    }

    pub(crate) fn return_converter(&self) -> &'static str {
        match self {
            Type::Int => "Value::into_int",
            Type::Float => "Value::into_float",
            Type::Bool => "Value::into_bool",
            Type::String => "Value::into_string",
            Type::Bytes => "Value::into_bytes",
        }
    }

    pub(crate) fn rust_value_ctor(&self, name: &str) -> String {
        match self {
            Type::Int => format!("Value::Int({})", name),
            Type::Float => format!("Value::Float({})", name),
            Type::Bool => format!("Value::Bool({})", name),
            Type::String => format!("Value::String({}.to_string())", name),
            Type::Bytes => format!("Value::Bytes({})", name),
        }
    }

    pub(crate) fn haskell_arg_expr(&self, name: &str) -> String {
        match self {
            Type::Int => format!("(fromIntegral {})", name),
            Type::Float => name.to_string(),
            Type::Bool => name.to_string(),
            Type::String => name.to_string(),
            Type::Bytes => name.to_string(),
        }
    }

    pub(crate) fn haskell_return_expr(&self, expr: &str) -> String {
        match self {
            Type::Int => format!("(fromIntegral ({}))", expr),
            Type::Float => expr.to_string(),
            Type::Bool => expr.to_string(),
            Type::String => expr.to_string(),
            Type::Bytes => expr.to_string(),
        }
    }
}
