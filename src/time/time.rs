use std::env;
use std::fmt::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::r#type::from_env_var;
/// 闰年
static LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// 平年
static COMMON_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn get_now_time() -> String {
    // 获取当前时间
    let start: SystemTime = SystemTime::now();
    let duration: Duration = start.duration_since(UNIX_EPOCH).unwrap();
    let total_seconds: u64 = duration.as_secs();
    let mut total_days: u64 = total_seconds / 86400;
    // 计算年份，考虑中间年份的闰年
    let mut year: u64 = 1970;
    while total_days >= if is_leap_year(year) { 366 } else { 365 } {
        total_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }
    // 计算月份和日期
    let mut month: i32 = 1;
    let month_days: [u64; 12] = if is_leap_year(year) {
        LEAP_YEAR
    } else {
        COMMON_YEAR
    };
    // 计算当前月份
    while total_days >= month_days[month as usize - 1] {
        total_days -= month_days[month as usize - 1];
        month += 1;
    }
    // 当前日期
    let day: u64 = total_days + 1;
    // 计算剩余小时
    let remaining_seconds: u64 = total_seconds % 86400;
    let timezone_offset: u64 = from_env_var().value();
    let hours: u64 = ((remaining_seconds + timezone_offset) / 3600) % 24;
    let minutes: u64 = (remaining_seconds % 3600) / 60;
    let seconds: u64 = remaining_seconds % 60;
    // 创建日期字符串
    let mut date_time: String = String::new();
    write!(
        &mut date_time,
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
    .unwrap_or_default();
    date_time
}

/// 判断是否为闰年
///
/// # 参数
/// `u64`: 年份
/// # 返回值
/// `bool`: 是否是闰年
fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
