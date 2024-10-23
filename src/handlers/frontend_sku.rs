use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json,
};
use uuid::Uuid;
use crate::model::request::frontend::find_sku::FindSku as RequestFrontendFindSku;
use crate::model::response::frontend::find_sku::FindSku as ResponseFrontendFindSku;

use crate::models::ApiResponse;
use crate::dao::sku_dao::SkuDao;
use crate::utils::cache::{CACHE,CacheType,Expiration};

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFrontendFindSku>,
)-> Result<(StatusCode, Json<ApiResponse<Option<ResponseFrontendFindSku>>>),(StatusCode,String)> {
    // return Err((StatusCode::INTERNAL_SERVER_ERROR,"Not implemented".to_string()));
    let sku_code = &request.sku_code;
    let key = format!("sku:{}",sku_code);

    if let Some((_,CacheType::Sku(sku_option))) = CACHE.get(&key){
        let sku_option = ResponseFrontendFindSku::from_db_sku(sku_option);
        tracing::trace!("Cache hit");
        return Ok((StatusCode::OK,Json(ApiResponse::success(Some(sku_option)))));
    }else{
        tracing::trace!("Cache miss");
        if let Ok(sku_option) = SkuDao::find_sku(&pool, sku_code).await{
            let sku_response: Option<ResponseFrontendFindSku> = ResponseFrontendFindSku::from_db_sku(sku_option.clone());
            let cache_type_with_sku: CacheType = CacheType::Sku(sku_option);
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,cache_type_with_sku));
            return Ok((StatusCode::OK,Json(ApiResponse::success ( Some(sku_response) ))));
        }else{
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,CacheType::Sku(None)));            
            return Ok((StatusCode::OK,Json(ApiResponse::success (None))));
        }
    }
}
