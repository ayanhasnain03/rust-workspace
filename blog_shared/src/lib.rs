use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] // Added Debug trait
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }
}
