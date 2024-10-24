use std::collections::HashMap;
use validator::ValidationErrors;
use crate::models::ApiResponse;
use serde_json::json;
use crate::models::ErrorResponse;
#[derive(Debug, Clone)]
pub enum ErrorCode{
    InternalServerError,
    SkuAlreadyExists,
    InvalidParameter,
    SkuNotFound,
    QuestionNotFound,
}

impl ErrorCode{
    pub fn code(&self)->&'static str{
        match self {
            Self::InternalServerError=>"500",
            Self::InvalidParameter => "0201020",
            Self::SkuAlreadyExists => "0201021",
            Self::SkuNotFound => "0201022",
            Self::QuestionNotFound => "1301002",
        }
    }
    pub fn message(&self)->&'static str{
        match self {
            Self::InternalServerError=>"程序内部处理错误(可能是数据库、中间件等问题)",
            Self::InvalidParameter => "入参错误",
            Self::SkuAlreadyExists => "商品已存在",
            Self::SkuNotFound => "商品找不到",
            Self::QuestionNotFound => "找不到对应的问题记录",
        }
    }
    pub fn to_response_from_validation_errors<T>(&self,parameters: ValidationErrors,data:Option<T>)->ApiResponse<T>{
        let error_response = ApiResponse::error( 
            self.code().to_string(),
            self.message().to_string(),         
            Some(json!(parameters)),
            data,
        );
        return error_response;
    }

    pub fn to_response_from_hashmap<T>(&self,parameters: HashMap<String,&String>,data:Option<T>)->ApiResponse<T>{
        let error_response = ApiResponse::error(
            self.code().to_string(),
            self.message().to_string(),            
            Some(json!(parameters)),
            data,
        );
        return error_response;
    }

    pub fn to_response_from_empty_parameters<T>(&self,data:Option<T>)->ApiResponse<T>{
        let error_response = ApiResponse::error( 
            self.code().to_string(),
            self.message().to_string(),         
            None,
            data,
        );
        return error_response;
    }

    pub fn to_error_response_from_parameters<T: Clone>(&self,parameters: HashMap<String,&String>,data:Option<T>)->ErrorResponse<T>{
        let error_response = ErrorResponse{
            code:self.code().to_string(),
            msg:self.message().to_string(),
            success: false,
            error_parameters:Some(json!(parameters)),
            data
        };
        error_response
    }

    pub fn to_error_response_without_parameters<T:Clone>(&self,data:Option<T>)->ErrorResponse<T>{
        let error_response: ErrorResponse<T> = ErrorResponse{
            code:self.code().to_string(),
            msg:self.message().to_string(),
            success: false,
            error_parameters:None,
            data
        };
        error_response
    }
}