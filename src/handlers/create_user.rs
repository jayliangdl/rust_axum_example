use rand::Rng;
use tracing::info;
use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::Json;

use uuid::Uuid;
use crate::models::request_models::CreateUser;
use crate::models::response_models::AppResponse;
use crate::models::response_models::User;
use crate::utils::error::BusinessError;

use validator::Validate;

fn create_user_sub(id:u64){
    info!("Created user sub with ID: {}", id);
}

#[instrument(name = "create_user", fields(request_id = %Uuid::new_v4()))]
pub async fn create_user(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<CreateUser>,
)-> Result<Json<AppResponse<User>>,BusinessError> {

    
    // let request_id = Uuid::new_v4(); // 生成唯一请求ID
    // let span = tracing::info_span!("create_user", request_id = %request_id);
    // let _enter = span.enter(); // 进入 Span
    // create_request_span("create_user");
    // info!("request.price:{}",request.price.fractional_digit_count());

    request.validate()?;

    // if let Err(errors) = request.validate(){
    //     let error_response = ApiResponse::error(
    //         "0201020".to_string(), 
    //         "参数错误".to_string(),             
    //         Some(json!(errors)),
    //         None
    //     );
    //     return Ok((StatusCode::OK,Json(error_response)))
    // }

    let _ = sqlx::query!(
        "INSERT INTO users (name, age) VALUES (?, ?)",
        request.username,
        request.age
    )
    .execute(&pool)
    .await?;    

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
    Ok(Json(AppResponse::success(user)))
}

