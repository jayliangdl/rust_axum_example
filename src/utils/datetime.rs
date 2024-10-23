use chrono::{TimeZone, Utc, Local};
use chrono_tz::Asia::Shanghai; // 使用北京时间

pub fn now_local() -> chrono::NaiveDateTime {
    // 获取当前UTC时间
    let now: chrono::DateTime<Utc> = Utc::now();
    
    // 将UTC时间转换为北京时间
    let beijing_time: chrono::DateTime<chrono_tz::Tz> = Shanghai.from_utc_datetime(&now.naive_utc());
    
    // 如果你确实需要 DateTime<Local> 类型，可以这样转换
    // 注意：这实际上会根据系统的本地时区进行转换，而不是保持北京时间。
    // 通常情况下，我们不建议这样做，因为这可能会导致时区信息丢失。
    let local_time: chrono::DateTime<Local> = beijing_time.with_timezone(&Local);
    
    return local_time.naive_local();
}

#[cfg(test)]
mod test{
    use crate::utils::datetime::now_local;
    #[test]
    fn test1(){        
        // 打印北京时间
        println!("当前北京时间是: {}", now_local().format("%Y-%m-%d %H:%M:%S").to_string());
    }
}