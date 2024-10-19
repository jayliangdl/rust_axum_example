use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use serde_json::json;

// 打印请求和响应的中间件
pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, request_body) = req.into_parts();
    let request_bytes = buffer_body("request", request_body).await?;

    // 获取整个 header 信息
    let headers = &parts.headers;
    let headers_string = format!("{:?}", headers);

    // 重新组合请求
    let req = Request::from_parts(parts, Body::from(request_bytes.clone()));

    // 获取 trace_id
    let trace_id = req
        .headers()
        .get("trace_id")
        .map(|v| v.to_str().unwrap_or_default().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // 通过下一个处理器
    let res = next.run(req).await;

    // 处理响应的 Parts 和 Body 部分
    let (parts, response_body) = res.into_parts();
    let response_status = parts.status; // 提取响应状态码
    let response_bytes = buffer_body("response", response_body).await;

    match response_bytes {
        Ok(response_bytes) => {
            let res = Response::from_parts(parts, Body::from(response_bytes.clone()));
            let request_body_str = std::str::from_utf8(&request_bytes).unwrap_or("<invalid utf-8>");
            let response_body_str = std::str::from_utf8(&response_bytes).unwrap_or("<invalid utf-8>");

            let log_data = json!({
                "trace_id": trace_id,
                "headers": headers_string,
                "request_body": request_body_str,
                "response_status_code": response_status.as_u16(),
                "response_body": response_body_str,
            });

            tracing::info!("{}", log_data);
            Ok(res)
        }
        Err((status_code, error_message)) => {
            let request_body_str = std::str::from_utf8(&request_bytes).unwrap_or("<invalid utf-8>");

            let log_data = json!({
                "trace_id": trace_id,
                "headers": headers_string,
                "request_body": request_body_str,
                "response_status_code": status_code.as_u16(),
                "response_body": format!("Failed to buffer response: {}", error_message),
            });

            tracing::error!("{}", log_data);
            Err((status_code, error_message))
        }
    }
}

async fn buffer_body<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    Ok(bytes)
}
