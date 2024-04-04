use rocket::serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    id: u64,
    title: String,
    content: String,
}

impl Task {
    pub fn new(id: u64, title: String, content: String) -> Self {
        Task { id, title, content }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
