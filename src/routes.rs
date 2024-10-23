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
        .route("/operation/create_question", post(create_question))
        .route("/operation/update_question", post(update_question))
}
