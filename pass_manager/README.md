# Pass Manager

A simple terminal-based password manager written in Rust. It connects to a PostgreSQL database, lets you store credentials, search them by website or username, list everything, and delete entries from the command line.

## What It Does

- Add a new credential with website name, username, and password.
- Search stored credentials by website name.
- Search stored credentials by username.
- Search stored credentials by website name and username together.
- List all saved credentials.
- Delete a credential by ID.

## Prerequisites

Before running the app locally, make sure you have:

- Rust installed via [rustup](https://rustup.rs/)
- PostgreSQL running locally or remotely
- A database and user that can create and read the `credentials` table

## Project Structure

```text
pass_manager/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── model.rs
│   ├── repository.rs
│   └── ui.rs
└── README.md
```

## Database Setup

The application expects a PostgreSQL database with a table named `credentials`.

Create the table with this SQL:

```sql
CREATE TABLE IF NOT EXISTS credentials (
    id BIGSERIAL PRIMARY KEY,
    website_name TEXT NOT NULL,
    user_name TEXT NOT NULL,
    password TEXT NOT NULL,
    UNIQUE (website_name, user_name)
);
```

## Local Setup

1. Clone the repository.
2. Create a PostgreSQL database if you do not already have one.
3. Run the SQL above to create the `credentials` table.
4. Create a `.env` file in the project root.
5. Add your database connection string to `.env`.

Example `.env`:

```env
DATABASE_URL=postgres://postgres:your_password@localhost:5432/pass_manager
```

## Run It

From the project root, start the application with:

```bash
cargo run
```

If the database connection is correct, the app will print a success message and open the main menu.

## How To Use

After startup, you will see a menu like this:

```text
===== Password Manager =====
1. Add Credential
2. Search Credential
3. List All Credentials
4. Delete Credential
5. Generate Password
q. Exit
============================
```

Choose an option and follow the prompts.

- `1` adds a new credential.
- `2` opens the search menu.
- `3` prints all saved credentials.
- `4` deletes a credential by ID.
- `q` exits the app.

### Search Menu

Option `2` opens a second menu:

```text
========== SEARCH MENU ==========
1. Search from Website Name
2. Search from User Name
3. Search from Website and User Name
```

## Notes

- The app reads `DATABASE_URL` from `.env` at startup, so the file must exist before you run it.
- Input is trimmed before it is used, which helps avoid accidental whitespace issues.
- The menu currently shows `Generate Password`, but that option is not wired into the command loop yet.

## Troubleshooting

- If you see `Database URI not found in .env file`, check that `.env` exists in the project root and that `DATABASE_URL` is set correctly.
- If you see a PostgreSQL connection error, verify the server is running, the host and port are correct, and the database user has access to the target database.
- If searches return no rows, confirm that your records were inserted into the same database instance you are connecting to.

## License

No license file is currently included in this repository.