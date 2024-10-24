use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::{
    health_check::{health_check, env_variable},    
    mock_timeout::mock_timeout,
    create_user::create_user,
    operation_sku::{create_sku, update_sku, find_sku},
    frontend_sku::find_sku as front_find_sku,
    client_sku::find_sku as client_find_sku,
    operation_qa::create_question,
    operation_qa::update_question,
    operation_qa::find_question_list_for_trad,
    operation_qa::top_question,
    operation_qa::cancel_top_question,
    operation_qa::disabled_question,
};

pub fn app_router() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/env_variable", get(env_variable))
        .route("/mock_timeout", post(mock_timeout))
        .route("/create_user", post(create_user))
        .route("/operation/create_sku", post(create_sku))
        .route("/operation/update_sku", post(update_sku))
        .route("/operation/find_sku", post(find_sku))
        .route("/frontend/find_sku", post(front_find_sku))
        .route("/client/find_sku", post(client_find_sku))
        .route("/operation/createQuestion", post(create_question))
        .route("/operation/updateQuestion", post(update_question))
        .route("/operation/findQuestionListForTrad", post(find_question_list_for_trad))
        .route("/operation/topQuestion", get(top_question))
        .route("/operation/cancelTopQuestion", get(cancel_top_question))
        .route("/operation/deleteQuestion", post(disabled_question))

    }
