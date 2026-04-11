use std::ffi::{CStr, CString, c_void};

use crate::codec::AcceptedTypes;

/// Dynamically typed value used by Vinculum's FFI layer.
///
/// This enum is intended to represent the most common value kinds crossing
/// language and ABI boundaries:
/// - primitive scalars
/// - Rust and C strings
/// - raw bytes
/// - pointers, handles, and callbacks
/// - references and collections
/// - structured key-value data
/// - optional and result values
/// - user-defined accepted values through `T`
#[repr(u8)]
pub enum Value<'a, T: AcceptedTypes> {
    // Special / unit-like
    Null,
    Unit, // ()

    // Booleans and characters
    Bool(bool),
    Char(char),

    // Signed integers
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Isize(isize),

    // Unsigned integers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    // Floating-point numbers
    Float32(f32),
    Float64(f64),

    // Rust text
    Str(&'a str),
    String(String),

    // C text
    CStr(&'a CStr),
    CString(CString),

    // Raw bytes
    Bytes(&'a [u8]),
    ByteVec(Vec<u8>),

    // Opaque pointers / handles / callable opaque values
    Ptr(*const c_void),
    MutPtr(*mut c_void),
    Handle(u64),
    FnPtr(*const c_void),

    // References
    // Not for now, i want us to use it
    // ```rust
    // let mut x = 10;
    //
    // let addr = (&mut x as *mut i32) as usize;
    // let ptr = addr as *mut i32;
    //
    // unsafe {
    //     *ptr = 42;
    // }
    // ```
    Ref(&'a Value<'a, T>),
    MutRef(&'a mut Value<'a, T>),

    // Borrowed / fixed collections
    Slice(&'a [Value<'a, T>]),
    Array(Box<[Value<'a, T>]>),
    Tuple(Box<[Value<'a, T>]>),

    // Owned dynamic collections
    Vec(Vec<Value<'a, T>>),
    Set(Vec<Value<'a, T>>),

    // Structured / associative data
    Record(Vec<(String, Value<'a, T>)>),
    Map(Vec<(Value<'a, T>, Value<'a, T>)>),

    // Wrappers / algebraic helpers
    Option(Option<Box<Value<'a, T>>>),
    Result(Result<Box<Value<'a, T>>, Box<Value<'a, T>>>),

    // Opaque value validated by the FFI type system
    Generic(T),
}

impl<'a, T: AcceptedTypes> Value<'a, T> {
    /// Returns the raw numeric discriminant of this value.
    ///
    /// The returned value corresponds to the variant order defined in this enum.
    /// Reordering variants is therefore a breaking change for any serialized,
    /// persisted, or FFI-exposed tag representation.
    #[inline]
    pub fn tag(&self) -> u8 {
        unsafe { *(self as *const _ as *const u8) }
    }
}

/// Marker type used to represent `null`.
///
/// This is a struct instead of using `()` so it can be distinguished
/// from Rust's unit type and have its own implementation.
pub struct Null();

/// Wrapper around `u64` used to represent opaque handles.
///
/// Using a struct avoids conflicts with the regular `u64` implementation
/// and allows treating handles differently.
pub struct Handle(pub u64);

/// Wrapper around a raw function pointer.
///
/// This is separate from `*const c_void` to avoid ambiguity with other
/// pointer codec and allow a dedicated implementation.
pub struct FnPtr(pub *const c_void);

/// Wrapper around a boxed slice of [`Value`].
///
/// This exists to distinguish arrays from other collections like tuples,
/// even if they share the same underlying representation.
pub struct Array<'a, T: AcceptedTypes>(pub Box<[Value<'a, T>]>);

/// Wrapper around a boxed slice of [`Value`].
///
/// This is separate from [`Array`] so backends can differentiate
/// tuple semantics from generic arrays.
pub struct Tuple<'a, T: AcceptedTypes>(pub Box<[Value<'a, T>]>);
