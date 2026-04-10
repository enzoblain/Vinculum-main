use super::accepted::ToValue;
use super::{AcceptedTypes, Value, BUFFER_SIZE};

impl<'a, T> Value<'a, T>
where
    T: AcceptedTypes + ToValue<T>,
{
    /// Serializes this value into the provided buffer starting at index `0`.
    ///
    /// This is the entry point for writing a [`Value`] into a preallocated byte
    /// buffer using Vinculum's binary FFI format.
    ///
    /// Internally, this forwards to `Value::write_bytes`.
    ///
    /// Not all [`Value`] variants are currently supported for serialization.
    /// Attempting to serialize an unsupported variant will result in a panic.
    ///
    /// # Panics
    ///
    /// Panics if the serialized value exceeds the internal buffer limits,
    /// if the provided buffer is too small, or if the value is not yet supported.
    #[inline]
    pub fn serialize(&self, buffer: &mut [u8]) {
        let _ = self.write_bytes(buffer, 0);
    }

    /// Writes this value into the provided buffer starting at `index`.
    ///
    /// Returns the next write position after the encoded value.
    ///
    /// This method is used internally for recursive serialization of nested
    /// [`Value`] structures such as arrays, tuples, maps, options, and results.
    ///
    /// The encoded layout always starts with the value tag, followed by the
    /// variant-specific payload.
    ///
    /// Not all [`Value`] variants are currently implemented.
    /// Unsupported variants will trigger a panic via `unimplemented!()`.
    ///
    /// # Parameters
    ///
    /// - `buffer`: destination byte buffer
    /// - `index`: starting position where the value should be written
    ///
    /// # Returns
    ///
    /// The index immediately after the last written byte.
    ///
    /// # Panics
    ///
    /// Panics if writing would exceed the allowed buffer size, if the value
    /// cannot fit into the provided buffer, or if the variant is not implemented.
    pub(crate) fn write_bytes(&self, buffer: &mut [u8], index: usize) -> usize {
        assert!(index < BUFFER_SIZE);
        buffer[index] = self.tag();

        let idx = index + 1;

        match self {
            Self::Null | Self::Unit => {}

            Self::Bool(b) => {
                assert!(idx < BUFFER_SIZE);
                buffer[idx] = if *b { 1 } else { 0 };
            }
            Self::Char(c) => {
                assert!(idx < BUFFER_SIZE);
                buffer[idx] = *c as u8;
            }

            Self::Int8(n) => {
                assert!(idx < BUFFER_SIZE);
                buffer[idx] = *n as u8;
            }
            Self::Int16(n) => {
                assert!(idx + 2 <= BUFFER_SIZE);
                buffer[idx..idx + 2].copy_from_slice(&n.to_le_bytes());
            }
            Self::Int32(n) => {
                assert!(idx + 4 <= BUFFER_SIZE);
                buffer[idx..idx + 4].copy_from_slice(&n.to_le_bytes());
            }
            Self::Int64(n) => {
                assert!(idx + 8 <= BUFFER_SIZE);
                buffer[idx..idx + 8].copy_from_slice(&n.to_le_bytes());
            }
            Self::Int128(n) => {
                assert!(idx + 16 <= BUFFER_SIZE);
                buffer[idx..idx + 16].copy_from_slice(&n.to_le_bytes());
            }
            Self::Isize(n) => {
                assert!(idx + size_of::<isize>() <= BUFFER_SIZE);
                buffer[idx..idx + size_of::<isize>()].copy_from_slice(&n.to_le_bytes());
            }

            Self::U8(n) => {
                assert!(idx < BUFFER_SIZE);
                buffer[idx] = *n;
            }
            Self::U16(n) => {
                assert!(idx + 2 <= BUFFER_SIZE);
                buffer[idx..idx + 2].copy_from_slice(&n.to_le_bytes());
            }
            Self::U32(n) => {
                assert!(idx + 4 <= BUFFER_SIZE);
                buffer[idx..idx + 4].copy_from_slice(&n.to_le_bytes());
            }
            Self::U64(n) => {
                assert!(idx + 8 <= BUFFER_SIZE);
                buffer[idx..idx + 8].copy_from_slice(&n.to_le_bytes());
            }
            Self::U128(n) => {
                assert!(idx + 16 <= BUFFER_SIZE);
                buffer[idx..idx + 16].copy_from_slice(&n.to_le_bytes());
            }
            Self::Usize(n) => {
                assert!(idx + size_of::<usize>() <= BUFFER_SIZE);
                buffer[idx..idx + size_of::<usize>()].copy_from_slice(&n.to_le_bytes());
            }

            Self::Float32(n) => {
                assert!(idx + 4 <= BUFFER_SIZE);
                buffer[idx..idx + 4].copy_from_slice(&n.to_le_bytes());
            }
            Self::Float64(n) => {
                assert!(idx + 8 <= BUFFER_SIZE);
                buffer[idx..idx + 8].copy_from_slice(&n.to_le_bytes());
            }

            Self::Str(s) => {
                let len = s.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(s.as_bytes());
            }
            Self::String(s) => {
                let len = s.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(s.as_bytes());
            }

            Self::CStr(s) => {
                let bytes = s.to_bytes();
                let len = bytes.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(bytes);
            }
            Self::CString(s) => {
                let bytes = s.as_c_str().to_bytes();
                let len = bytes.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(bytes);
            }

            Self::Bytes(bytes) => {
                let len = bytes.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(bytes);
            }
            Self::ByteVec(bytes) => {
                let len = bytes.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx + 1 + len <= BUFFER_SIZE);

                buffer[idx] = len as u8;
                buffer[idx + 1..idx + 1 + len].copy_from_slice(bytes);
            }

            Self::Slice(values) => {
                let len = values.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for v in *values {
                    offset = v.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Array(values) => {
                let len = values.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for v in values.iter() {
                    offset = v.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Tuple(values) => {
                let len = values.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for v in values.iter() {
                    offset = v.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Vec(values) | Self::Set(values) => {
                let len = values.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for v in values.iter() {
                    offset = v.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Record(fields) => {
                let len = fields.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for (key, value) in fields.iter() {
                    let l = key.len();

                    assert!(l <= u8::MAX as usize);
                    assert!(offset + 1 + l <= BUFFER_SIZE);

                    buffer[offset] = l as u8;
                    buffer[offset + 1..offset + 1 + l].copy_from_slice(key.as_bytes());
                    offset += 1 + l;

                    offset = value.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Map(entries) => {
                let len = entries.len();

                assert!(len <= u8::MAX as usize);
                assert!(idx < BUFFER_SIZE);

                buffer[idx] = len as u8;

                let mut offset = idx + 1;
                for (k, v) in entries.iter() {
                    offset = k.write_bytes(buffer, offset);
                    offset = v.write_bytes(buffer, offset);
                }

                return offset;
            }

            Self::Option(opt) => {
                assert!(idx < BUFFER_SIZE);

                return match opt {
                    None => {
                        buffer[idx] = 0;
                        idx + 1
                    }
                    Some(v) => {
                        buffer[idx] = 1;
                        v.write_bytes(buffer, idx + 1)
                    }
                };
            }

            Self::Result(res) => {
                assert!(idx < BUFFER_SIZE);

                let (tag, v) = match res {
                    Ok(v) => (1, v),
                    Err(e) => (0, e),
                };

                buffer[idx] = tag;
                v.write_bytes(buffer, idx + 1);
            }

            Self::Generic(g) => {
                return g.to_value().write_bytes(buffer, idx);
            }

            _ => unimplemented!(),
        }

        idx
    }
}
