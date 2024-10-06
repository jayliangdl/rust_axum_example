use tracing::info;
use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};
use serde_json::json;
use serde_json::Map;
use uuid::Uuid;
use chrono::{Utc,DateTime};
// use validator::Validate;
use crate::model::request::operation::{
    create_sku::CreateSku as RequestCreateSku,
    update_sku::UpdateSku as RequestUpdateSku,
    find_sku::FindSku as RequestFindSku,
};

use crate::model::response::operation::{
    create_sku::CreateSku as ResponseCreateSku,
    update_sku::UpdateSku as ResponseUpdateSku,
    find_sku::FindSku as ResponseFindSku,
};

use crate::model::db::sku::Price;
use crate::models::ApiResponse;
use crate::dao::sku_dao::SkuDao;

#[instrument(name = "create_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn create_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestCreateSku>,
)-> Result<(StatusCode, Json<ApiResponse<ResponseCreateSku>>),(StatusCode,String)> {
    if let Err(errors) = request.custom_validate(&pool).await{
        return Ok((StatusCode::OK,Json(errors)))
    }

    let sku = request.into_db_sku();

    // 获取当前时间戳
    let current_time:DateTime<Utc> = Utc::now();

    // 开始一个事务
    let mut transaction = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;

    SkuDao::insert_sku(&mut transaction, &sku, current_time).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert SKU".to_string()))?;

    //request.price_list转换成类型为model::db::sku::Price的price_list
    let price_list = request.price_list.clone().into_iter().map(|price|{
        let mut db_price = price.into_db_price(&request.sku_code);
        db_price.create_date_time = Some(current_time);
        db_price.update_date_time = Some(current_time);
        db_price
    }).collect::<Vec<Price>>();

    SkuDao::insert_sku_price_list(&mut transaction, &price_list, current_time).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert SKU price list".to_string()))?;

     // 创建 JSON 内容
     let content = json!({
        "name": &request.name,
        "description":&sku.description.unwrap_or_else(|| "".to_string()),

    });

    SkuDao::insert_sku_log(&mut transaction,&request.sku_code,content,current_time).await?;

    // 提交事务
    transaction.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;


    info!("Created SKU : {:?}", request);

    let response = ResponseCreateSku{
        sku_code: request.sku_code,
    };
    Ok((StatusCode::OK, Json(ApiResponse::SUCCESS { data: response })))
}

#[instrument(name = "update_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn update_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestUpdateSku>,
)-> Result<(StatusCode, Json<ApiResponse<ResponseUpdateSku>>),(StatusCode,String)> {
    if let Err(errors) = request.custom_validate(&pool).await{
        return Ok((StatusCode::OK,Json(errors)))
    }
    // 获取当前时间戳
    let current_time:DateTime<Utc> = Utc::now();

    // 开始一个事务
    let mut transaction: sqlx::Transaction<'_, sqlx::MySql> = pool.begin().await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;

    let sku = request.into_db_sku();
    SkuDao::update_sku(&mut transaction, &sku, current_time).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update SKU".to_string()))?;

     // 创建 JSON 内容
     let mut content = Map::new();
     if let Some(ref name) = request.name{
        content.insert("name".to_string(), json!(name));
     }
     if let Some(ref description) = request.description{
        content.insert("description".to_string(), json!(description));
    }
    SkuDao::insert_sku_log(&mut transaction,&request.sku_code,serde_json::Value::Object(content),current_time).await?;

    // 提交事务
    transaction.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;


    info!("Updated SKU : {:?}", &request);

    let response: ApiResponse<ResponseUpdateSku> = ApiResponse::SUCCESS { 
        data: ResponseUpdateSku
        {
            sku_code: request.sku_code
        }
    };
    Ok((StatusCode::OK, Json(response)))
}

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFindSku>,
)-> Result<(StatusCode, Json<ApiResponse<Option<ResponseFindSku>>>),(StatusCode,String)> {
    if let Ok(sku_option) = SkuDao::find_sku(&pool, &request.sku_code).await{
        let sku_response = ResponseFindSku::from_db_sku(sku_option);
        Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: (sku_response) })))        
    }else{
        Err((StatusCode::INTERNAL_SERVER_ERROR,"Cannot execute FindSku::from_db_sku".to_string()))
    }
}
