use rand::Rng;
use tracing::info;
use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};

use uuid::Uuid;
use crate::models::request_models::CreateUser;
use crate::models::response_models::User;

use crate::models::ApiResponse;
use validator::Validate;
use serde_json::json;

fn create_user_sub(id:u64){
    info!("Created user sub with ID: {}", id);
}

#[instrument(name = "create_user", fields(request_id = %Uuid::new_v4()))]
pub async fn create_user(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<CreateUser>,
)-> Result<(StatusCode, Json<ApiResponse<User>>),(StatusCode,String)> {

    
    // let request_id = Uuid::new_v4(); // 生成唯一请求ID
    // let span = tracing::info_span!("create_user", request_id = %request_id);
    // let _enter = span.enter(); // 进入 Span
    // create_request_span("create_user");
    // info!("request.price:{}",request.price.fractional_digit_count());

    if let Err(errors) = request.validate(){
        let error_response = ApiResponse::ERROR {
            error_code: "0201020".to_string(), 
            error_message: "参数错误".to_string(), 
            error_parameters: Some(json!(errors)) 
        };
        return Ok((StatusCode::OK,Json(error_response)))
    }

    let _ = sqlx::query!(
        "INSERT INTO users (name, age) VALUES (?, ?)",
        request.username,
        request.age
    )
    .execute(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR,"Failed to insert user".to_string()))?;

    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen();
    let user = User {
        id,
        username: request.username.clone()
    };
    create_user_sub(id);
    info!("Created user with ID: {}", id);
    // this will be converted into a JSON response
    // with a status code of 201 Created
    Ok((StatusCode::OK, Json(ApiResponse::success ( user ))))
}

