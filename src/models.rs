
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

use serde::{Deserialize, Serialize};
#[derive(Serialize,Debug,Deserialize)]
#[serde(tag="result")]
pub enum ApiResponse<T>{
    SUCCESS{
        code:String,
        msg:String,
        #[serde(rename = "requestId")]
        request_id:String,
        success:bool,
        data:T
    },
    ERROR{
        error_code:String,
        error_message:String,
        error_parameters:Option<serde_json::Value>,
    },
}



impl<T> ApiResponse<T>{
    pub fn success(data:T)->ApiResponse<T>{
        ApiResponse::SUCCESS{
            code:"0".to_string(),
            msg:"success".to_string(),
            request_id:uuid::Uuid::new_v4().to_string(),
            success:true,
            data
        }
    }
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


pub mod nacos_models{
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    #[derive(Serialize, Deserialize, Clone,Debug)]
    pub struct NacosInstance {
        pub ip: String,
        pub port: u16,
        #[serde(rename = "serviceName")]
        pub service_name: String,
        pub weight: f64,
        pub enable: bool,
        pub healthy: bool,
        pub ephemeral: bool,
        pub metadata: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize, Clone,Debug)]
    pub struct NacosService {
        pub name: String,
        #[serde(rename = "groupName")]
        pub group_name: String,
        pub clusters: String,
        #[serde(rename = "namespaceId")]
        pub namespace_id: String,
        pub instances: Option<Vec<NacosInstance>>,
    }

    #[derive(Serialize, Clone)]
    pub struct DeregisterParams {
        pub ip: String,
        pub port: u16,
        #[serde(rename = "serviceName")]
        pub service_name: String,
        pub cluster: String,
        #[serde(rename = "namespaceId")]
        pub namespace_id: String,
    }
}


