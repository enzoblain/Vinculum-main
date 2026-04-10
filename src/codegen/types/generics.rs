use std::collections::HashMap;

/// Resolves generic type variables into stable Rust generic names.
///
/// This struct maintains a mapping between source language generic identifiers
/// (e.g. `"a"`, `"b"`) and generated Rust type names (e.g. `"T0"`, `"T1"`).
///
/// It guarantees that:
/// - the same input name always resolves to the same Rust name
/// - new generic names are generated only once and reused consistently
///
/// Typically used during type resolution before code generation.
pub struct GenericResolver {
    /// Mapping from source generic names (e.g. `"a"`) to Rust names (e.g. `"T0"`).
    names: HashMap<String, String>,

    /// Counter used to generate fresh Rust generic names.
    next_index: usize,
}

impl GenericResolver {
    /// Creates a new empty resolver.
    ///
    /// The first resolved generic will be `"T0"`, then `"T1"`, etc.
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            next_index: 0,
        }
    }

    /// Resolves a generic name into a stable Rust generic identifier.
    ///
    /// If the name was already resolved, returns the existing mapping.
    /// Otherwise, generates a new Rust generic name (e.g. `"T0"`, `"T1"`),
    /// stores it, and returns it.
    ///
    /// # Example
    /// - `"a"` → `"T0"`
    /// - `"b"` → `"T1"`
    /// - `"a"` → `"T0"` (same as before)
    pub fn resolve(&mut self, name: &str) -> String {
        if let Some(existing) = self.names.get(name) {
            return existing.clone();
        }

        let generated = format!("T{}", self.next_index);

        self.next_index += 1;
        self.names.insert(name.to_string(), generated.clone());

        generated
    }

    /// Returns all resolved Rust generic names in a stable order.
    ///
    /// The returned list is sorted by generation order (`T0`, `T1`, ...),
    /// making it suitable for generating generic parameter lists.
    ///
    /// # Example
    /// If `"a"` → `"T0"` and `"b"` → `"T1"`,
    /// this returns `["T0", "T1"]`.
    pub(crate) fn all(&self) -> Vec<String> {
        let mut values: Vec<_> = self.names.values().cloned().collect();

        values.sort_by_key(|name| {
            name.strip_prefix('T')
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap_or(usize::MAX)
        });

        values
    }
}

impl Default for GenericResolver {
    fn default() -> Self {
        Self::new()
    }
}
