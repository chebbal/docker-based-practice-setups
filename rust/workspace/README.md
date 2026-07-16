# Workspace

A workspace is a set of related packages that share one Cargo.lock and
output directory. Each package contains one or more crates (a crate is the
unit of compilation — one library or binary). Use a workspace to organize a
large project as multiple packages, e.g. a Maven multi-module project.

Note: Rust's `mod` (namespacing within a crate) is the concept closest to a
Java package or Python module — not the workspace.

## Modules vs C++ classes

`mod` is not the C++ `class`/`struct` analog — it maps to a C++ `namespace`.
Rust splits into separate features what C++ fuses into one `class`:

| C++ (all in `class`)          | Rust                          |
| ----------------------------- | ----------------------------- |
| namespacing (`Foo::bar`)      | `mod`                         |
| data layout (members)         | `struct` / `enum` fields      |
| methods (`this`)              | `impl` block (`self`)         |
| access control                | `pub` keyword                 |
| constructor                   | associated fn, e.g. `new()`   |
| destructor (RAII)             | `Drop` trait                  |
| virtual / abstract            | `trait` + `dyn`               |

So the `class` equivalent is `struct` + `impl` (+ `trait` for polymorphism).
A `mod` is a compile-time namespace and visibility boundary — it holds no
data, is not a type, and cannot be instantiated.

Two things that trip up C++ people:

1. **Privacy is module-scoped, not type-scoped.** Private means "this module
   and its descendants," not "this type only." Two structs in the same `mod`
   can touch each other's private fields — no `friend` needed. The module,
   not the struct, is the encapsulation unit.

2. **There is no inheritance to subtract.** Rust has no implementation
   inheritance at all. Reuse is done by composition + traits (with default
   methods), never by extending a base class.

Shortcut: `mod` ↔ `namespace`, `struct` + `impl` ↔ `class`, `trait` ↔
abstract base / interface.

## Where tests live

Rust has two test locations for two purposes:

1. **Unit tests → same file**, in a `#[cfg(test)] mod tests` submodule (see
   `hellolib/src/lib.rs`). `#[cfg(test)]` compiles the module only during
   `cargo test`, so it costs nothing in release builds. Because a child
   module can see its ancestors' private items, these tests can exercise
   private functions. `use super::*;` pulls the parent module's items into
   scope so tests can call them unqualified.

2. **Integration tests → `tests/`**, a directory at the package root. Each
   file there compiles as its own crate that links the package, so it sees
   **only** the public API — use it to test the public contract as a
   consumer would.

   ```text
   hellolib/
     src/lib.rs
     tests/
       integration.rs
   ```

Rule of thumb: testing private guts → inline `mod tests`; testing the public
API → `tests/` file. Same-file unit tests are the idiomatic default, not a
smell.

A large inline test module can be split into its own file while keeping
private access:

```rust
// lib.rs
#[cfg(test)]
mod tests;   // -> src/tests.rs
```

### Running tests

```bash
cargo test                        # all packages in the workspace
cargo test -p hellolib            # just this package
cargo test -p hellolib it_works   # filter by test name
cargo test -p hellolib -- --nocapture   # show println! output
```
