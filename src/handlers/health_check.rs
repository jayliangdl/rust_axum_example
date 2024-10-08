use axum::{
    extract::Query,
    http::StatusCode, Json
};
use tracing::instrument;
use uuid::Uuid;
use std::env;

// #[instrument(name = "health_check", skip(request),fields(request_id = %Uuid::new_v4()))]
// pub async fn health_check(Json(request): Json<crate::models::request_models::HealthCheck>,) 
//     -> (StatusCode, Json<crate::models::response_models::HealthCheck>) {
//     let response = crate::models::response_models::HealthCheck {
//         text: format!("receive {}",request.text),
//     };    
//     (StatusCode::OK, Json(response))
// }

#[instrument(name = "health_check", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn health_check(Query(params): Query<crate::models::request_models::HealthCheck>,) 
    -> Result<(StatusCode, Json<crate::models::response_models::HealthCheck>), (StatusCode, String)> {
    let response = crate::models::response_models::HealthCheck {
        text: format!("receive {}",params.text),
    };    
    Ok((StatusCode::OK, Json(response)))
}

#[instrument(name = "env_variable", skip(params),fields(request_id = %Uuid::new_v4()))]
pub async fn env_variable(Query(params): Query<crate::models::request_models::EnvVariable>,) 
    -> Result<(StatusCode, Json<crate::models::response_models::EnvVariable>), (StatusCode, String)> {
        let text = params.text;
        let value = env::var(text).unwrap_or_else(|_|"".to_string());
    let response: crate::models::response_models::EnvVariable = crate::models::response_models::EnvVariable {
        value: format!("value {}",value),
    };    
    Ok((StatusCode::OK, Json(response)))
}
