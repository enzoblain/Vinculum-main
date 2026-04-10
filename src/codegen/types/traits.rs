use crate::codegen::types::generics::GenericResolver;

/// Represents a type that can cross the FFI boundary.
///
/// This trait defines the minimal information required to:
/// - map the type to Rust
/// - handle generic resolution
/// - decide how return values should be converted
pub trait FfiType {
    /// Returns the Rust type name corresponding to this FFI type.
    ///
    /// Example: `"i64"`, `"String"`, `"(i32, bool)"`.
    fn rust_type_name(&self) -> String;

    /// Returns the conversion method to apply on FFI return values.
    ///
    /// Typically:
    /// - `"try_into()"` for concrete types
    /// - `"into_generic()"` for generic types
    fn rust_return_conversion(&self) -> &'static str;

    /// Indicates whether this type (or any nested type) is generic.
    fn is_generic(&self) -> bool;

    /// Resolves generic type variables into concrete Rust type names.
    ///
    /// This walks the type recursively and replaces generic identifiers
    /// (e.g. `"a"`, `"b"`) with stable Rust generic names (e.g. `"T0"`, `"T1"`).
    ///
    /// The provided [`GenericResolver`] maintains a consistent mapping,
    /// ensuring that the same generic name always resolves to the same
    /// Rust type across the entire type structure.
    fn resolve_generics(&mut self, resolver: &mut GenericResolver);
}

/// Provides code generation utilities for FFI types.
///
/// This trait is responsible for producing Rust and target-language
/// expressions used during code generation.
pub trait FfiTypeCodegen {
    /// Generates a Rust expression that wraps a value into an FFI `Value`.
    ///
    /// Example output:
    /// `Value::<T>::Int64(x)` or `Value::<T>::Tuple(vec![...])`
    fn rust_value_expr(&self, value_name: &str, type_param: &str) -> String;

    /// Generates a pattern used to destructure a value
    /// in the target language (e.g. Haskell).
    ///
    /// Example: `VInt64 x`, `VTuple [a, b]`
    fn target_pattern(&self, binding_name: &str) -> String;

    /// Generates an expression converting a bound value
    /// into a usable value in the target language.
    ///
    /// Example: `(fromIntegral x)` or `(a, b)`
    fn target_value_expr(&self, binding_name: &str) -> String;
}

/// Convenience trait for types that can both describe an FFI type and
/// participate in code generation.
///
/// This trait does not define additional behavior. It only groups
/// [`FfiType`] and [`FfiTypeCodegen`] under a single bound to simplify
/// generic constraints.
pub trait FfiLangType: FfiType + FfiTypeCodegen {}

impl<T> FfiLangType for T where T: FfiType + FfiTypeCodegen {}
