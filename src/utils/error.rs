use std::collections::HashMap;
use serde::Serialize;
use validator::ValidationErrors;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
#[derive(Debug, Clone)]
pub enum BusinessError{
    InternalServerError((Option<String>,Option<HashMap<String, String>>)),
    SkuAlreadyExists((Option<String>,Option<HashMap<String, String>>)),
    InvalidParameter((Option<String>,Option<Vec<HashMap<String, String>>>)),
    SkuNotFound((Option<String>,Option<HashMap<String, String>>)),
    QuestionNotFound((Option<String>,Option<HashMap<String, String>>)),
    DivedByCannotBe0((Option<String>,Option<HashMap<String, String>>)),
}
#[derive(Serialize)]
pub struct Resp<D:Serialize,E:Serialize>{
    pub code:String,
    pub msg:String,
    pub success:bool,
    pub error_parameters:Option<E>,
    pub data:Option<D>,
}

impl<D,E> Resp<D,E>
where D:Serialize,E:Serialize,{
    fn new(code:String,msg:String,success:bool,data:Option<D>,error_parameters:Option<E>)->Self{
        Self{code,msg,success,data,error_parameters}
    }
    pub fn ok(code:String,msg:String,success:bool,data:Option<D>)->Self{
        Self::new(code,msg,success,data,None)
    }
    pub fn err(code:String,msg:String,error_parameters:Option<E>)->Self{
        Self::new(code,msg,false,None,error_parameters)
    }
}

impl IntoResponse for BusinessError{
    fn into_response(self) -> axum::response::Response {  
        let code = self.code();
        let default_message = self.msg();
        match self {
            BusinessError::InternalServerError((message,error_parameters)) 
            | BusinessError::SkuAlreadyExists((message,error_parameters)) 
            | BusinessError::SkuNotFound((message,error_parameters))
            | BusinessError::QuestionNotFound((message,error_parameters))
            | BusinessError::DivedByCannotBe0((message,error_parameters))=>{
                let into_response_tuple: Resp<String, HashMap<String, String>> = Resp::err(
                    code,
                    message.unwrap_or(default_message),                
                error_parameters,
                );
                Json(into_response_tuple).into_response()
            }, 
            BusinessError::InvalidParameter((message,error_parameters))=>{
                let into_response_tuple: Resp<String, Vec<HashMap<String, String>>> = Resp::err(
                    code,
                    message.unwrap_or(default_message),
                    error_parameters,
                );
                Json(into_response_tuple).into_response()
            }        
        }
        
    }
}

impl BusinessError{
    pub fn status_code(&self)->StatusCode{
        match self {
            Self::InternalServerError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        }
    }
    pub fn code(&self)->String{
        match self {
            Self::InternalServerError(_)=>"500".to_string(),
            Self::InvalidParameter(_) => "1399001".to_string(),
            Self::SkuAlreadyExists(_) => "0201021".to_string(),
            Self::SkuNotFound(_) => "0201022".to_string(),
            Self::QuestionNotFound(_) => "1301002".to_string(),
            Self::DivedByCannotBe0(_) => "1301003".to_string(),
        }
    }
    pub fn msg(&self) -> String {
        match self {
            Self::InternalServerError(_)=>"程序内部处理错误".to_string(),
            Self::InvalidParameter(_) => "参数校验异常".to_string(),
            Self::SkuAlreadyExists(_) => "商品已存在".to_string(),
            Self::SkuNotFound(_) => "商品找不到".to_string(),
            Self::QuestionNotFound(_) => "找不到对应的问题记录".to_string(),
            Self::DivedByCannotBe0(_) => "除数不能为0".to_string(),
        }
    }
}



impl From<sqlx::Error> for BusinessError {
    fn from(error: sqlx::Error) -> Self {
        BusinessError::InternalServerError(
            (Some(error.to_string()),None)
        )
    }
}

impl From<validator::ValidationErrors> for BusinessError {
    fn from(validation_errors: ValidationErrors) -> Self {
        // let error_message = validation_errors.to_string(); 
        // tracing::info!("error_message:{}",error_message);
        // 或者，遍历 `ValidationErrors` 中的字段来提取更细粒度的信息
        // let mut error_message = String::new();
        let mut errors_parameters = Vec::new();
        for (field, errors) in validation_errors.field_errors() {
            let mut error_parameters = HashMap::new();
            for error in errors {     
                error.params.iter().for_each(|(key, value)| {
                    error_parameters.insert(key.to_string(), value.to_string());
                });   
                let message = error.message.clone().unwrap_or_else(|| "未知错误".into());
                // error_message.push_str(&format!("{:?};",message));
                error_parameters.insert("message".to_string(),message.to_string());
                error_parameters.insert("field".to_string(),field.to_string());
            }
            errors_parameters.push(error_parameters);
        }
        // BusinessError::InvalidParameter(
        //     (Some(error_message), Some(error_parameters)) // 使用详细的错误信息
        // )
        BusinessError::InvalidParameter(
            (None, Some(errors_parameters)) // 使用详细的错误信息
        )
    }
}

impl From<serde_json::Error> for BusinessError {
    fn from(error: serde_json::Error) -> Self {
        BusinessError::InternalServerError(
            (Some(error.to_string()),None)
        )
    }
}