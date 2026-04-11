use super::args::Arg;
use super::derive::Derive;
use super::traits::FfiLangType;

/// Representation of a struct in the code generation pipeline.
///
/// A `Struct<T>` models a struct definition together with its associated
/// metadata. The type parameter `T` defines the language-specific type
/// representation used for fields.
pub struct Struct<T>
where
    T: FfiLangType,
{
    /// Documentation lines associated with the struct.
    pub(crate) documentation: Vec<String>,

    /// Derive attributes applied to the struct.
    pub(crate) derives: Vec<Derive>,

    /// Struct name.
    pub(crate) name: String,

    /// Struct fields.
    pub(crate) fields: Vec<Arg<T>>,
}
