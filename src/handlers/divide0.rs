use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::Query;
//此handler用于测试0作为除数的情况，检查是否会panic，并让程序崩溃
//结论：不会导致程序崩溃
pub async fn divide0(
    Query(params): Query<std::collections::HashMap<String,String>>,
    ) -> impl IntoResponse{
        let a= params.get("dived_by").unwrap().parse::<i32>().unwrap();
        return (StatusCode::OK,format!("divide0:{}", 10 / a));
}
