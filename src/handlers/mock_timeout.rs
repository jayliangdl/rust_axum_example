use crate::models::request_models::MockTimeout;
use tracing::instrument;
use axum::{
    http::StatusCode, Json
};
use tokio::time::sleep;
use std::time::Duration;
use tracing::info;
use crate::Uuid;

#[instrument(name = "mock_timeout", skip(payload),fields(request_id = %Uuid::new_v4()))]
pub async fn mock_timeout(
    Json(payload): Json<MockTimeout>,) -> (StatusCode, Json<()>) {
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