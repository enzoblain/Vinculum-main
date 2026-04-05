pub enum Value<T> {
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

    Generic(T),
    Tuple(Vec<Value<T>>),
}

pub trait AcceptedTypes {}

macro_rules! impl_accepted_types {
    ($($t:ty),* $(,)?) => {
        $(
            impl AcceptedTypes for $t {}
        )*
    };
}

impl_accepted_types!(
    i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char, String
);
