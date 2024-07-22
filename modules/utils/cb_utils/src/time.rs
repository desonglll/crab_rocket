/// # 获取当前北京时间（东八区）
///
/// 此函数获取当前世界时间（UTC），并将其转换为北京时间（东八区）。
///
/// ## 返回值
///
/// 返回当前的北京时间，类型为 `chrono::prelude::NaiveDateTime`。
///
/// ## 示例
///
/// ```
/// use crab_rocket_utils::time::get_e8_time;
/// let bj_time = get_e8_time();
/// println!("当前北京时间: {}", bj_time);
/// ```
///
/// ## 错误处理
///
/// 如果时区偏移量无效，会触发 `expect` 宏并引发恐慌（panic）。
pub fn get_e8_time() -> chrono::prelude::NaiveDateTime {
    // 世界时间与 北京时间的转换
    let dt = chrono::Utc::now();
    let fixed_dt = dt
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).expect("Invalid offset for east"));
    fixed_dt.naive_local()
}
