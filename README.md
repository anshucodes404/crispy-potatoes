# Rust Projects Workspace

This workspace contains a few small Rust crates:

- `file_organiser` - a utility crate for organizing files.
- `pass_manager` - a password manager crate that uses `serde` and `serde_json`.
- `student_management` - a student management crate.

## Structure

Each project is an independent Cargo crate with its own `Cargo.toml` and `src/main.rs` entry point.

## Getting Started

Run a crate from its directory with Cargo:

```bash
cd file_organiser
cargo run
```

Replace `file_organiser` with `pass_manager` or `student_management` as needed.