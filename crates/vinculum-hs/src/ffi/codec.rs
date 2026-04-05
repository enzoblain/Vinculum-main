use std::any::Any;
use std::convert::TryFrom;

use super::errors::FfiError;
use super::value::{AcceptedTypes, Value};

impl<T: 'static> Value<T> {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        match self {
            Value::Int8(x) => {
                buf.push(0);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int16(x) => {
                buf.push(1);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int32(x) => {
                buf.push(2);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Int64(x) => {
                buf.push(3);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word8(x) => {
                buf.push(4);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word16(x) => {
                buf.push(5);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word32(x) => {
                buf.push(6);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Word64(x) => {
                buf.push(7);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float32(x) => {
                buf.push(8);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float64(x) => {
                buf.push(9);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Bool(b) => {
                buf.push(10);
                buf.push(*b as u8);
            }
            Value::Char(c) => {
                buf.push(11);
                let code = *c as u32;
                buf.extend_from_slice(&code.to_le_bytes());
            }
            Value::String(s) => {
                buf.push(12);
                let payload = s.as_bytes();
                buf.extend_from_slice(&(payload.len() as u64).to_le_bytes());
                buf.extend_from_slice(payload);
            }
            Value::Tuple(t) => {
                buf.push(13);
                buf.push(t.len() as u8);

                for v in t {
                    buf.extend_from_slice(&v.to_bytes());
                }
            }
            Value::Generic(g) => {
                let g = g as &dyn Any;

                if let Some(x) = g.downcast_ref::<i8>() {
                    buf = Value::<()>::Int8(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<i16>() {
                    buf = Value::<()>::Int16(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<i32>() {
                    buf = Value::<()>::Int32(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<i64>() {
                    buf = Value::<()>::Int64(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<bool>() {
                    buf = Value::<()>::Bool(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<char>() {
                    buf = Value::<()>::Char(*x).to_bytes();
                } else if let Some(x) = g.downcast_ref::<String>() {
                    buf = Value::<()>::String(x.clone()).to_bytes();
                }
            }
        }

        buf
    }

    #[allow(dead_code)]
    pub(crate) fn encode_slice(args: &[Value<T>]) -> Vec<u8> {
        let mut buf = Vec::new();

        for arg in args {
            buf.extend_from_slice(&arg.to_bytes());
        }

        buf
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Value<T> {
        Self::from_bytes_checked(bytes)
            .expect("internal FFI decode error: invalid bytes returned by Haskell")
    }

    pub(crate) fn from_bytes_checked(bytes: &[u8]) -> Result<Value<T>, FfiError> {
        Self::decode_one(bytes).map(|(value, _)| value)
    }

    fn decode_one(bytes: &[u8]) -> Result<(Value<T>, usize), FfiError> {
        if bytes.is_empty() {
            return Err(FfiError::UnexpectedEof);
        }

        let tag = bytes[0];

        match tag {
            0 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 1];
                arr.copy_from_slice(&bytes[1..2]);
                Ok((Value::Int8(i8::from_le_bytes(arr)), 2))
            }
            1 => {
                if bytes.len() < 3 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Ok((Value::Int16(i16::from_le_bytes(arr)), 3))
            }
            2 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok((Value::Int32(i32::from_le_bytes(arr)), 5))
            }
            3 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok((Value::Int64(i64::from_le_bytes(arr)), 9))
            }
            4 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 1];
                arr.copy_from_slice(&bytes[1..2]);
                Ok((Value::Word8(u8::from_le_bytes(arr)), 2))
            }
            5 => {
                if bytes.len() < 3 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Ok((Value::Word16(u16::from_le_bytes(arr)), 3))
            }
            6 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok((Value::Word32(u32::from_le_bytes(arr)), 5))
            }
            7 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok((Value::Word64(u64::from_le_bytes(arr)), 9))
            }
            8 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok((Value::Float32(f32::from_le_bytes(arr)), 5))
            }
            9 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok((Value::Float64(f64::from_le_bytes(arr)), 9))
            }
            10 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }
                Ok((Value::Bool(bytes[1] != 0), 2))
            }
            11 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                let code = u32::from_le_bytes(arr);
                char::from_u32(code)
                    .map(Value::Char)
                    .map(|value| (value, 5))
                    .ok_or(FfiError::InvalidChar(code))
            }
            12 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }

                let mut len_buf = [0u8; 8];
                len_buf.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_buf) as usize;

                if bytes.len() < 9 + len {
                    return Err(FfiError::UnexpectedEof);
                }

                let payload = &bytes[9..9 + len];
                let string =
                    String::from_utf8(payload.to_vec()).map_err(|_| FfiError::DecodeError)?;
                Ok((Value::String(string), 9 + len))
            }
            13 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }

                let tuple_len = bytes[1] as usize;
                let mut values = Vec::with_capacity(tuple_len);
                let mut offset = 2;

                for _ in 0..tuple_len {
                    let (value, consumed) = Self::decode_one(&bytes[offset..])?;

                    values.push(value);

                    offset += consumed;
                }

                Ok((Value::Tuple(values), offset))
            }
            _ => Err(FfiError::InvalidTag(tag)),
        }
    }
}

macro_rules! impl_try_from_value {
    ($target:ty, $variant:ident) => {
        impl<T> TryFrom<Value<T>> for $target {
            type Error = FfiError;

            fn try_from(value: Value<T>) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(x) => Ok(x),
                    _ => Err(FfiError::DecodeError),
                }
            }
        }
    };
}

impl_try_from_value!(i8, Int8);
impl_try_from_value!(i16, Int16);
impl_try_from_value!(i32, Int32);
impl_try_from_value!(i64, Int64);
impl_try_from_value!(u8, Word8);
impl_try_from_value!(u16, Word16);
impl_try_from_value!(u32, Word32);
impl_try_from_value!(u64, Word64);
impl_try_from_value!(f32, Float32);
impl_try_from_value!(f64, Float64);
impl_try_from_value!(bool, Bool);
impl_try_from_value!(char, Char);
impl_try_from_value!(String, String);

impl<T: 'static + AcceptedTypes> Value<T> {
    pub fn into_generic(self) -> Result<T, FfiError> {
        let any_val: Box<dyn Any> = match self {
            Value::Generic(x) => return Ok(x),
            Value::Int8(x) => Box::new(x),
            Value::Int16(x) => Box::new(x),
            Value::Int32(x) => Box::new(x),
            Value::Int64(x) => Box::new(x),
            Value::Word8(x) => Box::new(x),
            Value::Word16(x) => Box::new(x),
            Value::Word32(x) => Box::new(x),
            Value::Word64(x) => Box::new(x),
            Value::Float32(x) => Box::new(x),
            Value::Float64(x) => Box::new(x),
            Value::Bool(x) => Box::new(x),
            Value::Char(x) => Box::new(x),
            Value::String(x) => Box::new(x),
            Value::Tuple(_) => return Err(FfiError::DecodeError),
        };

        any_val
            .downcast::<T>()
            .map(|x| *x)
            .map_err(|_| FfiError::DecodeError)
    }
}
