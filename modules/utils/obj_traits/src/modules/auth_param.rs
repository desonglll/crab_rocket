use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct AuthParam {
    pub session_id: Uuid,
}

impl AuthParam {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
        }
    }
}
