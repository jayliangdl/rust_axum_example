use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, routing::{get, post}, BoxError, Extension, Router
};

use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;
use tracing::info;
use uuid::Uuid;
mod logging;
mod db;
mod handlers;
mod model;
mod models;
mod dao;
mod error;
mod cache;
use std::env; 
use dotenv::dotenv;

use logging::init_log;
use db::init_pool;

use handlers::{
    health_check::health_check,
    mock_timeout::mock_timeout,
    create_user::create_user,
    operation_sku::{create_sku,update_sku,find_sku},
    frontend_sku::find_sku as front_find_sku,
};


#[tokio::main]
async fn main() {    
    // 加载环墶变量
    dotenv().ok();  

    // 从环境变量中获取日志级别，如果没有设置则默认为info
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_|"info".to_string());
    // let log_level = "info".to_string();
    info!("log_level:{}",log_level);
    init_log(log_level).await;//日志初始化
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");//获取数据库连接地址
    // let database_url = "mysql://root:root@localhost:3306/rust_sqlx_example".to_string();
    
    let pool = init_pool(&database_url).await.expect("Cannot init the database pool");//连接池初始化
    info!("Started processing request");
    let app = Router::new()
    .route("/health_check", get(health_check))
    .route("/mock_timeout", post(mock_timeout))
    .route("/create_user", post(create_user))
    .route("/operation/create_sku", post(create_sku))
    .route("/operation/update_sku", post(update_sku))
    .route("/operation/find_sku", post(find_sku))
    .route("/frontend/find_sku", post(front_find_sku))
    .layer(TraceLayer::new_for_http())// 添加日志记录中间件
    .layer(ConcurrencyLimitLayer::new(100)) // 限制并发请求数量
    .layer(
        ServiceBuilder::new()
        // timeout will produce an error if the handler takes
        // too long so we must handle those
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .timeout(Duration::from_secs(3))// 设置超时时间
    )
    .layer(Extension(pool)); // 将连接池添加到应用程序中

    //获取端口号，如果没有设置则默认为3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // let port = "3000".to_string();

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Started server at {}",port);
    axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
    
}

// 定义优雅关闭的信号处理
async fn shutdown_signal() {
    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    // info!("Shutdown signal received");
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}







