use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::Json;
use tracing::instrument;
use uuid::Uuid;
use hyper::Method;
use crate::model::request::client::find_sku::FindSku as RequestClientFindSku;
use crate::model::response::client::find_sku::FindSku as ResponseClientFindSku;
use crate::models::response_models::AppResponse;
use crate::utils::error::BusinessError;
use crate::utils::request_to_internal_service::request;
use crate::services_dependence::MY_RUST_SERVICE_SERVER;

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(client_find_sku): Json<RequestClientFindSku>
)-> Result<Json<AppResponse<Option<ResponseClientFindSku>>>,BusinessError> {
    let request_body = serde_json::to_string(&client_find_sku)?;
        // .map_err(|op_err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to serialize request body: {}", op_err)))?;
    let response_body = request(
        &MY_RUST_SERVICE_SERVER.to_string(),
        &"/frontend/find_sku".to_string(),
        &Method::POST,
        &request_body
    ).await;
    let response_body = match response_body{
        Ok(body) => body,
        Err(e) => return Err(BusinessError::InternalServerError((Some(e),None)))?,
    };
    let api_response = serde_json::from_str(&response_body)?;
    Ok(Json(AppResponse::success(api_response)))
}
