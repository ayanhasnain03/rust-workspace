
cargo new --vcs none blog_api

Creates a new Rust binary crate named blog_api.
The --vcs none flag skips initializing a version control system (like Git). This is useful when:
You’re adding the crate to an existing repository.
You’re working in a workspace where version control is already managed at the root.
Result: A folder structure for blog_api like this:


blog_api/
├── Cargo.toml
└── src/
    └── main.rs
cargo new --vcs none blog_web

Similar to the above, but creates a Rust binary crate named blog_web.
Result: A folder structure for blog_web:


blog_web/
├── Cargo.toml
└── src/
    └── main.rs
cargo new --vcs none --lib blog_shared

Creates a Rust library crate named blog_shared.
Libraries are used for code meant to be reused by other crates (e.g., Post struct).
Result: A folder structure for blog_shared:

css
Copy code
blog_shared/
├── Cargo.toml
└── src/
    └── lib.rs
--vcs none Explained
By default, cargo new initializes a Git repository (git init) and adds a .gitignore file tailored for Rust projects. The --vcs none flag disables this behavior, so:

No Git repository is initialized.
No .gitignore file is created.
Adding a README.md File for blog_web
To document the purpose and usage of blog_web, create a README.md file in its root directory.

blog_web/README.md:


# Blog Web

This is the web frontend of the blog application.

## Features
- User interface for interacting with the blog API.
- Displays posts and other blog content.
- Written in Rust for performance and reliability.
