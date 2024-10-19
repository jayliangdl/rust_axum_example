use tracing::info;
use tracing::instrument;
use axum::Extension;
use axum_extra::TypedHeader;
use headers::UserAgent;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};
use uuid::Uuid;
// use validator::Validate;
use crate::model::request::operation::create_question::CreateQuestion as RequestCreateQuestion;
use crate::model::response::operation::create_question::CreateQuestion as ResponseCreateQuestion;

use crate::models::ApiResponse;
use crate::dao::qa_dao::QuestionDao;

#[instrument(name = "create_question", fields(request_id = %Uuid::new_v4()))]
pub async fn create_question(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(headers): TypedHeader<UserAgent>,
    Json(request): Json<RequestCreateQuestion>,
)-> Result<(StatusCode, Json<ApiResponse<ResponseCreateQuestion>>),(StatusCode,String)> {
    if let Err(errors) = request.custom_validate().await{
        return Ok((StatusCode::OK,Json(errors)))
    }

    info!("创建Question : {:?}", request.clone());
    let question = request.into_db_question();


    // 开始一个事务
    let mut transaction = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;

    QuestionDao::insert_question(&mut transaction, &question).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入Question到数据库失败".to_string()))?;

    let answer_list = request.answer_list;
    for answer in &answer_list{
        let db_answer = answer.into_db_answer(question.question_code.clone());
        QuestionDao::insert_answer(&mut transaction, &db_answer).await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入Answer到数据库失败".to_string()))?;
    }

    // 提交事务
    transaction.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;

    let response = ResponseCreateQuestion{
        data: None
    };
    let api_response: ApiResponse<ResponseCreateQuestion> = ApiResponse::success(
         response 
    );
    Ok((StatusCode::OK, Json(api_response)))
}

// #[instrument(name = "update_sku", fields(request_id = %Uuid::new_v4()))]
// pub async fn update_sku(
//     Extension(pool): Extension<MySqlPool>,
//     Json(request): Json<RequestUpdateSku>,
// )-> Result<(StatusCode, Json<ApiResponse<ResponseUpdateSku>>),(StatusCode,String)> {
//     if let Err(errors) = request.custom_validate(&pool).await{
//         return Ok((StatusCode::OK,Json(errors)))
//     }
//     // 获取当前时间戳
//     let current_time:DateTime<Utc> = Utc::now();

//     // 开始一个事务
//     let mut transaction: sqlx::Transaction<'_, sqlx::MySql> = pool.begin().await
//     .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;

//     let sku = request.into_db_sku();
//     SkuDao::update_sku(&mut transaction, &sku, current_time).await
//         .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update SKU".to_string()))?;

//      // 创建 JSON 内容
//      let mut content = Map::new();
//      if let Some(ref name) = request.name{
//         content.insert("name".to_string(), json!(name));
//      }
//      if let Some(ref description) = request.description{
//         content.insert("description".to_string(), json!(description));
//     }
//     SkuDao::insert_sku_log(&mut transaction,&request.sku_code,serde_json::Value::Object(content),current_time).await?;

//     // 提交事务
//     transaction.commit().await
//         .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;


//     info!("Updated SKU : {:?}", &request);

//     let response: ApiResponse<ResponseUpdateSku> = ApiResponse::SUCCESS { 
//         data: ResponseUpdateSku
//         {
//             sku_code: request.sku_code
//         }
//     };
//     Ok((StatusCode::OK, Json(response)))
// }

// #[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
// pub async fn find_sku(
//     Extension(pool): Extension<MySqlPool>,
//     Json(request): Json<RequestFindSku>,
// )-> Result<(StatusCode, Json<ApiResponse<Option<ResponseFindSku>>>),(StatusCode,String)> {
//     if let Ok(sku_option) = SkuDao::find_sku(&pool, &request.sku_code).await{
//         let sku_response = ResponseFindSku::from_db_sku(sku_option);
//         Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: (sku_response) })))        
//     }else{
//         Err((StatusCode::INTERNAL_SERVER_ERROR,"Cannot execute FindSku::from_db_sku".to_string()))
//     }
// }
