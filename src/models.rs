

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

    #[derive(Serialize,Debug)]
    pub struct PageResponse<T>{
        #[serde(rename = "totalCount")]
        pub total_records:i64,
        #[serde(rename = "curPageNum")]
        pub current_pageno:i64,
        #[serde(rename = "pageSize")]
        pub page_size:i64,
        #[serde(rename = "totalPage")]
        pub total_pages:i64,
        #[serde(rename = "list")]
        pub data:Option<Vec<T>>,
    }

    impl <T> PageResponse<T>{
        pub fn new(total_records:i64,current_pageno:i64,page_size:i64,total_pages:i64,data:Option<Vec<T>>)->Self{
            PageResponse{
                total_records,
                current_pageno,
                page_size,
                total_pages,
                data,
            }
        }
    }

    #[derive(Serialize,Debug)]
    pub struct AppResponse<T>{
        pub code:String,
        pub msg:String,
        pub success:bool,
        pub data:T,
    }
    impl <T> AppResponse<T>{
        pub fn success(data:T)->Self{
            AppResponse{
                code:"0".to_string(),
                msg:"success".to_string(),
                success:true,
                data,
            }
        }
    }
}

use serde::{Deserialize, Serialize};
#[derive(Serialize,Debug,Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T>{
    SUCCESS{
        code:String,
        msg:String,
        success:bool,        
        #[serde(rename = "requestId")]
        request_id:String,
        data:Option<T>,
    },
    ERROR{
        code:String,
        msg:String,
        success:bool,
        request_id:String,
        error_parameters:Option<serde_json::Value>,
        data:Option<T>,
    },
}

impl<T> ApiResponse<T>{
    pub fn success(data:Option<T>)->ApiResponse<T>{
        ApiResponse::SUCCESS{
            code:"0".to_string(),
            msg:"success".to_string(),
            request_id:uuid::Uuid::new_v4().to_string(),
            success:true,
            data,
        }
    }

    pub fn error(code:String,msg:String,error_parameters:Option<serde_json::Value>,data:Option<T>)->ApiResponse<T>{
        ApiResponse::ERROR{
            code,
            msg,
            success:false,
            request_id:uuid::Uuid::new_v4().to_string(),
            error_parameters,
            data
        }
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
