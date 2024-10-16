use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};
use tracing::instrument;
use uuid::Uuid;
use hyper::Method;
use crate::model::request::client::find_sku::FindSku as RequestClientFindSku;
use crate::model::response::client::find_sku::FindSku as ResponseClientFindSku;
use crate::utils::request_to_internal_service::request;
use crate::models::ApiResponse;
use crate::services_dependence::MY_RUST_SERVICE_SERVER;

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(client_find_sku): Json<RequestClientFindSku>
)-> Result<(StatusCode, Json<ApiResponse<Option<ResponseClientFindSku>>>),(StatusCode,String)> {
    let request_body = serde_json::to_string(&client_find_sku)
        .map_err(|op_err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to serialize request body: {}", op_err)))?;
    let response_body = request(
        &MY_RUST_SERVICE_SERVER.to_string(),
        &"/frontend/find_sku".to_string(),
        &Method::POST,
        &request_body
    ).await;
    let response_body = match response_body{
        Ok(body) => body,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR,e))?,
    };
    let api_response: ApiResponse<ResponseClientFindSku> = serde_json::from_str(&response_body)
        .map_err(|op_err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to deserialize response body: {}", op_err)))?;
    match api_response{
        ApiResponse::SUCCESS{data} => Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data:Some(data) }))),
        ApiResponse::ERROR{error_code,error_message,error_parameters} => Ok((StatusCode::OK,Json(ApiResponse::ERROR { error_code,error_message,error_parameters }))),
    }   
}
