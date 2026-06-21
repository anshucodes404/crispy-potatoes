# Student Management — CLI (Rust)

A small, polished command-line student management system written in Rust. It provides a friendly interactive menu to add, remove and search student records with input validation and clear output formatting.

--

Table of Contents
- [Why this project](#why-this-project)
- [Features](#features)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [Project Structure](#project-structure)
- [Internals & Design](#internals--design)
- [Extending & Persistence](#extending--persistence)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Why this project

This repository is ideal for Rust learners who want a practical CLI project demonstrating:

- Ownership and borrowing patterns
- Enum and pattern matching usage
- User input parsing and validation
- Working with collections and simple search algorithms

## Features

- Add student records with: `id` (u128), `name`, `class` (1–10), `phone` (10 digits)
- Delete student records by `id` and `class`
- Search students by name (partial match), by class, or by id + class
- Robust input validation with helpful error messages
- Pure in-memory storage (simple and easy to extend)

## Quick Start

Prerequisites

- Rust toolchain (rustup) and Cargo

Build and run (debug):

```bash
cargo build
cargo run
```

Build and run (release):

```bash
cargo build --release
./target/release/student_management
```

Tip: `cargo run` is the easiest way to try the app while developing.

## Usage Examples

When you run the program, the interactive menu appears:

```
========== USER PORTAL ==========

SELECT THE OPTION TO CONTINUE
1. ADD A STUDENT DETAIL
2. DELETE A STUDENT DETAIL
3. GET A STUDENT INFO
q. QUIT
```

Add a student (example flow):

```
Select: 1
ID: 101
NAME: John Doe
CLASS: 8
PHONE: 9876543210
```

Search by name (partial match):

```
Select: 3
Search option: 1
Enter name fragment: John

Results:
 - 101 | John Doe | Class 8 | 9876543210
```

Delete a student:

```
Select: 2
ID: 101
CLASS: 8
Student removed successfully.
```

## Project Structure

```
student_management/
├── Cargo.toml
└── src
      └── main.rs
```

All logic currently lives in `src/main.rs` for simplicity. Splitting into modules is straightforward when the project grows.

## Internals & Design

- `Student` struct: stores `id: u128`, `name: String`, `class: Class`, `phone: String`.
- `Class` enum: represents class levels 1..=10 and a fallback `OTHER`.
- Storage: `Vec<Student>` kept in memory for the lifetime of the program.
- Validation utilities: `is_phone_valid()`, `map_to_class()` and typed readers for safe input parsing.

## Extending & Persistence

Suggestions to evolve the app:

- Persist data to disk (JSON/CSV) using `serde` + `serde_json` or `csv` crates
- Use `HashMap` indexes to speed up lookups by `id`
- Add an `edit` command to update records
- Add command-line flags (with `clap`) to support batch import/export

## Testing

Add unit tests to `src/main.rs` using `#[cfg(test)]` blocks. Example:

```rust
#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn phone_validation() {
            assert!(is_phone_valid("9876543210"));
            assert!(!is_phone_valid("12345"));
      }
}
```

Run tests:

```bash
cargo test
```

## Contributing

Contributions are welcome. A good workflow:

1. Fork the repository
2. Create a feature branch
3. Open a pull request with a clear description

If you propose changes to the data model or storage format, include migration guidance.

## License

This project is provided for educational purposes. Feel free to reuse or adapt the code — add a license file if you plan to publish or distribute commercially.

---

If you want, I can also:

- Add a short example `students.json` and implement saving/loading using `serde`
- Split the code in `src/lib.rs` + `src/main.rs` and add unit tests
- Create a small CONTRIBUTING.md and CODE_OF_CONDUCT

Would you like me to make any of those improvements now?
