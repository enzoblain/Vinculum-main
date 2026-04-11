use crate::codegen::types::errors::InvalidArgumentName;
use crate::codegen::types::traits::FfiLangType;

/// A named function argument associated with a language-specific FFI type.
///
/// `Arg<T>` is generic over the backend type representation, which makes it
/// reusable across multiple code generation targets.
///
/// # Invariants
///
/// The validity of [`Arg::name`] depends on the constructor used:
/// - [`Arg::new`] stores the provided name unchanged
/// - [`Arg::try_new`] validates and normalizes the name before storing it
///
/// Prefer [`Arg::try_new`] whenever the name originates from parsed input
/// or any other unchecked source.
pub struct Arg<T>
where
    T: FfiLangType,
{
    /// Argument name.
    ///
    /// When constructed through [`Arg::try_new`], this value is guaranteed to
    /// be validated and normalized for Rust code generation.
    pub(crate) name: String,

    /// Language-specific type associated with the argument.
    pub(crate) r#type: T,
}

impl<T> Arg<T>
where
    T: FfiLangType,
{
    /// Creates a new argument after validating and normalizing its name.
    ///
    /// The provided name must satisfy [`is_valid_variable_name`]. If valid,
    /// it is then passed through [`normalize_arg_name`] before being stored.
    ///
    /// Rust keywords are accepted as input, but are normalized into raw
    /// identifiers such as `r#type` for code generation.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidArgumentName`] if the provided name does not match
    /// the expected variable naming rules.
    pub fn try_new(name: impl Into<String>, r#type: T) -> Result<Self, InvalidArgumentName> {
        let name: String = name.into();

        if !is_valid_variable_name(&name) {
            return Err(InvalidArgumentName(name));
        }

        Ok(Self {
            name: normalize_arg_name(name),
            r#type,
        })
    }
}

/// Returns whether `name` is accepted as a source-level argument name.
///
/// Accepted names follow a restricted identifier format:
/// - they must not be empty
/// - they must start with a lowercase ASCII letter
/// - remaining characters may only be ASCII alphanumeric characters,
///   underscores (`_`), or apostrophes (`'`)
///
/// This function only checks structural validity. Rust keywords are still
/// considered valid at this stage and are handled separately by
/// [`normalize_arg_name`].
///
/// The accepted syntax is intentionally close to simple identifier forms
/// commonly used in functional language frontends.
#[inline]
pub(crate) fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut bytes = name.as_bytes().iter();

    let Some(&first) = bytes.next() else {
        return false;
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    bytes.all(|&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'\'')
}

/// Normalizes an argument name for Rust code generation.
///
/// If the provided name is a Rust keyword, it is converted into a raw
/// identifier by prefixing it with `r#`. Otherwise, it is returned unchanged.
///
/// This function assumes the input is already structurally valid and does not
/// perform validation on its own.
#[inline]
pub(crate) fn normalize_arg_name(name: String) -> String {
    if is_rust_keyword(&name) {
        format!("r#{name}")
    } else {
        name
    }
}

/// Returns whether `name` is treated as a Rust keyword by the code generator.
///
/// Keywords detected here are escaped through raw identifier syntax during
/// name normalization.
pub(crate) fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    )
}
