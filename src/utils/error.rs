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
}

impl ErrorCode{
    pub fn code(&self)->&'static str{
        match self {
            Self::InternalServerError=>"500",
            Self::InvalidParameter => "0201020",
            Self::SkuAlreadyExists => "0201021",
            Self::SkuNotFound => "0201022",
        }
    }
    pub fn message(&self)->&'static str{
        match self {
            Self::InternalServerError=>"程序内部处理错误(可能是数据库、中间件等问题)",
            Self::InvalidParameter => "入参错误",
            Self::SkuAlreadyExists => "商品已存在",
            Self::SkuNotFound => "商品找不到",
        }
    }
    pub fn to_response_from_validation_errors<T>(&self,parameters: ValidationErrors)->ApiResponse<T>{
        // let error_response: ErrorResponse = ErrorResponse{
        //     error_code:self.code().to_string(),
        //     error_message:self.message().to_string(),
        //     error_parameters:Some(json!(parameters)),
        // };
        let error_response = ApiResponse::ERROR {
            error_code: self.code().to_string(),
            error_message: self.message().to_string(),
            error_parameters: Some(json!(parameters))
        };
        return error_response;
    }

    pub fn to_response_from_hashmap<T>(&self,parameters: HashMap<String,&String>)->ApiResponse<T>{
        let error_response = ApiResponse::ERROR {
            error_code: self.code().to_string(),
            error_message: self.message().to_string(),
            error_parameters: Some(json!(parameters))
        };
        return error_response;
    }

    pub fn to_response_from_empty_parameters<T>(&self)->ApiResponse<T>{
        let error_response = ApiResponse::ERROR {
            error_code: self.code().to_string(),
            error_message: self.message().to_string(),
            error_parameters: None
        };
        return error_response;
    }

    pub fn to_error_response_from_parameters(&self,parameters: HashMap<String,&String>)->ErrorResponse{
        let error_response = ErrorResponse{
            error_code:self.code().to_string(),
            error_message:self.message().to_string(),
            error_parameters:Some(json!(parameters)),
        };
        error_response
    }

    pub fn to_error_response_without_parameters(&self)->ErrorResponse{
        let error_response = ErrorResponse{
            error_code:self.code().to_string(),
            error_message:self.message().to_string(),
            error_parameters:None
        };
        error_response
    }
}