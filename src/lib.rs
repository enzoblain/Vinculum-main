//! Vinculum core library.
//!
//! This crate provides the core abstractions for building FFI bridges
//! using a unified and strongly-typed value system.
//!
//! It exposes two main modules:
//!
//! - [`codec`]: runtime representation and conversion of values exchanged
//!   across FFI boundaries
//! - [`codegen`]: type system and utilities for generating bindings and
//!   glue code
//!
//! The [`codec`] module defines:
//! - [`codec::Value`]: the unified representation of FFI data
//! - traits such as [`codec::AcceptedTypes`] and [`codec::ToValue`]
//! - macros to register supported codecs
//!
//! Most users will primarily interact with the [`ffi`] layer built on top
//! of these components.
pub mod codec;
pub mod codegen;
