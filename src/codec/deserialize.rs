use std::ffi::CString;
use std::mem::size_of;

use super::accepted::ToValue;
use super::{AcceptedTypes, BUFFER_SIZE, Value};

impl<'a, T> Value<'a, T>
where
    T: AcceptedTypes + ToValue<T>,
{
    /// Deserializes a [`Value`] from the start of the provided buffer.
    ///
    /// This is the entry point for reading a value encoded with Vinculum's
    /// binary FFI format.
    ///
    /// # Panics
    ///
    /// Panics if the buffer is too small, malformed, contains an unknown tag,
    /// or exceeds internal bounds.
    #[inline]
    pub fn deserialize(buffer: &'a [u8]) -> Self {
        let (value, _) = Self::read_bytes(buffer, 0);

        value
    }

    /// Reads a [`Value`] from `buffer` starting at `index`.
    ///
    /// Returns the decoded value and the next read position.
    ///
    /// # Panics
    ///
    /// Panics if the input is invalid or truncated.
    pub(crate) fn read_bytes(buffer: &'a [u8], index: usize) -> (Self, usize) {
        assert!(index < buffer.len(), "deserialize: index out of bounds");
        assert!(
            index < BUFFER_SIZE,
            "deserialize: index exceeds BUFFER_SIZE"
        );

        let tag = buffer[index];
        let idx = index + 1;

        match tag {
            t if t == Value::<T>::Null.tag() => (Self::Null, idx),
            t if t == Value::<T>::Unit.tag() => (Self::Unit, idx),

            t if t == Value::<T>::Bool(false).tag() => {
                assert!(idx < buffer.len(), "deserialize Bool: missing payload");
                match buffer[idx] {
                    0 => (Self::Bool(false), idx + 1),
                    1 => (Self::Bool(true), idx + 1),
                    _ => panic!("deserialize Bool: invalid boolean payload"),
                }
            }

            t if t == Value::<T>::Char('\0').tag() => {
                assert!(idx < buffer.len(), "deserialize Char: missing payload");
                (Self::Char(buffer[idx] as char), idx + 1)
            }

            t if t == Value::<T>::Int8(0).tag() => {
                assert!(idx < buffer.len(), "deserialize Int8: missing payload");
                (Self::Int8(buffer[idx] as i8), idx + 1)
            }
            t if t == Value::<T>::Int16(0).tag() => {
                assert!(
                    idx + 2 <= buffer.len(),
                    "deserialize Int16: truncated payload"
                );
                let mut bytes = [0u8; 2];
                bytes.copy_from_slice(&buffer[idx..idx + 2]);
                (Self::Int16(i16::from_le_bytes(bytes)), idx + 2)
            }
            t if t == Value::<T>::Int32(0).tag() => {
                assert!(
                    idx + 4 <= buffer.len(),
                    "deserialize Int32: truncated payload"
                );
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(&buffer[idx..idx + 4]);
                (Self::Int32(i32::from_le_bytes(bytes)), idx + 4)
            }
            t if t == Value::<T>::Int64(0).tag() => {
                assert!(
                    idx + 8 <= buffer.len(),
                    "deserialize Int64: truncated payload"
                );
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&buffer[idx..idx + 8]);
                (Self::Int64(i64::from_le_bytes(bytes)), idx + 8)
            }
            t if t == Value::<T>::Int128(0).tag() => {
                assert!(
                    idx + 16 <= buffer.len(),
                    "deserialize Int128: truncated payload"
                );
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&buffer[idx..idx + 16]);
                (Self::Int128(i128::from_le_bytes(bytes)), idx + 16)
            }
            t if t == Value::<T>::Isize(0).tag() => {
                let s = size_of::<isize>();
                assert!(
                    idx + s <= buffer.len(),
                    "deserialize Isize: truncated payload"
                );

                let n = match s {
                    8 => {
                        let mut bytes = [0u8; 8];
                        bytes.copy_from_slice(&buffer[idx..idx + 8]);
                        isize::from_le_bytes(bytes)
                    }
                    4 => {
                        let mut bytes = [0u8; 4];
                        bytes.copy_from_slice(&buffer[idx..idx + 4]);
                        i32::from_le_bytes(bytes) as isize
                    }
                    _ => panic!("unsupported isize width"),
                };

                (Self::Isize(n), idx + s)
            }

            t if t == Value::<T>::U8(0).tag() => {
                assert!(idx < buffer.len(), "deserialize U8: missing payload");
                (Self::U8(buffer[idx]), idx + 1)
            }
            t if t == Value::<T>::U16(0).tag() => {
                assert!(
                    idx + 2 <= buffer.len(),
                    "deserialize U16: truncated payload"
                );
                let mut bytes = [0u8; 2];
                bytes.copy_from_slice(&buffer[idx..idx + 2]);
                (Self::U16(u16::from_le_bytes(bytes)), idx + 2)
            }
            t if t == Value::<T>::U32(0).tag() => {
                assert!(
                    idx + 4 <= buffer.len(),
                    "deserialize U32: truncated payload"
                );
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(&buffer[idx..idx + 4]);
                (Self::U32(u32::from_le_bytes(bytes)), idx + 4)
            }
            t if t == Value::<T>::U64(0).tag() => {
                assert!(
                    idx + 8 <= buffer.len(),
                    "deserialize U64: truncated payload"
                );
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&buffer[idx..idx + 8]);
                (Self::U64(u64::from_le_bytes(bytes)), idx + 8)
            }
            t if t == Value::<T>::U128(0).tag() => {
                assert!(
                    idx + 16 <= buffer.len(),
                    "deserialize U128: truncated payload"
                );
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&buffer[idx..idx + 16]);
                (Self::U128(u128::from_le_bytes(bytes)), idx + 16)
            }
            t if t == Value::<T>::Usize(0).tag() => {
                let s = size_of::<usize>();
                assert!(
                    idx + s <= buffer.len(),
                    "deserialize Usize: truncated payload"
                );

                let n = match s {
                    8 => {
                        let mut bytes = [0u8; 8];
                        bytes.copy_from_slice(&buffer[idx..idx + 8]);
                        usize::from_le_bytes(bytes)
                    }
                    4 => {
                        let mut bytes = [0u8; 4];
                        bytes.copy_from_slice(&buffer[idx..idx + 4]);
                        u32::from_le_bytes(bytes) as usize
                    }
                    _ => panic!("unsupported usize width"),
                };

                (Self::Usize(n), idx + s)
            }

            t if t == Value::<T>::Float32(0.0).tag() => {
                assert!(
                    idx + 4 <= buffer.len(),
                    "deserialize Float32: truncated payload"
                );
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(&buffer[idx..idx + 4]);
                (Self::Float32(f32::from_le_bytes(bytes)), idx + 4)
            }
            t if t == Value::<T>::Float64(0.0).tag() => {
                assert!(
                    idx + 8 <= buffer.len(),
                    "deserialize Float64: truncated payload"
                );
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&buffer[idx..idx + 8]);
                (Self::Float64(f64::from_le_bytes(bytes)), idx + 8)
            }

            t if t == Value::<T>::Str("").tag() => {
                assert!(idx < buffer.len(), "deserialize Str: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(end <= buffer.len(), "deserialize Str: truncated payload");
                let s = std::str::from_utf8(&buffer[start..end])
                    .expect("deserialize Str: invalid UTF-8");

                (Self::Str(s), end)
            }

            t if t == Value::<T>::String(String::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize String: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(end <= buffer.len(), "deserialize String: truncated payload");
                let s = String::from_utf8(buffer[start..end].to_vec())
                    .expect("deserialize String: invalid UTF-8");

                (Self::String(s), end)
            }

            t if t == Value::<T>::Bytes(&[]).tag() => {
                assert!(idx < buffer.len(), "deserialize Bytes: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(end <= buffer.len(), "deserialize Bytes: truncated payload");
                (Self::Bytes(&buffer[start..end]), end)
            }

            t if t == Value::<T>::ByteVec(Vec::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize ByteVec: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(
                    end <= buffer.len(),
                    "deserialize ByteVec: truncated payload"
                );
                (Self::ByteVec(buffer[start..end].to_vec()), end)
            }

            t if t == Value::<T>::CStr(c"").tag() => {
                assert!(idx < buffer.len(), "deserialize CStr: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(end <= buffer.len(), "deserialize CStr: truncated payload");

                let bytes = &buffer[start..end];
                let cstring = CString::new(bytes).expect("deserialize CStr: interior nul byte");
                let leaked = Box::leak(cstring.into_boxed_c_str());

                (Self::CStr(leaked), end)
            }

            t if t == Value::<T>::CString(CString::new("").unwrap()).tag() => {
                assert!(idx < buffer.len(), "deserialize CString: missing length");
                let len = buffer[idx] as usize;
                let start = idx + 1;
                let end = start + len;

                assert!(
                    end <= buffer.len(),
                    "deserialize CString: truncated payload"
                );

                let cstring = CString::new(buffer[start..end].to_vec())
                    .expect("deserialize CString: interior nul byte");

                (Self::CString(cstring), end)
            }

            t if t == Value::<T>::Slice(&[]).tag() => {
                assert!(idx < buffer.len(), "deserialize Slice: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut values = Vec::with_capacity(len);

                for _ in 0..len {
                    let (value, next) = Self::read_bytes(buffer, offset);
                    values.push(value);
                    offset = next;
                }

                let leaked = Box::leak(values.into_boxed_slice());
                (Self::Slice(leaked), offset)
            }

            t if t == Value::<T>::Array(Box::<[Value<'a, T>]>::default()).tag() => {
                assert!(idx < buffer.len(), "deserialize Array: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut values = Vec::with_capacity(len);

                for _ in 0..len {
                    let (value, next) = Self::read_bytes(buffer, offset);
                    values.push(value);
                    offset = next;
                }

                (Self::Array(values.into_boxed_slice()), offset)
            }

            t if t == Value::<T>::Tuple(Box::<[Value<'a, T>]>::default()).tag() => {
                assert!(idx < buffer.len(), "deserialize Tuple: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut values = Vec::with_capacity(len);

                for _ in 0..len {
                    let (value, next) = Self::read_bytes(buffer, offset);
                    values.push(value);
                    offset = next;
                }

                (Self::Tuple(values.into_boxed_slice()), offset)
            }

            t if t == Value::<T>::Vec(Vec::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize Vec: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut values = Vec::with_capacity(len);

                for _ in 0..len {
                    let (value, next) = Self::read_bytes(buffer, offset);
                    values.push(value);
                    offset = next;
                }

                (Self::Vec(values), offset)
            }

            t if t == Value::<T>::Set(Vec::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize Set: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut values = Vec::with_capacity(len);

                for _ in 0..len {
                    let (value, next) = Self::read_bytes(buffer, offset);
                    values.push(value);
                    offset = next;
                }

                (Self::Set(values), offset)
            }

            t if t == Value::<T>::Record(Vec::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize Record: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut fields = Vec::with_capacity(len);

                for _ in 0..len {
                    assert!(
                        offset < buffer.len(),
                        "deserialize Record: missing key length"
                    );
                    let key_len = buffer[offset] as usize;
                    let key_start = offset + 1;
                    let key_end = key_start + key_len;

                    assert!(key_end <= buffer.len(), "deserialize Record: truncated key");

                    let key = String::from_utf8(buffer[key_start..key_end].to_vec())
                        .expect("deserialize Record: invalid UTF-8 key");

                    offset = key_end;

                    let (value, next) = Self::read_bytes(buffer, offset);
                    fields.push((key, value));
                    offset = next;
                }

                (Self::Record(fields), offset)
            }

            t if t == Value::<T>::Map(Vec::new()).tag() => {
                assert!(idx < buffer.len(), "deserialize Map: missing length");
                let len = buffer[idx] as usize;

                let mut offset = idx + 1;
                let mut entries = Vec::with_capacity(len);

                for _ in 0..len {
                    let (key, next1) = Self::read_bytes(buffer, offset);
                    let (value, next2) = Self::read_bytes(buffer, next1);
                    entries.push((key, value));
                    offset = next2;
                }

                (Self::Map(entries), offset)
            }

            t if t == Value::<T>::Option(None).tag() => {
                assert!(idx < buffer.len(), "deserialize Option: missing option tag");

                match buffer[idx] {
                    0 => (Self::Option(None), idx + 1),
                    1 => {
                        let (value, next) = Self::read_bytes(buffer, idx + 1);
                        (Self::Option(Some(Box::new(value))), next)
                    }
                    _ => panic!("deserialize Option: invalid option tag"),
                }
            }

            t if t == Value::<T>::Result(Ok(Box::new(Self::Unit))).tag() => {
                assert!(idx < buffer.len(), "deserialize Result: missing result tag");

                match buffer[idx] {
                    0 => {
                        let (err, next) = Self::read_bytes(buffer, idx + 1);
                        (Self::Result(Err(Box::new(err))), next)
                    }
                    1 => {
                        let (ok, next) = Self::read_bytes(buffer, idx + 1);
                        (Self::Result(Ok(Box::new(ok))), next)
                    }
                    _ => panic!("deserialize Result: invalid result tag"),
                }
            }

            _ => unimplemented!("deserialize: unknown or unsupported tag {}", tag),
        }
    }
}
