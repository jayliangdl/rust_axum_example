extern crate rust_axum_example;
use rust_axum_example::utils::nacos;
use rust_axum_example::utils::load_balance::fetch_load_balance_from_nacos;
// use rust_axum_example::utils::nacos::start_nacos_watch;
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

use rust_axum_example::utils::logging::init_log;
use rust_axum_example::utils::db::init_pool;

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
    // 加载环境变量
    dotenv().ok();  

    // 从环境变量中获取日志级别，如果没有设置则默认为info
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_|"info".to_string());
    info!("log_level:{}", log_level);
    init_log(log_level).await; // 日志初始化
    
    // 获取数据库连接地址
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let pool = init_pool(&database_url).await.expect("Cannot init the database pool"); // 连接池初始化
    info!("Started processing request");

    // 获取端口号，如果没有设置则默认为3000
    let port = env::var("PORT_SET_WHEN_RUN")
        .unwrap_or_else(|_| env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    let port_num = port.parse::<u16>().expect("PORT must be a number");
    
    // 定义应用程序路由和中间件
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
        .layer(TraceLayer::new_for_http()) // 添加日志记录中间件
        .layer(ConcurrencyLimitLayer::new(100)) // 限制并发请求数量
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(3)) // 设置超时时间
        )
        .layer(Extension(pool)); // 将连接池添加到应用程序中

    // 绑定监听器
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    // 启动服务器但不阻塞
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal());

    let load_server_load_balance = fetch_load_balance_from_nacos().await;

    // 在服务器启动之后再进行 Nacos 注册
    let nacos_handle = {
        // 获取 Nacos 配置
        let nacos_url = env::var("NACOS_URL").unwrap_or_else(|_| "http://localhost:8848".to_string());
        let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "my_rust_service".to_string());
        let group_name = env::var("GROUP_NAME").unwrap_or_else(|_| "DEFAULT_GROUP".to_string());
        let namespace_id = env::var("NAMESPACE_ID").unwrap_or_else(|_| "public".to_string());
        let ip = env::var("SERVICE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());

        // 启动 Nacos 管理器
        nacos::start_nacos(
            &nacos_url,
            &service_name,
            &group_name,
            &namespace_id,
            &ip,
            port_num,
        ).await.expect("Failed to start Nacos manager")
    };

    // 启动服务器
    tokio::select! {
        res = server => {
            if let Err(e) = res {
                error!("Server error: {:?}", e);
            }
        }
        _ = nacos_handle.join_handle => {
            error!("Nacos task failed");
        }

    }

    info!("Application shutdown complete");
}

// 定义优雅关闭信号
async fn shutdown_signal() {
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
    // 假设 nacos_handle 在某处可用
    // 需要确保这里能够访问 nacos_handle
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







