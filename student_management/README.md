# Student Management System

A command-line student management system built in Rust, providing an interactive interface to manage student records with comprehensive search and CRUD operations.

## Overview

This project implements a simple yet effective student management application that allows educators and administrators to:
- **Add** new student records with personal information
- **Delete** existing student records
- **Search** for students using multiple criteria (name, class, or student ID)
- **View** student details in a formatted output

The application uses an in-memory vector-based storage system and provides a user-friendly CLI interface with input validation.

## Features

### Core Functionality

- **Add Student**: Create new student records with the following details:
  - Unique Student ID (u128)
  - Student Name (String)
  - Class Level (I-X)
  - Phone Number (10-digit validation)

- **Delete Student**: Remove student records by specifying:
  - Student ID
  - Class Level (to ensure accurate deletion)

- **Search Student**: Find student records using three search methods:
  - **By Name**: Search using partial or full student name (case-sensitive)
  - **By Class**: Filter all students in a specific class level
  - **By ID and Class**: Find a specific student using ID and class combination

### Input Validation

- **Phone Number Validation**: Ensures phone numbers are exactly 10 digits
- **Class Validation**: Restricts class input to levels I-X (1-10)
- **ID Parsing**: Validates numeric input for student IDs
- **Error Handling**: Graceful error messages for invalid inputs

### Data Structure

- **Student Struct**: Contains all essential student information
  ```rust
  struct Student {
      id: u128,
      name: String,
      class: Class,
      phone: String,
  }
  ```

- **Class Enum**: Represents class levels (I through X, with OTHER fallback)

## Installation

### Prerequisites

- Rust 1.70+ (toolchain with edition 2024)
- Cargo (comes with Rust)

### Setup

1. Clone or download the project:
```bash
cd student_management
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## Usage

### Running the Application

Start the program:
```bash
cargo run
```

### Interactive Menu

Upon starting, you'll see the main menu:

```
========== USER PORTAL ==========

SELECT THE OPTION TO CONTINUE
1. ADD A STUDENT DETAIL
2. DELETE A STUDENT DETAIL
3. GET A STUDENT INFO
q. QUIT
```

### Menu Options

#### Option 1: Add a Student Detail

1. Select option `1` from the main menu
2. Enter the following information when prompted:
   - **ID**: A unique identifier (any valid 128-bit unsigned integer)
   - **NAME**: Student's full name
   - **CLASS**: Class level (1-10 corresponding to classes I-X)
   - **PHONE**: 10-digit phone number

Example:
```
ID: 101
NAME: John Doe
CLASS: 8
PHONE: 9876543210
```

#### Option 2: Delete a Student Detail

1. Select option `2` from the main menu
2. Enter the student information to identify which record to delete:
   - **ID**: Student's ID
   - **CLASS**: Student's class level

Example:
```
ID: 101
CLASS: 8
```

#### Option 3: Get Student Information

1. Select option `3` from the main menu
2. Choose your search method from the search menu:

```
========== SEARCH STUDENT ==========

1. SEARCH FROM NAME
2. SEARCH FROM CLASS
3. SEARCH FROM ID
```

**Search by Name:**
- Enter a partial or full student name
- Returns all students whose names contain the search string

**Search by Class:**
- Enter a class level (1-10)
- Returns all students in that class

**Search by ID:**
- Enter student ID and class level
- Returns the specific student record if found

#### Option q: Quit

Select `q` to exit the program.

## Project Structure

```
student_management/
├── Cargo.toml              # Project manifest with metadata and dependencies
├── README.md               # This file
└── src/
    └── main.rs             # Main source code
```

## Code Architecture

### Core Functions

- **`add_student()`**: Adds a new student to the collection
- **`remove_student()`**: Removes a student by ID and class
- **`search_student_from_name()`**: Searches students by name (partial match)
- **`search_student_from_class()`**: Retrieves all students in a specific class
- **`search_student_from_id_and_class()`**: Finds specific student by ID and class
- **`map_to_class()`**: Converts string input to Class enum
- **`is_phone_valid()`**: Validates phone number length
- **`read_input()`**: Reads user input with custom prompt
- **`read_id()`, `read_name()`, `read_class()`, `read_phone()`**: Type-specific input readers with validation

## Data Persistence

**Note**: This application stores data in memory only. All data is lost when the program exits. For persistent storage, consider:
- Implementing file I/O (JSON/CSV serialization)
- Using a database backend (SQLite, PostgreSQL, etc.)
- Serializing with `serde` crate

## Build Variants

### Debug Build
```bash
cargo build
./target/debug/student_management
```

### Release Build (Optimized)
```bash
cargo build --release
./target/release/student_management
```

## Testing

The project includes a `Student` struct and `Class` enum derived with `#[derive(PartialEq, Debug)]` for easier testing and debugging.

To add unit tests, modify `src/main.rs` with `#[cfg(test)]` modules:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_validation() {
        assert!(is_phone_valid("9876543210"));
        assert!(!is_phone_valid("987654321"));
    }
}
```

Then run tests with:
```bash
cargo test
```

## Dependencies

Currently, this project has **no external dependencies**. It uses only Rust standard library:
- `std::io` - for input/output operations
- `std::fmt` - for formatting (via derive macros)

## Future Enhancements

Potential improvements for this project:

1. **Persistent Storage**
   - Implement file-based storage (JSON/CSV)
   - Add database support (SQLite)

2. **Additional Features**
   - Edit/update student records
   - Display all students with pagination
   - Export student records to file
   - Import student records from file

3. **Code Improvements**
   - Add comprehensive error handling with custom error types
   - Implement logging
   - Add unit and integration tests
   - Create more modular structure with separate modules

4. **User Interface**
   - Add colors and better formatting (using `colored` crate)
   - Implement table-based display of results
   - Add confirmation prompts for destructive operations

5. **Performance**
   - Implement indexing for faster searches
   - Use more efficient data structures (HashMap for O(1) lookups)

## Troubleshooting

### Invalid Class Error
- Ensure you enter class numbers 1-10 only
- Class levels correspond to: 1=I, 2=II, 3=III, ..., 10=X

### Invalid Phone Number Error
- Phone numbers must be exactly 10 digits
- No hyphens, spaces, or other characters

### Student Not Found
- Verify the exact spelling when searching by name (case-sensitive)
- For deletion and ID search, ensure both ID and class level match exactly

### Input Parsing Errors
- For ID input, enter only numeric values
- Avoid special characters and spaces unless appropriate

## License

This project is provided as-is for educational purposes.

## Author

Created as a Rust learning project for understanding:
- Rust ownership and borrowing
- Pattern matching and enums
- CLI input/output
- Vector operations and searching
- Error handling and validation

## Getting Help

For issues or questions:
1. Review the Usage section above
2. Check the Troubleshooting section
3. Examine the main.rs source code for implementation details
