use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::Notify;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, BoxError, Extension, 
};
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use tracing::{info, error};
use std::env;
use dotenv::dotenv;
use std::sync::atomic::{AtomicUsize, Ordering};

use rust_axum_example::utils::load_balance::fetch_load_balance_from_nacos;
use rust_axum_example::utils::nacos;
use rust_axum_example::utils::logging::init_log;
use rust_axum_example::utils::db::init_pool;
use rust_axum_example::utils::request_counter::request_counter_middleware;
use rust_axum_example::utils::request_loging::print_request_response;
use rust_axum_example::routes::app_router;

#[tokio::main]
async fn main() {    
    // 加载环境变量
    dotenv().ok();  
    // 获取日志级别并初始化日志
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_|"info".to_string());
    info!("log_level:{}", log_level);
    init_log(log_level).await; // 日志初始化
    
    // 获取数据库连接地址并初始化连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = init_pool(&database_url).await.expect("Cannot init the database pool");

    // Nacos 配置
    let nacos_url = env::var("NACOS_URL").unwrap_or_else(|_| "http://localhost:8848".to_string());
    let service_name = env::var("SERVICE_NAME_WHEN_RUN")
        .unwrap_or_else(|_| env::var("SERVICE_NAME").unwrap_or_else(|_| "my_rust_service".to_string()));
    let group_name = env::var("GROUP_NAME").unwrap_or_else(|_| "DEFAULT_GROUP".to_string());
    let namespace_id = env::var("NAMESPACE_ID").unwrap_or_else(|_| "public".to_string());
    let ip = env::var("SERVICE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT_SET_WHEN_RUN")
        .unwrap_or_else(|_| env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    let port_num = port.parse::<u16>().expect("PORT must be a number");


    // 从 Nacos 中获取其他微服务的实例列表，并添加到缓存中（每3秒从 Nacos 中刷新一次）
    let _ = fetch_load_balance_from_nacos().await;

    // 创建一个 Notify 实例，用于通知主任务服务器已准备好
    let notify_1 = Arc::new(Notify::new());
    let notify_2 = notify_1.clone();

    // 创建一个 Notify 实例，用于通知服务器关闭
    let shutdown_notify_1 = Arc::new(Notify::new());
    let shutdown_notify_2 = shutdown_notify_1.clone();

    // 创建一个全局的请求计数器
    let request_counter = Arc::new(AtomicUsize::new(0));
    let is_shutting_down = Arc::new(AtomicBool::new(false));

    // 打印正在处理的请求数量
    // let request_counter_clone = Arc::clone(&request_counter);
    // tokio::spawn(log_request_count(port_num.clone(),request_counter_clone));

    // 创建 API 路由
    let app = app_router()
        .layer(TraceLayer::new_for_http()) // 添加日志记录中间件
        .layer(ConcurrencyLimitLayer::new(100)) // 限制并发请求数量
        .layer(
            ServiceBuilder::new()
                // timeout will produce an error if the handler takes too long so we must handle those
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(3)) // 设置超时时间
        )        
        .layer(
            ServiceBuilder::new()
                // 添加请求计数器中间件
                .layer(Extension(request_counter.clone())) // 将计数器添加到 Extension 中
                .layer(Extension(is_shutting_down.clone())) // 将计数器添加到 Extension 中
                .layer(axum::middleware::from_fn(request_counter_middleware))
        )
        .layer(
            ServiceBuilder::new()
                // 添加打印请求和响应日志的中间件
                .layer(axum::middleware::from_fn(print_request_response))
        )

    //     .layer(
    // ServiceBuilder::new().layer(axum::middleware::from_fn(
    //     {
    //         let request_counter = request_counter.clone();
    //         move |req: Request<axum::body::Body>, next: Next<>| {
    //             let counter = request_counter.clone();
    //             async move {
    //                 // 请求进入时，增加计数器
    //                 counter.fetch_add(1, Ordering::SeqCst);
    //                 info!("Request count: {}", counter.load(Ordering::SeqCst));
    //                 let response = next.run(req).await;
    //                 // 请求完成时，减少计数器
    //                 counter.fetch_sub(1, Ordering::SeqCst);
    //                 response
    //             }
    //         }
    //     },
    //     )))
        .layer(Extension(pool)); // 将连接池添加到应用程序中

    

    // 绑定监听器
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Bound to {}", addr);

    // 启动 Web 服务器任务
    let server_handle = {
        tokio::spawn(async move {
            // 使用 axum::serve_with_shutdown 来启动服务器
            let server = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    shutdown_notify_2.notified().await
            });
    
            // 启动服务器前通知主任务
            notify_2.notify_one();
    
            // 启动服务器
            server.await.unwrap();
        })
    };

    // 等待服务器启动的通知
    notify_1.notified().await;
    info!("Server is up and running on {}", addr);

    // 向 Nacos 注册本应用实例
    let nacos_handle = nacos::start_nacos(
        &nacos_url,
        &service_name,
        &group_name,
        &namespace_id,
        &ip,
        port_num,
    ).await.expect("Failed to start Nacos manager");

    // 定义优雅关闭信号，监听 SIGINT 和 SIGTERM
    let shutdown_handle = tokio::spawn(async move {
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
        info!("本实例正在关闭");
        is_shutting_down.store(true, Ordering::SeqCst);  
        // 发送注销信号给 Nacos
        if let Err(e) = nacos_handle.shutdown.send(()) {
            error!("Failed to send shutdown signal to Nacos: {:?}", e);
        }

        // 等待 Nacos 任务完成
        if let Err(e) = nacos_handle.join_handle.await {
            error!("Nacos task failed: {:?}", e);
        }    
        
        // 等待所有请求完成
        loop{
            // 检查请求计数器是否为 0
            if request_counter.load(Ordering::SeqCst) == 0 {
                info!("All requests are completed");
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
                   
        // 发送关闭信号给服务器
        shutdown_notify_1.notify_one();
    });

    // 等待服务器任务和关闭任务完成
    let _ = tokio::join!(server_handle, shutdown_handle);

    info!("Application shutdown complete");
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

#[allow(dead_code)]
async fn log_request_count(port: u16, request_counter_clone:Arc<AtomicUsize>){
    loop{
        error!("port: {}, Request count: {}", port, request_counter_clone.load(Ordering::SeqCst));
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
