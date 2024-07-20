use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProductFilter {
    pub product_id: Option<i32>,
    pub user_id: Option<i32>, // 新增字段 user_id
    pub name: Option<String>,
    pub description: Option<String>,     // 新增字段 description
    pub sku: Option<String>,             // 新增字段 sku
    pub image: Option<String>,           // 新增字段 image
    pub price_min: Option<f64>,          // 新增字段 price_min
    pub price_max: Option<f64>,          // 新增字段 price_max
    pub discount_price_min: Option<f64>, // 新增字段 discount_price_min
    pub discount_price_max: Option<f64>, // 新增字段 discount_price_max
    pub is_discounted: Option<bool>,     // 新增字段 is_discounted
    pub is_valid: Option<bool>,          // 新增字段 is_valid
    pub stock_quantity_min: Option<i32>, // 新增字段 stock_quantity_min
    pub stock_quantity_max: Option<i32>, // 新增字段 stock_quantity_max
    pub is_in_stock: Option<bool>,       // 新增字段 is_in_stock
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub supplier_id: Option<i32>,   // 新增字段 supplier_id
    pub weight_min: Option<f64>,    // 新增字段 weight_min
    pub weight_max: Option<f64>,    // 新增字段 weight_max
    pub dimensions: Option<String>, // 新增字段 dimensions
    pub status: Option<String>,     // 新增字段 status
    pub public: Option<bool>,       // 新增字段 public
}
