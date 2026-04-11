# Vinculum

[![Crates.io](https://img.shields.io/crates/v/vinculum-main.svg)](https://crates.io/crates/vinculum-main)
[![Docs.rs](https://docs.rs/vinculum-main/badge.svg)](https://docs.rs/vinculum-main)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/your-org/vinculum/ci.yml?branch=main)](https://github.com/enzoblain/vinculum/actions)

> Call a Haskell function as if it were a Rust function: no `unsafe`, no handwritten bindings.

Vinculum generates Rust binding files at build time by reading the target language's exported function signatures. The result is idiomatic, type-safe Rust code that calls across the language boundary without any FFI boilerplate.

This crate is the **shared foundation** of the ecosystem: common types, binary serialization format, and compile-time safety guarantees. Language-specific crates (e.g. `vinculum-hs`) depend on it and handle the runtime bridge and codegen.

---

## Architecture

```
┌──────────────────────────────────────┐
│           vinculum-main              │
│   Value · AcceptedTypes · ToValue    │
│   Serialization · Macros             │
└──────────────────┬───────────────────┘
                   │
          ┌──────────────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
          │  vinculum-hs     │      more backends
          │   (Haskell)      │    │    planned · · ·    │
          └──────────────────┘    └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
```

Every backend shares the same `Value` type and binary format. The only difference is how each backend moves bytes across the language boundary.

---

## What This Crate Provides

### `Value`

A unified, backend-agnostic representation of all data exchanged through Vinculum:

```rust,ignore
Value::Int64(42)
Value::Float64(3.14)
Value::String("hello".to_string())
Value::Bool(true)
Value::Array(vec![...])
Value::Unit
```

### `AcceptedTypes`

A compile-time mechanism that restricts which Rust types may cross the FFI boundary. Types must be explicitly registered before use:

```rust
accepted_i64!();
accepted_string!();
```

Attempting to use an unregistered type is a **compile error**, not a runtime panic.

### `ToValue`

A conversion trait that transforms Rust values into `Value`:

```rust,ignore
let v: Value = 42i64.to_value();
```

Implementations are generated automatically for registered types. Custom implementations can be provided manually for complex types.

### Serialization format

A deterministic binary format designed to be simple, compact, and easy to implement in any language:

```
[tag: 1 byte][payload: variable]
```

- 1-byte tag identifying the value type
- Little-endian encoding for all numeric types
- Length-prefixed encoding for dynamic data (strings, arrays)
- Recursive structure for nested values

Implementing a new backend means implementing a decoder for this format — the data model is already defined here.

---

## Quick Start

> For end-to-end usage including codegen and the Haskell bridge, see [`vinculum-hs`](https://github.com/your-org/vinculum-hs).

`vinculum-main` is typically pulled in as a transitive dependency. If you are implementing a new backend or working with the core types directly:

```toml
[dependencies]
vinculum-main = "0.1"
```

---

## Binding generation

The core value proposition of Vinculum is **automatic binding generation**. At build time, a language-specific backend (e.g. `vinculum-hs`) reads the exported function signatures from the target language and generates a Rust source file — one typed function per export.

```rust
// generated — do not edit
pub fn add(a: i64, b: i64) -> i64 { ... }
pub fn greet(name: String) -> String { ... }
```

These functions are safe, typed, and call directly into the foreign library. No `extern "C"`, no raw pointers, no manual serialization.

`vinculum-main` provides the types and serialization layer that generated bindings are built on.

---

## Ecosystem

| Crate           | Status  | Description                                          |
| --------------- | ------- | ---------------------------------------------------- |
| `vinculum-main` | Active  | Shared types, traits, and serialization (this crate) |
| `vinculum-hs`   | Active  | Haskell ↔ Rust bridge                                |
| more backends   | Planned | —                                                    |

---

## Roadmap

- [x] Core `Value` type system
- [x] Binary serialization format
- [x] Compile-time type validation
- [x] Deserialization API
- [ ] Rust binding file generation
- [ ] Safer buffer handling (bounded writers, error propagation)
- [ ] `no_std` compatibility
- [ ] Backend-specific extension traits
- [ ] Performance benchmarks

---

## Contributing

Contributions are welcome. Please open an issue before submitting large changes.

Because this crate is the foundation of the entire ecosystem, pull requests are held to a higher standard than in language-specific crates:

- **Minimal**: small, focused changes preferred
- **Generic**: no language-specific logic
- **Backward-compatible**: breaking changes require a major version bump and prior discussion

---

## License

This project is licensed under the [MIT](LICENSE) license.
