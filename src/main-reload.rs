use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::Html, routing::{get, post}, BoxError, Json, Router
};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tower::ServiceBuilder;
use rand::Rng;

use tracing_subscriber::filter::LevelFilter;
use tracing::{info, subscriber, trace,instrument};
use tokio::fs::OpenOptions;

use uuid::Uuid;
use std::io;




async fn init_log() {
    

    // 创建一个控制台日志记录
    let console_subscriber = tracing_subscriber::fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(false)
        .with_filter(LevelFilter::INFO);

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
        .with_filter(LevelFilter::INFO);

    let subscriber = tracing_subscriber::registry()
        .with(console_subscriber)
        .with(file_subscriber);

    // 设置全局日志记录
    subscriber::set_global_default(subscriber).expect("Could not set global default");

}


#[tokio::main]
async fn main() {    

    init_log().await;//日志初始化
    info!("Started processing request");
    trace!("Started processing request--trace");
    let app = Router::new()
        .route("/", get(index))
        .route("/mock_timeout", post(mock_timeout))
        .route("/create_user", post(create_user))
        .route("/set_level", post(set_level)) // 添加设置日志级别的路由
        .layer(TraceLayer::new_for_http())// 添加日志记录中间件
        .layer(ConcurrencyLimitLayer::new(100)) // 限制并发请求数量
        .layer(
        ServiceBuilder::new()
            // timeout will produce an error if the handler takes
            // too long so we must handle those
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(3))// 设置超时时间
        );

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

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[instrument(name = "mock_timeout", skip(payload),fields(request_id = %Uuid::new_v4()))]
async fn mock_timeout(
    Json(payload): Json<MockTimeout>,
) -> (StatusCode, Json<()>) {
    // let request_id = Uuid::new_v4(); // 生成唯一请求ID
    // let span = tracing::info_span!("mock_timeout", request_id = %request_id);
    // let _enter = span.enter(); // 进入 Span
    // create_request_span("mock_timeout");

    // 模拟长时间操作
    sleep(Duration::from_secs(payload.sleep_seconds)).await; // 睡眠 payload.sleep_seconds 秒以确保超时
    info!("Mock timeout");
    // this will be converted into a JSON response
    // with a status code of 201 Created
    (StatusCode::OK, Json(()))
}

#[derive(Deserialize)]
struct MockTimeout {
    sleep_seconds: u64,
}

fn create_user_sub(id:u64){
    info!("Created user sub with ID: {}", id);
}

#[instrument(name = "create_user", fields(request_id = %Uuid::new_v4()))]
async fn create_user(
    Json(payload): Json<CreateUser>,
)-> (StatusCode, Json<User>) {

    // let request_id = Uuid::new_v4(); // 生成唯一请求ID
    // let span = tracing::info_span!("create_user", request_id = %request_id);
    // let _enter = span.enter(); // 进入 Span
    // create_request_span("create_user");

    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    let user = User {
        id,
        username: payload.username,
    };
    create_user_sub(id);
    info!("Created user with ID: {}", id);
    trace!("trace Created user with ID: {}", id);
    // this will be converted into a JSON response
    // with a status code of 201 Created
    (StatusCode::OK, Json(user))
}

#[derive(Deserialize,Debug)]
struct CreateUser {
    username: String,
}

#[derive(Serialize,Debug)]
struct User {
    id: u64,
    username: String,
}


async fn set_level(){
    init_log_trace().await;
}

async fn init_log_trace() {
    
    use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filter = filter::LevelFilter::WARN;
let (filter, reload_handle) = reload::Layer::new(filter);
tracing_subscriber::registry()
  .with(filter)
  .with(fmt::Layer::default())
  .init();
info!("This will be ignored");
let _= reload_handle.modify(|filter| *filter = filter::LevelFilter::TRACE);
info!("This will be logged");

}