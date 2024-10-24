use axum::http::response;
use tracing::info;
use tracing::instrument;
use axum::Extension;
use axum_extra::TypedHeader;
use headers::UserAgent;
use sqlx::mysql::MySqlPool;
use validator::Validate;
use std::collections::HashMap;
use axum::{
    extract::Query,
    http::StatusCode, Json
};
use uuid::Uuid;
// use validator::Validate;
use crate::model::request::operation::{
    create_question::CreateQuestion as RequestCreateQuestion,
    update_question::UpdateQuestion as RequestUpdateQuestion,
    find_question_list_for_trad::FindQuestionListForTrad as RequestFindQuestionListForTrad,
    top_question::TopQuestion as RequestTopQuestion,
};
use crate::model::response::operation::find_question_list_for_trad::Question;
use crate::model::response::operation::{
    create_question::CreateQuestion as ResponseCreateQuestion,
    update_question::UpdateQuestion as ResponseUpdateQuestion,
    find_question_list_for_trad::FindQuestionListForTrad as ResponseFindQuestionListForTrad,
    top_question::TopQuestion as ResponseTopQuestion,
};
use crate::utils::datetime::now_local;
use crate::utils::error::ErrorCode;
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
         Some(response)
    );
    Ok((StatusCode::OK, Json(api_response)))
}

#[instrument(name = "update_question", fields(request_id = %Uuid::new_v4()))]
pub async fn update_question(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(headers): TypedHeader<UserAgent>,
    Json(request): Json<RequestUpdateQuestion>,
)-> Result<(StatusCode, Json<ApiResponse<ResponseUpdateQuestion>>),(StatusCode,String)> {
    if let Err(errors) = request.custom_validate().await{
        return Ok((StatusCode::OK,Json(errors)))
    }

    //先依据question_code查询数据库，确保question_code存在
    if let Ok(question_option) = QuestionDao::query_question_by_question_code(&pool, &request.question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), &request.question_code);
            let api_response = ErrorCode::QuestionNotFound.to_response_from_hashmap::<ResponseUpdateQuestion>(parameters,None);
            return Ok((StatusCode::OK,Json(api_response)))            
        }
    }else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR,"查询Question失败".to_string()));
    }
    info!("更新Question : {:?}", request.clone());
    let mut question = request.into_db_question();

    let now = now_local();
    question.update_time(now);


    // 开始一个事务
    let mut transaction = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;

    QuestionDao::update_question(&mut transaction, &question).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "更新Question到数据库失败".to_string()))?;

    QuestionDao::delete_answer(&mut transaction, &question.question_code).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "删除Answer到数据库失败".to_string()))?;

    let answer_list = request.answer_list;
    for answer in &answer_list{
        let db_answer = answer.into_db_answer(question.question_code.clone());
        QuestionDao::insert_answer(&mut transaction, &db_answer).await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "插入Answer到数据库失败".to_string()))?;
    }

    // 提交事务
    transaction.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;

    let response = ResponseUpdateQuestion{
        data: true
    };
    let api_response: ApiResponse<ResponseUpdateQuestion> = ApiResponse::success(
         Some(response)
    );
    Ok((StatusCode::OK, Json(api_response)))
}

#[instrument(name = "findQuestionListForTrad", fields(request_id = %Uuid::new_v4()))]
pub async fn find_question_list_for_trad(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFindQuestionListForTrad>,
)-> Result<(StatusCode, Json<ApiResponse<ResponseFindQuestionListForTrad>>),(StatusCode,String)> {
    request.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let current_pageno = request.current_pageno;
    let page_size = request.page_size;
    
    if let Ok(page_questions) = QuestionDao::query_question_list(&pool, &request, current_pageno, page_size).await{        
        let mut response_questions = vec![];
        for question in &page_questions.data {
            let question_code = &question.question_code;
            match QuestionDao::query_answer_by_question_code(&pool, question_code).await {
                Ok(answers) => {
                    let question_response = Question::from_db_questions(question.clone(),answers);
                    response_questions.push(question_response);
                },
                Err(_) => {
                    let question_response = Question::from_db_questions(question.clone(),vec![]);
                    response_questions.push(question_response);
                }
            }
        }
        let response_find_question_list_for_trad = ResponseFindQuestionListForTrad::new(
            page_questions.total_records,
            page_questions.current_pageno,
            page_questions.page_size,
            page_questions.total_pages,
            response_questions,
        );
        let api_response = ApiResponse::success(
            Some(response_find_question_list_for_trad)
       );
       Ok((StatusCode::OK, Json(api_response)))
    }else{
        Err((StatusCode::INTERNAL_SERVER_ERROR,"Cannot execute FindSku::from_db_sku".to_string()))
    }
}

#[instrument(name = "top_question", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn top_question(
    Extension(pool): Extension<MySqlPool>,
    Query(params): Query<crate::model::request::operation::top_question::TopQuestion>,
    ) 
    -> Result<(StatusCode, Json<ApiResponse<bool>>), (StatusCode, String)> {
    let question_code = &params.question_code;
    if let Ok(question_option) = QuestionDao::query_question_by_question_code(&pool, &question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), question_code);
            let api_response = ErrorCode::QuestionNotFound.to_response_from_hashmap::<bool>(parameters,None);
            return Ok((StatusCode::OK,Json(api_response)))            
        }else{
            // 开始一个事务
            let mut transaction = pool.begin().await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;
            let next_sort = QuestionDao::have_next_sort(&pool).await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction".to_string()))?;
            // let sort:i32 = 1;
            QuestionDao::update_sort_by_question_code(&mut transaction, question_code, next_sort).await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "置顶Question失败".to_string()))?;

            // 提交事务
            transaction.commit().await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction".to_string()))?;
        }
    }else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR,"查询Question失败".to_string()));
    }
    let api_response = ApiResponse::success(
        Some(true)
   );
    Ok((StatusCode::OK, Json(api_response)))

}



