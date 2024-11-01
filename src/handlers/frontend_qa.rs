use tracing::instrument;
use axum::Extension;
use axum::extract::Query;
use axum::Json;

use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use crate::model::request::frontend::get_question_by_code::GetQuestionByCode as RequestGetQuestionByCode;
use crate::model::response::frontend::get_question_by_code::GetQuestionByCode as ResponseGetQuestionByCode;
use crate::model::cache::qa::Question as CacheQuestion;

use crate::dao::qa_dao::QuestionDao;
use crate::models::response_models::AppResponse;
use crate::utils::cache::{CACHE,CacheType,Expiration};
use crate::utils::error::BusinessError;


#[instrument(name = "get_question_by_code", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn get_question_by_code(
    Extension(pool): Extension<MySqlPool>,
    Query(params): Query<RequestGetQuestionByCode>,
    ) 
    -> Result<Json<AppResponse<Option<ResponseGetQuestionByCode>>>, BusinessError> {
    let question_code = &params.question_code;
    let key = format!("question_code:{}",question_code);
    //检查缓存是否存在
    if let Some((_,CacheType::Question(question_option))) = CACHE.get(&key){
        let question_option = ResponseGetQuestionByCode::from_cache(question_option);
        tracing::trace!("Cache hit");
        return Ok(Json(AppResponse::success(question_option)));
    }else{
        //缓存不存在，从数据库中查询，并将查询结果存入缓存
        tracing::trace!("Cache miss");
        let db_review_list = QuestionDao::query_answer_by_question_code(&pool, question_code).await?;
        
        if let Ok(db_question_option) = QuestionDao::find_question_by_question_code(&pool, question_code).await{
            let db_question = db_question_option.unwrap();
            let question_response = ResponseGetQuestionByCode::from_db(db_question.clone(),db_review_list.clone());
            let cache_question = CacheQuestion::from_db(db_question, db_review_list);
            let cache_type_with_question: CacheType = CacheType::Question(Some(cache_question));
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,cache_type_with_question));
            return Ok(Json(AppResponse::success(Some(question_response))));
        }else{
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,CacheType::Question(None)));            
            return Ok(Json(AppResponse::success(None)));
        }
    }
}