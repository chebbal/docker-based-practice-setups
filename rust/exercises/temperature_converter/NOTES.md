# temperature_converter — learning notes

A small CLI used to practice Rust concepts. Each milestone gets a git commit (and optional tag).

## Current state

- **Version:** `0.1.0`
- **Tag:** `v0.1-cli-basics` (suggested)
- **Run:** `cargo run -- 32` or `cargo run -- 100 --to-farenheit`

## Milestones

### v0.1 — CLI basics (current)

**Concepts**

- `cargo new`, project layout (`Cargo.toml`, `src/main.rs`)
- `std::env::args` for CLI arguments
- `Vec<String>` and indexing (`args[1]`)
- `parse()` returns `Result` — used `unwrap()` for now
- Optional flag via `args.contains(&"--to-farenheit".to_string())`
- `const` for conversion factors
- Basic functions and `f64` arithmetic

**Usage**

```bash
cargo run -- 32                  # Fahrenheit → Celsius
cargo run -- 100 --to-farenheit  # Celsius → Fahrenheit
```

**Open questions / follow-ups**

- Replace `unwrap()` with proper `Result` handling
- Use `args.iter().any(|arg| arg == "--to-farenheit")` to avoid allocating a `String`
- Flag order is fixed: temperature must be `args[1]`

---

### v0.2 — Error handling (planned)

- [ ] Replace `unwrap()` with `match` or `?`
- [ ] Friendly error when input is not a number
- [ ] Custom error type or `anyhow` / `thiserror`

### v0.3 — CLI parsing (planned)

- [ ] `clap` derive for args and `--help`
- [ ] Fix spelling: `farenheit` → `fahrenheit`

### v0.4 — Structure & tests (planned)

- [ ] Split into `lib.rs` + `main.rs`
- [ ] Unit tests for `faren_2_celsius` / `celsius_2_faren`
- [ ] Integration tests for CLI

### Later experiments

- Traits (`Display`, custom `Temperature` type)
- `examples/` for isolated concept demos
- `CARGO_PKG_VERSION` and `--version` flag

## Session log

| Date       | Focus              | Notes                                      |
|------------|--------------------|--------------------------------------------|
| 2026-06-15 | Project setup, CLI | `cargo new`, args parsing, `unwrap`, flags |
