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
    Json
};
use uuid::Uuid;
use crate::model::request::operation::{
    create_question::CreateQuestion as RequestCreateQuestion,
    update_question::UpdateQuestion as RequestUpdateQuestion,
    find_question_list_for_trad::FindQuestionListForTrad as RequestFindQuestionListForTrad,
    delete_question::DeleteQuestion as RequestDeleteQuestion,
    top_question::TopQuestion as RequestTopQuestion,
    cancel_top_question::CancelTopQuestion as RequestCancelTopQuestion
};
use crate::model::response::operation::find_question_list_for_trad::Question;
use crate::model::response::operation:: find_question_list_for_trad::FindQuestionListForTrad as ResponseFindQuestionListForTrad;

use crate::utils::datetime::now_local;
use crate::utils::error::BusinessError;
use crate::dao::qa_dao::QuestionDao;
use crate::models::response_models::AppResponse;

#[instrument(name = "create_question", fields(request_id = %Uuid::new_v4()))]
pub async fn create_question(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(headers): TypedHeader<UserAgent>,
    Json(request): Json<RequestCreateQuestion>,
)-> Result<Json<AppResponse<String>>,BusinessError> {
    request.custom_validate().await?;

    info!("创建Question : {:?}", request.clone());
    let question = request.into_db_question();
    // 开始一个事务
    let mut transaction = pool.begin().await?;

    let new_question_id = QuestionDao::insert_question(&mut transaction, &question).await?;

    let answer_list = request.answer_list;
    for answer in &answer_list{
        let db_answer = answer.into_db_answer(question.question_code.clone());
        QuestionDao::insert_answer(&mut transaction, &db_answer).await?;
    }

    // 提交事务
    transaction.commit().await?;
    
    Ok(Json(AppResponse::success(new_question_id.to_string())))
}

#[instrument(name = "update_question", fields(request_id = %Uuid::new_v4()))]
pub async fn update_question(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(headers): TypedHeader<UserAgent>,
    Json(request): Json<RequestUpdateQuestion>,
)-> Result<Json<AppResponse<bool>>,BusinessError> {
    request.custom_validate().await?;    

    //先依据question_code查询数据库，确保问题记录是存在的
    if let Ok(question_option) = QuestionDao::find_question_by_question_code(&pool, &request.question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), request.question_code.clone());
            return Err(BusinessError::QuestionNotFound(
                (None,Some(parameters))
            ))
        }
    }else {
        return Err(BusinessError::QuestionNotFound(
            (Some("以问题编号查询问题记录是否存在--此步骤执行失败".to_string()),None)
        ))
    }
    info!("更新Question : {:?}", request.clone());
    let mut question = request.into_db_question();

    let now = now_local();
    question.update_time(now);


    // 开始一个事务
    let mut transaction = pool.begin().await?;

    let _ = QuestionDao::update_question(&mut transaction, &question).await;
    let _ = QuestionDao::delete_answer(&mut transaction, &question.question_code).await;

    let answer_list = request.answer_list;
    for answer in &answer_list{
        let db_answer = answer.into_db_answer(question.question_code.clone());
        let _ = QuestionDao::insert_answer(&mut transaction, &db_answer).await;
    }

    // 提交事务
    transaction.commit().await?;

    let response = true;
    Ok(Json(AppResponse::success(response)))
}

#[instrument(name = "findQuestionListForTrad", fields(request_id = %Uuid::new_v4()))]
pub async fn find_question_list_for_trad(
    Extension(pool): Extension<MySqlPool>,
    Json(request): Json<RequestFindQuestionListForTrad>,
)-> Result<Json<AppResponse<ResponseFindQuestionListForTrad>>,BusinessError> {
    request.validate()?;
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
        Ok(Json(AppResponse::success(response_find_question_list_for_trad)))
    }else{
        Err(BusinessError::InternalServerError(
            (None,None)
        ))
    }
}

#[instrument(name = "top_question", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn top_question(
    Extension(pool): Extension<MySqlPool>,
    Query(params): Query<RequestTopQuestion>,
    ) 
    -> Result<Json<AppResponse<bool>>,BusinessError>  {
    let question_code = &params.question_code;
    if let Ok(question_option) = QuestionDao::find_question_by_question_code(&pool, &question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), question_code.clone());
            return Err(BusinessError::QuestionNotFound(
                (None,Some(parameters))
            ));     
        }else{
            // 开始一个事务
            let mut transaction = pool.begin().await?;
            let next_sort = QuestionDao::have_next_sort(&pool).await?;
            // let sort:i32 = 1;
            let _ = QuestionDao::update_sort_by_question_code(&mut transaction, question_code, next_sort).await;
            // 提交事务
            transaction.commit().await?;
        }
    }else {
        return Err(BusinessError::InternalServerError(
            (Some("查询Question失败".to_string()),None))
        )
    }
    Ok(Json(AppResponse::success(true)))

}

#[instrument(name = "cancel_top_question", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn cancel_top_question(
    Extension(pool): Extension<MySqlPool>,
    Query(params): Query<RequestCancelTopQuestion>,
    ) 
    -> Result<Json<AppResponse<bool>>, BusinessError> {
    let question_code = &params.question_code;
    if let Ok(question_option) = QuestionDao::find_question_by_question_code(&pool, &question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), question_code.clone());
            return Err(BusinessError::QuestionNotFound(
                (None,Some(parameters))
            ));            
        }else{
            // 开始一个事务
            let mut transaction = pool.begin().await?;
            let reset_sort = 0;
            QuestionDao::update_sort_by_question_code(&mut transaction, question_code, reset_sort).await?;

            // 提交事务
            transaction.commit().await?;
        }
    }else {
        return Err(BusinessError::InternalServerError(
            (Some("以问题编号查询问题记录是否存在--此步骤执行失败".to_string()),None)
        ));
    }
    Ok(Json(AppResponse::success(true)))

}

#[instrument(name = "disabled_question", fields(request_id = %Uuid::new_v4()))]
pub async fn disabled_question(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(headers): TypedHeader<UserAgent>,
    Json(request): Json<RequestDeleteQuestion>,
)-> Result<Json<AppResponse<bool>>,BusinessError> {
    request.validate()?;
    //先依据question_code查询数据库，确保question_code存在
    if let Ok(question_option) = QuestionDao::find_question_by_question_code(&pool, &request.question_code).await{
        if question_option.is_none(){
            let mut parameters= HashMap::new();
            parameters.insert("question_code".to_string(), request.question_code.clone());
            return Err(BusinessError::QuestionNotFound(
                (None,Some(parameters))
            ))
        }else{
            // 开始一个事务
            let mut transaction = pool.begin().await?;
            QuestionDao::disabled_question_and_answer_by_question_code(&mut transaction, &request.question_code).await?;
            // 提交事务
            transaction.commit().await?;
        }
    }else {
        return Err(BusinessError::InternalServerError(
            (Some("以问题编号查询问题记录是否存在--此步骤执行失败".to_string()),None)
        ));
    }
    Ok(Json(AppResponse::success(true)))
}



