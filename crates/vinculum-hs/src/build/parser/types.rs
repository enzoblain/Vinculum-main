use super::errors::ParseError;
use std::convert::TryFrom;

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
    String,
    Generic {
        name: String,
        rust_generic_name: String,
    },
    Tuple(Vec<HaskellType>),
}

impl TryFrom<&str> for HaskellType {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim();

        if let Some(inner) = parse_tuple_inner(s) {
            let items = split_tuple_types(inner)
                .into_iter()
                .map(HaskellType::try_from)
                .collect::<Result<Vec<_>, _>>()?;

            return Ok(Self::Tuple(items));
        }

        if s.chars().next().is_some_and(|c| c.is_lowercase()) {
            return Ok(Self::Generic {
                name: s.to_string(),
                rust_generic_name: String::new(),
            });
        }

        match s {
            "Int" => Ok(Self::I64),
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
            "String" => Ok(Self::String),
            _ => Err(ParseError::UnsupportedHaskellType(s.to_owned())),
        }
    }
}

impl HaskellType {
    pub(crate) fn rust_name(&self) -> String {
        match self {
            HaskellType::I8 => "i8".to_string(),
            HaskellType::I16 => "i16".to_string(),
            HaskellType::I32 => "i32".to_string(),
            HaskellType::I64 => "i64".to_string(),
            HaskellType::U8 => "u8".to_string(),
            HaskellType::U16 => "u16".to_string(),
            HaskellType::U32 => "u32".to_string(),
            HaskellType::U64 => "u64".to_string(),
            HaskellType::F32 => "f32".to_string(),
            HaskellType::F64 => "f64".to_string(),
            HaskellType::Bool => "bool".to_string(),
            HaskellType::Char => "char".to_string(),
            HaskellType::String => "String".to_string(),
            HaskellType::Generic {
                rust_generic_name, ..
            } => rust_generic_name.to_string(),
            HaskellType::Tuple(types) => {
                let inner = types
                    .iter()
                    .map(HaskellType::rust_name)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", inner)
            }
        }
    }

    pub(crate) fn to_rust_value_with_param(&self, name: &str, type_param: &str) -> String {
        match self {
            HaskellType::Tuple(types) => {
                let values = types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| ty.to_rust_value_with_param(&format!("{name}.{i}"), type_param))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("Value::<{}>::Tuple(vec![{}])", type_param, values)
            }
            HaskellType::Generic { .. } => format!("Value::<{}>::Generic({})", type_param, name),
            HaskellType::String => format!("Value::<{}>::String({})", type_param, name),
            _ => format!(
                "Value::<{}>::{}({})",
                type_param,
                self.value_variant_name(),
                name
            ),
        }
    }

    pub(crate) fn haskell_pattern(&self, name: &str) -> String {
        match self {
            HaskellType::Generic { .. } => name.to_string(),
            HaskellType::Tuple(types) => {
                let items = types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| ty.haskell_pattern(&format!("{name}_{i}")))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("VTuple [{}]", items)
            }
            HaskellType::String => format!("VString {}", name),
            _ => format!("V{} {}", self.value_variant_name(), name),
        }
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

            HaskellType::Tuple(types) => {
                let items = types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| ty.to_haskell_value(&format!("{name}_{i}")))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", items)
            }

            HaskellType::String => name.to_string(),
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

            HaskellType::Tuple(_) => name.to_string(),

            HaskellType::String => name.to_string(),
            _ => name.to_string(),
        }
    }

    pub(crate) fn is_generic(&self) -> bool {
        match self {
            HaskellType::Generic { .. } => true,
            HaskellType::Tuple(types) => types.iter().any(HaskellType::is_generic),
            _ => false,
        }
    }

    pub(crate) fn resolve_generics<F>(&mut self, resolve: &mut F)
    where
        F: FnMut(&str) -> String,
    {
        match self {
            HaskellType::Generic {
                name,
                rust_generic_name,
            } => {
                *rust_generic_name = resolve(name);
            }
            HaskellType::Tuple(types) => {
                for ty in types {
                    ty.resolve_generics(resolve);
                }
            }
            _ => {}
        }
    }

    pub(crate) fn rust_return_conversion(&self) -> &'static str {
        if self.is_generic() {
            "into_generic()"
        } else {
            "try_into()"
        }
    }

    pub(crate) fn haskell_encoder(&self) -> String {
        match self {
            HaskellType::Generic { .. } | HaskellType::Tuple(_) => "encodeValue".to_string(),
            _ => format!("encode{}", self.value_variant_name()),
        }
    }

    fn value_variant_name(&self) -> &'static str {
        match self {
            HaskellType::I8 => "Int8",
            HaskellType::I16 => "Int16",
            HaskellType::I32 => "Int32",
            HaskellType::I64 => "Int64",
            HaskellType::U8 => "Word8",
            HaskellType::U16 => "Word16",
            HaskellType::U32 => "Word32",
            HaskellType::U64 => "Word64",
            HaskellType::F32 => "Float32",
            HaskellType::F64 => "Float64",
            HaskellType::Bool => "Bool",
            HaskellType::Char => "Char",
            HaskellType::String => "String",
            HaskellType::Generic { .. } => "Generic",
            HaskellType::Tuple(_) => "Tuple",
        }
    }
}

fn parse_tuple_inner(s: &str) -> Option<&str> {
    if !s.starts_with('(') || !s.ends_with(')') {
        return None;
    }

    let inner = &s[1..s.len() - 1];
    if inner.is_empty() {
        return None;
    }

    let mut depth = 0usize;
    let mut has_top_level_comma = false;

    for c in inner.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
            }
            ',' if depth == 0 => {
                has_top_level_comma = true;
            }
            _ => {}
        }
    }

    if depth != 0 || !has_top_level_comma {
        return None;
    }

    Some(inner)
}

fn split_tuple_types(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0usize;

    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }

    parts.push(s[start..].trim());
    parts
}
