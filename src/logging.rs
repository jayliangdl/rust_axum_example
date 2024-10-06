// use tracing_subscriber::filter::LevelFilter;
// use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing::subscriber;
use tokio::fs::OpenOptions;
use std::io;


pub async fn init_log(log_level:String) {
    let console_env_filter = EnvFilter::new(log_level.clone());
    let file_env_filter = EnvFilter::new(log_level);

    // 创建一个控制台日志记录
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(false)
        // .with_filter(LevelFilter::INFO);
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
        // .with_filter(LevelFilter::INFO);
        .with_filter(file_env_filter);

    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber);

    // 设置全局日志记录
    subscriber::set_global_default(subscriber).expect("Could not set global default");

}