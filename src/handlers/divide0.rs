use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::Query;
use axum::Json;
use crate::utils::error::BusinessError;
//此handler用于测试0作为除数的情况，检查是否会panic，并让程序崩溃
//结论：不会导致程序崩溃
pub async fn divide0(
    Query(params): Query<std::collections::HashMap<String,String>>,
    ) -> impl IntoResponse{
        let a= params.get("dived_by").unwrap().parse::<i32>().unwrap();
        return (StatusCode::OK,format!("divide0:{}", 10 / a));
}

pub async fn divide_0(
    Query(params): Query<std::collections::HashMap<String,String>>,
    ) -> Result<Json<Divide0Response>,BusinessError>{
        let a: i32= params.get("dived_by").unwrap().parse::<i32>().unwrap();
        check_dived_by(a)?;
        // if let Err(e) = result{
        //     return Err(Json(e));
        // }
        let result = 10/a;
        let result = Divide0Response::new(result);
        Ok(result.have_resp())
}

fn check_dived_by(a:i32)->Result<i32,BusinessError>{
    if a==0{
        
        Err(BusinessError::DivedByCannotBe0(
            (None,None)
        ))
        // Err((StatusCode::INTERNAL_SERVER_ERROR, "插入Question到数据库失败".to_string()))
    }else{
        Ok(a)
    }
}

#[derive(serde::Deserialize,Clone)]
pub struct Divide0Request{
    pub dived_by:i32,
}

#[derive(serde::Serialize,Clone)]
pub struct Divide0Response{
    pub dived_by:i32,
}
impl Divide0Response{
    pub fn new(dived_by:i32)->Self{
        Self{
            dived_by
        }
    }

    pub fn have_resp(self)->Json<Self>{
        Json(self)
    }
}