extern crate rust_axum_example;
use rust_axum_example::nacos;
use rust_axum_example::nacos::start_nacos_watch;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, routing::{get, post}, BoxError, Extension, Router
};

use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tracing::{info,error};
use std::env; 
use dotenv::dotenv;

use rust_axum_example::logging::init_log;
use rust_axum_example::db::init_pool;

use rust_axum_example::handlers::{
    health_check::{health_check,env_variable},    
    mock_timeout::mock_timeout,
    create_user::create_user,
    operation_sku::{create_sku,update_sku,find_sku},
    frontend_sku::find_sku as front_find_sku,
    client_sku::find_sku as client_find_sku,
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
    
    //获取端口号，如果没有设置则默认为3000
    let port = env::var("PORT_SET_WHEN_RUN").unwrap_or_else(|_| env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    // let port = "3002".to_string();
    let port_num = port.parse::<u16>().expect("PORT must be a number");//端口号转换为u16类型

    let pool = init_pool(&database_url).await.expect("Cannot init the database pool");//连接池初始化
    info!("Started processing request");

    // Nacos 配置
    let nacos_url = env::var("NACOS_URL").unwrap_or_else(|_| "http://localhost:8848".to_string());
    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "my_rust_service".to_string());
    let group_name = env::var("GROUP_NAME").unwrap_or_else(|_| "DEFAULT_GROUP".to_string());
    let namespace_id = env::var("NAMESPACE_ID").unwrap_or_else(|_| "public".to_string());
    let ip = env::var("SERVICE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());

    // 启动 Nacos 管理器
    let nacos_handle = nacos::start_nacos(
        &nacos_url,
        &service_name,
        &group_name,
        &namespace_id,
        &ip,
        port_num,
    ).await.expect("Failed to start Nacos manager");

    tokio::time::sleep(Duration::from_secs(5)).await;
    // nacos_url: &str, service_name: &str, group_name: &str, namespace_id: &str
    let _ = start_nacos_watch(&nacos_url, &group_name, &namespace_id).await;
    // //sleep 5s
    // tokio::time::sleep(Duration::from_secs(5)).await;
    // // 获取服务实例
    // let _ = get_service_instances(&nacos_url, &service_name, &group_name, &namespace_id).await;


    // 定义优雅关闭信号，监听 SIGINT 和 SIGTERM
    let shutdown_signal = async {
        // 创建一个新的信号监听器，监听 SIGINT 和 SIGTERM
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");
            let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
            tokio::select! {
                _ = sigint.recv() => {
                    info!("Received SIGINT");
                }
                _ = sigterm.recv() => {
                    info!("Received SIGTERM");
                }
            }
        }
        #[cfg(not(unix))]
        {
            // 对于非 Unix 系统，仅监听 Ctrl+C
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
            info!("Received Ctrl+C");
        }

        // 发送注销信号给 Nacos
        if let Err(e) = nacos_handle.shutdown.send(()) {
            error!("Failed to send shutdown signal to Nacos: {:?}", e);
        }
    };


    let app = Router::new()
    .route("/health_check", get(health_check))
    .route("/env_variable", get(env_variable))
    .route("/mock_timeout", post(mock_timeout))
    .route("/create_user", post(create_user))
    .route("/operation/create_sku", post(create_sku))
    .route("/operation/update_sku", post(update_sku))
    .route("/operation/find_sku", post(find_sku))
    .route("/frontend/find_sku", post(front_find_sku))
    .route("/client/find_sku", post(client_find_sku))
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

    

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Started server at {}",port);
    axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal)
    .await
    .unwrap();

    // 等待 Nacos 任务完成
    if let Err(e) = nacos_handle.join_handle.await {
        error!("Nacos task failed: {:?}", e);
    }
    info!("Application shutdown complete");

    
}

// // 定义优雅关闭的信号处理
// #[instrument(name = "shutdown_signal", fields(request_id = %Uuid::new_v4()))]
// #[allow(dead_code)]
// async fn shutdown_signal() {
//     // 等待 Ctrl+C 信号
//     tokio::signal::ctrl_c()
//         .await
//         .expect("Failed to install Ctrl+C handler");
//     info!("Shutdown signal received");
// }

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







