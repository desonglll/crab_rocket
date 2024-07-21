use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct OrderFilter {
    pub order_id: Option<i32>,
    pub customer_id: Option<i32>,
    pub order_date_min: Option<NaiveDateTime>,
    pub order_date_max: Option<NaiveDateTime>,
    pub total_amount_min: Option<f64>,
    pub total_amount_max: Option<f64>,
    pub status: Option<String>,
}
