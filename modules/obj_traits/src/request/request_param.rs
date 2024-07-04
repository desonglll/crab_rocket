use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RequestParam<P> {
    pub pagination: P,
}

impl<P> RequestParam<P> {
    pub fn new(pagination: P) -> Self {
        Self { pagination }
    }
}
