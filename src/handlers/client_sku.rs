use tracing::instrument;
use hyper::{Body, Client, Method, Request};
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};
use uuid::Uuid;
use crate::model::request::client::find_sku::FindSku as RequestClientFindSku;
use crate::model::response::client::find_sku::FindSku as ResponseClientFindSku;
use crate::model::request::frontend::find_sku::FindSku as RequestFrontFindSku;

use crate::models::ApiResponse;
use hyper::header::CONTENT_TYPE;
use tracing::info;
use crate::cache::{CACHE,CacheType};
use crate::cache::key::get_service_list_key;

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(client_find_sku): Json<RequestClientFindSku>
)-> Result<(StatusCode, Json<ApiResponse<Option<ResponseClientFindSku>>>),(StatusCode,String)> {
    // let service_url = std::env::var("SERVICE_URL")
    let key = get_service_list_key("my_rust_service".to_string());
    let (_,cache_type) = CACHE.get(&key)
    .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get SERVICE_URL".to_string()))?;
match cache_type {       
    CacheType::NacosService(service) => {        
            let instances = &service.instances.unwrap();
            let instances = instances;
            let instance = &instances[0];
            let ip = &instance.clone().ip;
            let port = &instance.port;
            let header_url = format!("http://{}:{}",ip,port);
            let uri = format!("{}/frontend/find_sku", header_url);
            let uri = uri.parse::<hyper::Uri>()
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse URI: {}", e)))?;
            let front_find_sku:RequestFrontFindSku = RequestFrontFindSku{
                sku_code:client_find_sku.sku_code
            };
            let json_body = serde_json::to_string(&front_find_sku)
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to serialize request body: {}", e)))?;
            let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(json_body))
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create request: {}", e)))?;
            let client = Client::new();
            let res = client.request(req).await
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to send request: {}", e)))?;
            //获取响应体
            let body_bytes = hyper::body::to_bytes(res.into_body()).await
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read response body: {}", e)))?;
            let body_str = String::from_utf8_lossy(&body_bytes);    
            // Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: None })))
                //反序列JSON为 ResponseClientFindSku构造体
            let api_response: ApiResponse<ResponseClientFindSku> = serde_json::from_str(&body_str)
            .map_err(|op_err| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to deserialize response body: {}", op_err)))?;

            // 根据响应类型返回相应的结果
            match api_response{
                ApiResponse::SUCCESS{data} => Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data:Some(data) }))),
                ApiResponse::ERROR{error_code,error_message,error_parameters} => Ok((StatusCode::OK,Json(ApiResponse::ERROR { error_code,error_message,error_parameters }))),
            }
        },
        _ => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get SERVICE_URL".to_string()));
        }
        
    }
    
    
}
