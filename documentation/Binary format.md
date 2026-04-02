# Vinculum Binary Format

This section defines how Rust and Haskell values are serialized into a compact binary representation.

---

## Overview

Each encoded value follows the same structure:

| Field | Size     | Description               |
|-------|----------|---------------------------|
| Tag   | 1 byte   | Identifies the value type |
| Data  | variable | Encoded payload           |

### Encoding Rules

- The **tag** is a single byte (`Word8`) identifying the type.
- The **payload** format depends on the tag.
- All numeric values are encoded in **little-endian**.
- Variable-size types (`String`, `Bytes`) are **length-prefixed** using a 64-bit unsigned integer.
- Nested values (e.g. `Option`) contain a full encoded value, including its own tag.

---

## Primitive Types

These types have a fixed payload size.

| Rust   | Haskell  | `Value` Variant | Tag        | Payload | Description                 |
|--------|----------|-----------------|------------|---------|-----------------------------|
| `i8`   | `Int8`   | `VInt8`         | `00000000` | 1 byte  | signed 8-bit integer        |
| `i16`  | `Int16`  | `VInt16`        | `00000001` | 2 bytes | signed 16-bit integer       |
| `i32`  | `Int32`  | `VInt32`        | `00000010` | 4 bytes | signed 32-bit integer       |
| `i64`  | `Int64`  | `VInt64`        | `00000011` | 8 bytes | signed 64-bit integer       |
| `u8`   | `Word8`  | `VWord8`        | `00000100` | 1 byte  | unsigned 8-bit integer      |
| `u16`  | `Word16` | `VWord16`       | `00000101` | 2 bytes | unsigned 16-bit integer     |
| `u32`  | `Word32` | `VWord32`       | `00000110` | 4 bytes | unsigned 32-bit integer     |
| `u64`  | `Word64` | `VWord64`       | `00000111` | 8 bytes | unsigned 64-bit integer     |
| `f32`  | `Float`  | `VFloat32`      | `00001000` | 4 bytes | IEEE-754 single precision   |
| `f64`  | `Double` | `VFloat64`      | `00001001` | 8 bytes | IEEE-754 double precision   |
| `bool` | `Bool`   | `VBool`         | `00001010` | 1 byte  | 0 = false, 1 = true         |
| `char` | `Char`   | `VChar`         | `00001011` | 4 bytes | Unicode code point (UTF-32) |

> Tags are shown in binary form for clarity but are stored as a single byte.

---

## Complex Types (Variable Size)

These types include additional metadata (e.g. length or nested values).

| Rust        | Haskell      | `Value` Variant | Tag        | Payload                         | Description                                                                                         |
|-------------|--------------|-----------------|------------|---------------------------------|-----------------------------------------------------------------------------------------------------|
| `String`    | `String`     | `VString`       | `00001100` | 8 bytes + N bytes               | length-prefixed UTF-8 string                                                                        |
| `Vec<u8>`   | `ByteString` | `VBytes`        | `00001101` | 8 bytes + N bytes               | length-prefixed raw bytes                                                                           |
| `Option<T>` | `Maybe a`    | `VOption`       | `00001110` | 1 byte + **full encoded value** | **recursive**: optional value of any type (can nest infinitely, e.g. `Option<Option<Option<i64>>>`) |
| `Vec<Value>` | `[Value]`   | `VVec`          | `00001111` | 8 bytes + N * encoded values    | **recursive**: length-prefixed sequence of heterogeneous values, each with its own tag               |

---

## How Option Recursion Works

### The Mechanism

When encoding/decoding an `Option<T>`, the payload consists of:

1. **Sub-tag** (1 byte):
    - `0` = `None` (nothing follows)
    - `1` = `Some` (followed by a **full encoded value**)

2. **Full Encoded Value** (variable, only if sub-tag is `1`):
    - Any complete encoded value, **including its main tag**
    - This can be an `i64`, a `String`, **or another `Option`**

### Example: `Option<Option<i64>>`

**Case 1: `None`**

```
[14, 0]
```

**Case 2: `Some(None)`**

```
[14, 1, 14, 0]
 ↑   ↑  ↑   ↑
 |   |  |   +-- inner Option is None (sub-tag 0)
 |   |  +------ inner Option tag (14)
 |   +--------- outer Option is Some (sub-tag 1)
 +------------- outer Option tag (14)
```

**Case 3: `Some(Some(42))`**

```
[14, 1, 14, 1, 3, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
 ↑   ↑  ↑   ↑  ↑  ↑                                                 ↑
 |   |  |   |  |  +-- i64 payload (8 bytes, little-endian 42)
 |   |  |   |  +------ i64 tag (3)
 |   |  |   +--------- inner Option is Some (sub-tag 1)
 |   |  +------------- inner Option tag (14)
 |   +---------------- outer Option is Some (sub-tag 1)
 +-------------------- outer Option tag (14)
```

### Why This Works

The key insight: **`Option` doesn't care what type is inside.** The payload after sub-tag `1` is always a **complete,
self-describing value with its own tag**. The decoder reads:

- **Rust** (`from_bytes_checked`): Recursively calls itself to decode whatever tag comes next
- **Haskell** (`decodeOne`): Recursively calls itself to decode whatever tag comes next

**No depth limit** — only available memory bounds the nesting.

---