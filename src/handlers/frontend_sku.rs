use tracing::trace;
use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::{
    http::StatusCode, Json
};
use uuid::Uuid;
use crate::model::request::frontend::find_sku::FindSku as RequestFrontendFindSku;
use crate::model::response::frontend::find_sku::FindSku as ResponseFrontendFindSku;

use crate::models::ApiResponse;
use crate::dao::sku_dao::SkuDao;
use crate::cache::{CACHE,CacheType,Expiration};

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFrontendFindSku>,
)-> Result<(StatusCode, Json<ApiResponse<Option<ResponseFrontendFindSku>>>),(StatusCode,String)> {
    let sku_code = &request.sku_code;
    let key = format!("sku:{}",sku_code);

    // if let CacheType::Sku(sku) = CACHE.get_with(key.clone(),  || {
    //     // let sku = crate::model::db::sku::Sku{
    //     //     sku_code: "test".to_string(),
    //     //     name: "test".to_string(),
    //     //     description: Some("test".to_string()),
    //     // };

    //     if let Ok(sku_option) = SkuDao::find_sku(&pool, sku_code).await{
    //         return CacheType::Sku(sku_option);
    //     }else{
    //         return CacheType::Sku(None);
    //     }
    //     info!("Cache miss");
        
    // }){
    //     let sku = ResponseFrontendFindSku::from_db_sku(sku);
    //     return Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: sku })));
    // }
    // return Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: None })));


    if let Some((_,CacheType::Sku(sku_option))) = CACHE.get(&key){
        let sku_option = ResponseFrontendFindSku::from_db_sku(sku_option);
        tracing::info!("Cache hit");
        return Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: sku_option })));
    }else{
        tracing::info!("Cache miss");
        if let Ok(sku_option) = SkuDao::find_sku(&pool, sku_code).await{
            let sku_response = ResponseFrontendFindSku::from_db_sku(sku_option.clone());
            let cache_type_with_sku: CacheType = CacheType::Sku(sku_option);
            CACHE.get_with(key.clone(), || (Expiration::AfterShortTime,cache_type_with_sku));
            // CACHE.insert(key.clone(), cache_type_with_sku);
            return Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: sku_response })));
        }else{
            CACHE.get_with(key.clone(), || (Expiration::AfterShortTime,CacheType::Sku(None)));
            // CACHE.insert(key.clone(), CacheType::Sku(None));
            return Ok((StatusCode::OK,Json(ApiResponse::SUCCESS { data: None })));
        }
    }
}
