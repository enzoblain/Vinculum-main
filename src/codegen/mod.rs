//! Code generation utilities and intermediate representations.
//!
//! This module provides the building blocks required to describe and
//! transform functions and types before generating target-language bindings.
//!
//! The [`types`] module defines the core type system used during code
//! generation, including function signatures, argument representations,
//! and FFI-aware type abstractions.

pub mod rust;
pub mod types;
