use rand::Rng;
use tracing::info;
use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};

use dotenv::dotenv;

use uuid::Uuid;
use crate::models::request_models::Create;
use crate::models::response_models::User;


#[instrument(name = "create_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn create_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<create_sku>,
)-> Result<(StatusCode, Json<User>),(StatusCode,String)> {

    // let request_id = Uuid::new_v4(); // 生成唯一请求ID
    // let span = tracing::info_span!("create_user", request_id = %request_id);
    // let _enter = span.enter(); // 进入 Span
    // create_request_span("create_user");

    dotenv().ok();  
    

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
        username: request.username.clone(),
    };
    create_user_sub(id);
    info!("Created user with ID: {}", id);
    // this will be converted into a JSON response
    // with a status code of 201 Created
    Ok((StatusCode::OK, Json(user)))
}
