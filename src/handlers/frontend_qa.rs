use tracing::instrument;
use axum::Extension;
use axum::extract::Query;
use axum::{
    http::StatusCode, Json,
};

use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use crate::model::request::frontend::get_question_by_code::GetQuestionByCode as RequestGetQuestionByCode;
use crate::model::response::frontend::get_question_by_code::GetQuestionByCode as ResponseGetQuestionByCode;
use crate::model::cache::qa::Question as CacheQuestion;

use crate::dao::qa_dao::QuestionDao;
use crate::models::ApiResponse;
use crate::utils::cache::{CACHE,CacheType,Expiration};


#[instrument(name = "get_question_by_code", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn get_question_by_code(
    Extension(pool): Extension<MySqlPool>,
    Query(params): Query<RequestGetQuestionByCode>,
    ) 
    -> Result<(StatusCode, Json<ApiResponse<Option<ResponseGetQuestionByCode>>>), (StatusCode, String)> {
    let question_code = &params.question_code;
    let key = format!("question_code:{}",question_code);
    if let Some((_,CacheType::Question(question_option))) = CACHE.get(&key){
        let question_option = ResponseGetQuestionByCode::from_cache(question_option);
        tracing::trace!("Cache hit");
        return Ok((StatusCode::OK,Json(ApiResponse::success(Some(question_option)))));
    }else{
        tracing::trace!("Cache miss");
        // let answers:Vec<ResponseAnswerGetQuestionByCode> = QuestionDao::query_answer_by_question_code(&pool, question_code).await
        //     .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR,format!("数据库错误： {}",e.1)))?
        //     .into_iter().map(|answer|ResponseAnswerGetQuestionByCode::from_db(answer)).collect();
        
        let db_review_list = QuestionDao::query_answer_by_question_code(&pool, question_code).await
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR,format!("数据库错误： {}",e.1)))?;
        
        if let Ok(db_question_option) = QuestionDao::query_question_by_question_code(&pool, question_code).await{
            let db_question = db_question_option.unwrap();
            let question_response = ResponseGetQuestionByCode::from_db(db_question.clone(),db_review_list.clone());
            let cache_question = CacheQuestion::from_db(db_question, db_review_list);
            let cache_type_with_question: CacheType = CacheType::Question(Some(cache_question));
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,cache_type_with_question));
            return Ok((StatusCode::OK,Json(ApiResponse::success ( Some(Some(question_response)) ))));
        }else{
            CACHE.insert(key.clone(), (Expiration::AfterShortTime,CacheType::Question(None)));            
            return Ok((StatusCode::OK,Json(ApiResponse::success (None))));
        }
    }
}