use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

use tracing::subscriber;
use tokio::fs::OpenOptions;
use std::io;
use time::macros::offset;
use tracing_subscriber::fmt::time::OffsetTime;
use dotenv::dotenv;
use std::env;
use std::sync::Once;

pub async fn init_log() {
    dotenv().ok();
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_|"info".to_string());
    tracing::info!("log_level:{}", log_level);
    std::env::set_var("TZ", "Asia/Shanghai");
    let console_env_filter = EnvFilter::new(log_level.clone());
    let file_env_filter = EnvFilter::new(log_level);
    // let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    let utc_plus_8 = offset!(+08:00); // 使用 time 宏定义 UTC+8 偏移
    let offset_time = OffsetTime::new(utc_plus_8, time::format_description::well_known::Rfc3339);

    // 创建一个控制台日志记录
    let console_subscriber = tracing_subscriber::fmt::layer()
    .with_writer(io::stdout)
    .with_ansi(false)
    .with_file(true)
    .with_line_number(true)
    // .with_filter(LevelFilter::INFO);
    .with_timer(offset_time.clone())
    .with_filter(console_env_filter);

    // 使用 OpenOptions 创建文件以写入日志，设置为追加模式
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("app.log")
        .await
        .expect("Could not open log file").try_into_std().unwrap();

    // 创建一个文件日志记录
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(log_file)        
        .with_ansi(false)
        .with_line_number(true)
        .with_file(true)
        // .with_filter(LevelFilter::INFO);
        .with_timer(offset_time)
        .with_filter(file_env_filter);

    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber);

    static INIT: Once = Once::new();
    INIT.call_once(||{
        subscriber::set_global_default(subscriber).expect("Could not set global default");
    });
    // 设置全局日志记录

}