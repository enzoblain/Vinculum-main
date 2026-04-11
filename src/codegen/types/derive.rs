use strum_macros::AsRefStr;

/// Rust derive attributes used during code generation.
///
/// Each variant corresponds to a standard library trait that can be
/// derived via `#[derive(...)]`. The string representation of a variant
/// matches its Rust identifier and can be obtained with [`AsRefStr`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
pub enum Derive {
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
}
