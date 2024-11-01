use axum::{
    extract::Query,
    Json
};
use tracing::instrument;
use uuid::Uuid;
use std::env;

use crate::utils::error::BusinessError;

#[instrument(name = "health_check", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn health_check(Query(params): Query<crate::models::request_models::HealthCheck>,) 
    -> Result<Json<crate::models::response_models::HealthCheck>, BusinessError> {
    let response = crate::models::response_models::HealthCheck {
        text: format!("receive {}",params.text),
    };    
    Ok(Json(response))
}

#[instrument(name = "env_variable", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn env_variable(Query(params): Query<crate::models::request_models::EnvVariable>,) 
    -> Result<Json<crate::models::response_models::EnvVariable>, BusinessError> {
        let text = params.text;
        let value = env::var(text).unwrap_or_else(|_|"".to_string());
    let response: crate::models::response_models::EnvVariable = crate::models::response_models::EnvVariable {
        value: format!("value {}",value),
    };    
    Ok(Json(response))
}
