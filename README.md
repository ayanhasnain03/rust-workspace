
cargo new --vcs none blog_api

Creates a new Rust binary crate named blog_api.
The --vcs none flag skips initializing a version control system (like Git). This is useful when:



cargo new --vcs none blog_web

Similar to the above, but creates a Rust binary crate named blog_web.



cargo new --vcs none --lib blog_shared

Creates a Rust library crate named blog_shared.
Libraries are used for code meant to be reused by other crates (e.g., Post struct).
Result: A folder structure for blog_shared:


