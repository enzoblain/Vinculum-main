use crate::codegen::types::args::Arg;
use crate::codegen::types::derive::Derive;
use crate::codegen::types::traits::FfiLangType;
use crate::codegen::types::GenericResolver;

/// Representation of a function in the code generation pipeline.
///
/// A `Function<T>` models a function signature together with its associated
/// metadata. The type parameter `T` defines the language-specific type
/// representation used for both arguments and return type.
pub struct Function<T>
where
    T: FfiLangType,
{
    /// Documentation lines associated with the function.
    pub(crate) documentation: Vec<String>,

    /// Derive attributes applied to the generated item, if applicable.
    pub(crate) derives: Vec<Derive>,

    /// Function name.
    pub(crate) name: String,

    pub(crate) generic_resolver: GenericResolver,

    /// Function arguments.
    pub(crate) args: Vec<Arg<T>>,

    /// Return type of the function.
    pub(crate) return_type: T,
}
