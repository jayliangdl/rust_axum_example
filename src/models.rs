
pub mod request_models{
    use validator::Validate;
    use serde::{Deserialize, Serialize};

    #[derive(serde::Deserialize, Serialize,Validate,Clone,Debug)]
    pub struct CreateUser {
        pub username: String,
        pub age: u8,
    }

    #[derive(Deserialize)]
    pub struct MockTimeout {
        pub sleep_seconds: u64,
    }
    
    #[derive(Deserialize)]
    pub struct HealthCheck{
        pub text: String,
    }
    #[derive(Deserialize)]
    pub struct EnvVariable{
        pub text: String,
    }
}

pub mod response_models{
    use serde::Serialize;
    #[derive(Serialize,Debug)]
    pub struct User {
        pub id: u64,
        pub username: String
    }

    #[derive(Serialize,Debug)]
    pub struct HealthCheck{
        pub text: String,
    }

    #[derive(Serialize,Debug)]
    pub struct EnvVariable{
        pub value: String,
    }
}

use serde::Serialize;
#[derive(Serialize,Debug)]
#[serde(tag="result")]
pub enum ApiResponse<T>{
    SUCCESS{
        data:T
    },
    ERROR{
        error_code:String,
        error_message:String,
        error_parameters:Option<serde_json::Value>,
    },
}

pub struct  ErrorResponse{
    pub error_code:String,
    pub error_message:String,
    pub error_parameters:Option<serde_json::Value>,
}

impl ErrorResponse{
    pub fn to_api_response<T>(&self)->ApiResponse<T>{
        let api_response = ApiResponse::ERROR { 
            error_code: self.error_code.to_string(), 
            error_message: self.error_message.to_string(), 
            error_parameters: self.error_parameters.clone()
        };
        api_response

    }
}


