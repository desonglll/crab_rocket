pub fn get_e8_time() -> chrono::prelude::NaiveDateTime {
    // 世界时间与 北京时间的转换
    let dt = chrono::Utc::now();
    let fixed_dt = dt
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).expect("Invalid offset for east"));
    fixed_dt.naive_local()
}
