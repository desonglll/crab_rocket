use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskInfo {
    pub count: i64,
}

impl TaskInfo {
    pub fn new_empty() -> Self { TaskInfo { count: -1 } }
}
