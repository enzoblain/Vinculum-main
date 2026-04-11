//! Core FFI codec and traits for the Vinculum runtime.
//!
//! This module provides:
//!
//! - [`Value`]: the unified representation of data exchanged across FFI boundaries
//! - [`AcceptedTypes`]: a marker trait defining which Rust codec are allowed
//!   for a given backend
//! - [`ToValue`]: a conversion trait used to transform Rust values into [`Value`]
//!
//! It also exposes helper wrapper codec such as [`Null`], [`Handle`],
//! [`FnPtr`], [`Array`], and [`Tuple`] to disambiguate representations that
//! would otherwise conflict or be ambiguous at compile time.
//!
//! Most users should not implement traits manually and instead rely on the
//! provided `accepted_*` macros to register supported codec.

pub(crate) mod accepted;
mod deserialize;
mod serialize;
mod value;

pub use accepted::{AcceptedTypes, ToValue};
pub use value::{Array, FnPtr, Handle, Null, Tuple, Value};

/// Maximum buffer size used for serialization.
///
/// This constant defines the upper bound for the number of bytes that can be
/// written when serializing a [`Value`] into a buffer.
///
/// It is used to prevent buffer overflows and enforce a fixed-size limit
/// during encoding.
///
/// Serialization will panic if this limit is exceeded.
pub(super) const BUFFER_SIZE: usize = 1028 * 1028;
