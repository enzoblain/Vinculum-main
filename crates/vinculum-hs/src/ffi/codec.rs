use crate::ffi::errors::FfiError;
use crate::ffi::value::Value;

impl Value {
    #[allow(dead_code)]
    pub(crate) fn try_into_int8(self) -> Result<i8, FfiError> {
        match self {
            Value::Int8(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_int8(self) -> i8 {
        self.try_into_int8()
            .expect("internal FFI type error: expected Value::Int8")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_int16(self) -> Result<i16, FfiError> {
        match self {
            Value::Int16(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_int16(self) -> i16 {
        self.try_into_int16()
            .expect("internal FFI type error: expected Value::Int16")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_int32(self) -> Result<i32, FfiError> {
        match self {
            Value::Int32(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_int32(self) -> i32 {
        self.try_into_int32()
            .expect("internal FFI type error: expected Value::Int32")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_int64(self) -> Result<i64, FfiError> {
        match self {
            Value::Int64(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_int64(self) -> i64 {
        self.try_into_int64()
            .expect("internal FFI type error: expected Value::Int64")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_word8(self) -> Result<u8, FfiError> {
        match self {
            Value::Word8(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_word8(self) -> u8 {
        self.try_into_word8()
            .expect("internal FFI type error: expected Value::Word8")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_word16(self) -> Result<u16, FfiError> {
        match self {
            Value::Word16(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_word16(self) -> u16 {
        self.try_into_word16()
            .expect("internal FFI type error: expected Value::Word16")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_word32(self) -> Result<u32, FfiError> {
        match self {
            Value::Word32(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_word32(self) -> u32 {
        self.try_into_word32()
            .expect("internal FFI type error: expected Value::Word32")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_word64(self) -> Result<u64, FfiError> {
        match self {
            Value::Word64(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_word64(self) -> u64 {
        self.try_into_word64()
            .expect("internal FFI type error: expected Value::Word64")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_float32(self) -> Result<f32, FfiError> {
        match self {
            Value::Float32(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_float32(self) -> f32 {
        self.try_into_float32()
            .expect("internal FFI type error: expected Value::Float32")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_float64(self) -> Result<f64, FfiError> {
        match self {
            Value::Float64(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_float64(self) -> f64 {
        self.try_into_float64()
            .expect("internal FFI type error: expected Value::Float64")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_bool(self) -> Result<bool, FfiError> {
        match self {
            Value::Bool(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_bool(self) -> bool {
        self.try_into_bool()
            .expect("internal FFI type error: expected Value::Bool")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_char(self) -> Result<char, FfiError> {
        match self {
            Value::Char(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_char(self) -> char {
        self.try_into_char()
            .expect("internal FFI type error: expected Value::Char")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_string(self) -> Result<String, FfiError> {
        match self {
            Value::String(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_string(self) -> String {
        self.try_into_string()
            .expect("internal FFI type error: expected Value::String")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_bytes(self) -> Result<Vec<u8>, FfiError> {
        match self {
            Value::Bytes(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.try_into_bytes()
            .expect("internal FFI type error: expected Value::Bytes")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_option(self) -> Result<Option<Box<Value>>, FfiError> {
        match self {
            Value::Option(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_option(self) -> Option<Box<Value>> {
        self.try_into_option()
            .expect("internal FFI type error: expected Value::Option")
    }

    #[allow(dead_code)]
    pub(crate) fn try_into_vec(self) -> Result<Vec<Value>, FfiError> {
        match self {
            Value::Vec(value) => Ok(value),
            _ => Err(FfiError::DecodeError),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn into_vec(self) -> Vec<Value> {
        self.try_into_vec()
            .expect("internal FFI type error: expected Value::Vec")
    }

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
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Value::Bytes(b) => {
                buf.push(13);
                buf.extend_from_slice(&(b.len() as u64).to_le_bytes());
                buf.extend_from_slice(b);
            }
            Value::Option(opt) => {
                buf.push(14);
                match opt {
                    Some(inner) => {
                        buf.push(1);
                        buf.extend_from_slice(&inner.to_bytes());
                    }
                    None => {
                        buf.push(0);
                    }
                }
            }
            Value::Vec(values) => {
                buf.push(15);
                buf.extend_from_slice(&(values.len() as u64).to_le_bytes());
                for value in values {
                    buf.extend_from_slice(&value.to_bytes());
                }
            }
        }

        buf
    }

    #[allow(dead_code)]
    pub(crate) fn encode_slice(args: &[Value]) -> Vec<u8> {
        let mut buf = Vec::new();

        for arg in args {
            buf.extend_from_slice(&arg.to_bytes());
        }

        buf
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Value {
        Self::from_bytes_checked(bytes)
            .expect("internal FFI decode error: invalid bytes returned by Haskell")
    }

    pub(crate) fn from_bytes_checked(bytes: &[u8]) -> Result<Value, FfiError> {
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
                Ok(Value::Int8(i8::from_le_bytes(arr)))
            }
            1 => {
                if bytes.len() < 3 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Ok(Value::Int16(i16::from_le_bytes(arr)))
            }
            2 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok(Value::Int32(i32::from_le_bytes(arr)))
            }
            3 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok(Value::Int64(i64::from_le_bytes(arr)))
            }
            4 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 1];
                arr.copy_from_slice(&bytes[1..2]);
                Ok(Value::Word8(u8::from_le_bytes(arr)))
            }
            5 => {
                if bytes.len() < 3 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 2];
                arr.copy_from_slice(&bytes[1..3]);
                Ok(Value::Word16(u16::from_le_bytes(arr)))
            }
            6 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok(Value::Word32(u32::from_le_bytes(arr)))
            }
            7 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok(Value::Word64(u64::from_le_bytes(arr)))
            }
            8 => {
                if bytes.len() < 5 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[1..5]);
                Ok(Value::Float32(f32::from_le_bytes(arr)))
            }
            9 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Ok(Value::Float64(f64::from_le_bytes(arr)))
            }
            10 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }
                Ok(Value::Bool(bytes[1] != 0))
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
                    .ok_or(FfiError::InvalidChar(code))
            }
            12 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                if bytes.len() < 9 + len {
                    return Err(FfiError::UnexpectedEof);
                }

                let s = String::from_utf8(bytes[9..9 + len].to_vec())
                    .map_err(|_| FfiError::InvalidUtf8)?;
                Ok(Value::String(s))
            }
            13 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                if bytes.len() < 9 + len {
                    return Err(FfiError::UnexpectedEof);
                }

                Ok(Value::Bytes(bytes[9..9 + len].to_vec()))
            }
            14 => {
                if bytes.len() < 2 {
                    return Err(FfiError::UnexpectedEof);
                }

                let option_tag = bytes[1];
                match option_tag {
                    0 => Ok(Value::Option(None)),
                    1 => {
                        if bytes.len() < 3 {
                            return Err(FfiError::UnexpectedEof);
                        }

                        let inner = Self::from_bytes_checked(&bytes[2..])?;

                        Ok(Value::Option(Some(Box::new(inner))))
                    }
                    _ => Err(FfiError::InvalidTag(option_tag)),
                }
            }
            15 => {
                if bytes.len() < 9 {
                    return Err(FfiError::UnexpectedEof);
                }

                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let vec_len = u64::from_le_bytes(len_bytes) as usize;

                let mut values = Vec::new();
                let mut pos = 9;

                for _ in 0..vec_len {
                    if pos >= bytes.len() {
                        return Err(FfiError::UnexpectedEof);
                    }
                    let value = Self::from_bytes_checked(&bytes[pos..])?;
                    let encoded = value.to_bytes();
                    pos += encoded.len();
                    values.push(value);
                }

                Ok(Value::Vec(values))
            }
            _ => Err(FfiError::InvalidTag(tag)),
        }
    }
}
