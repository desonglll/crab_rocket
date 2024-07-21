use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InventoryFilter {
    pub inventory_id: Option<i32>,
    pub product_id: Option<i32>, // 根据之前表的定义，可能需要这个字段来过滤
    pub location: Option<String>, // 假设有个 location 字段用来过滤
    pub quantity_min: Option<i32>, // 对应 inventory 表中的 quantity 字段
    pub quantity_max: Option<i32>, // 对应 inventory 表中的 quantity 字段
    pub last_updated_min: Option<NaiveDateTime>,
    pub last_updated_max: Option<NaiveDateTime>,
}
