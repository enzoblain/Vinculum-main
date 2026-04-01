pub(crate) enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),

    Word8(u8),
    Word16(u16),
    Word32(u32),
    Word64(u64),

    Float32(f32),
    Float64(f64),

    Bool(bool),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    Option(Option<Box<Value>>),
    Vec(Vec<Value>),
}
