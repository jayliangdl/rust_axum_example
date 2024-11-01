use tracing::instrument;
use axum::Extension;
use sqlx::mysql::MySqlPool;
use axum::Json;
use uuid::Uuid;
use crate::model::request::frontend::find_sku::FindSku as RequestFrontendFindSku;
use crate::model::response::frontend::find_sku::FindSku as ResponseFrontendFindSku;

use crate::models::response_models::AppResponse;
use crate::dao::sku_dao::SkuDao;
use crate::utils::cache::{CACHE,CacheType,Expiration};
use crate::utils::error::BusinessError;

#[instrument(name = "find_sku", fields(request_id = %Uuid::new_v4()))]
pub async fn find_sku(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFrontendFindSku>,
)-> Result<Json<AppResponse<Option<ResponseFrontendFindSku>>>,BusinessError> {
    let sku_code = &request.sku_code;
    let key = format!("sku:{}",sku_code);

    if let Some((_,CacheType::Sku(sku_option))) = CACHE.get(&key){
        let sku_option = ResponseFrontendFindSku::from_db_sku(sku_option);
        tracing::trace!("Cache hit");
        return Ok(Json(AppResponse::success(sku_option)));        
    }else{
        tracing::trace!("Cache miss");
        if let Ok(sku_option) = SkuDao::find_sku(&pool, sku_code).await{
            let sku_response: Option<ResponseFrontendFindSku> = ResponseFrontendFindSku::from_db_sku(sku_option.clone());
            let cache_type_with_sku: CacheType = CacheType::Sku(sku_option);
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,cache_type_with_sku));
            return Ok(Json(AppResponse::success(sku_response)));                    
        }else{
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,CacheType::Sku(None)));   
            return Ok(Json(AppResponse::success(None)));
        }
    }
}
