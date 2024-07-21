use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ShipmentFilter {
    pub shipment_id: Option<i32>,
    pub order_id: Option<i32>,                    // 订单 ID
    pub shipment_date_min: Option<NaiveDateTime>, // 最早发货日期
    pub shipment_date_max: Option<NaiveDateTime>, // 最晚发货日期
    pub delivery_address: Option<String>,         // 收货地址
    pub status: Option<String>,                   // 发货状态
}
