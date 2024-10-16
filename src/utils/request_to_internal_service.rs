use hyper::header::CONTENT_TYPE;
use crate::utils::cache::{CACHE,CacheType};
use crate::utils::cache::key::get_service_instance_key;
use tracing::{info, instrument, error};
use uuid::Uuid;
use hyper::{Body, Client, Method, Request, StatusCode};
use super::load_balance::Instance;
enum ResponseResult{
    Success(String),
    Retry(),
    Error(String),
}
#[instrument(name="request",fields(request_id = %Uuid::new_v4()))]
pub async fn request(service_name:&String,url:&String,method:&Method,body:&String) -> Result<String,String> {
    let key: String = get_service_instance_key(service_name.to_string());
    let (_,cache_type) = CACHE.get(&key)
        .ok_or("Failed to get SERVICE_URL".to_string())?;
    match cache_type {       
        CacheType::LoadBalance(service) => {  
            const MAX_RETRIES: usize = 3;
            for attempt in 1..=MAX_RETRIES {
                let response_result = req(service.next().await,method,body, &attempt, &MAX_RETRIES).await;  
                match response_result{
                    ResponseResult::Success(body)=>{
                        return Ok(body);
                    },
                    ResponseResult::Retry() => {
                        info!("再次重试请求，目前次数:{}",&attempt);
                        continue;
                    },
                    ResponseResult::Error(e) => {
                        error!("请求失败:{}",e);
                        return Err(e);
                    }
                }
            }
            return Err(format!("已达到重试上线"));
        },
        _ => {
            return Err("Failed to get SERVICE_URL".to_string());
        }        
    };
}

async fn req(instance:Option<Instance>,method:&Method,body:&String, attempt: &usize,max_retries:&usize) -> ResponseResult{
    if let Some(instance) = instance{
        let ip = instance.ip.clone();
        let port = instance.port.clone();
        let header_url = format!("http://{}:{}",ip,port);
        let uri = format!("{}/frontend/find_sku", header_url);
        let uri = uri.parse::<hyper::Uri>();
        const BASE_DELAY_MS: u64 = 100; // 基础延迟时间，毫秒
        if let Err(e) = uri{            
            return ResponseResult::Error(format!("Failed to parse URI {}",e)); 
        }else{
            let uri = uri.unwrap();
            tracing::info!("uri:{}",uri);
            let req= Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body.clone()))
            .map_err(|e| format!("Failed to create request: {}", e));
            if let Err(e) = req{
                error!("请求失败，失败原因是: {}",e);
                return ResponseResult::Error(format!("{}",e)); 
            }else{
                let req = req.unwrap();
                let client = Client::new();
                let res = client.request(req).await;
                match res{
                    Ok(res)=>{                    
                        // 如果返回的状态码是 503，则重试            
                        if res.status()==StatusCode::SERVICE_UNAVAILABLE{
                            instance.add_failed_counter();
                            // 如果重试次数小于最大重试次数，则延迟一段时间后重试
                            if *attempt < *max_retries {
                                // 重试延迟时间 = 基础延迟时间 * 2^尝试次数
                                let delay = BASE_DELAY_MS * 2u64.pow(*attempt as u32);
                                // 等待一段时间后重试
                                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                                // 返回类型是重试
                                return ResponseResult::Retry();
                            }else{
                                return ResponseResult::Error("请求重试已达上限".to_string());                            
                            }
                        }else{                            
                            let body = hyper::body::to_bytes(res.into_body()).await;
                            match body{
                                Ok(body)=>{
                                    let body = String::from_utf8(body.to_vec()).unwrap();
                                    return ResponseResult::Success(body);
                                },
                                Err(e)=>{
                                    instance.add_failed_counter();
                                    return ResponseResult::Error(format!("Failed to get response body: {}",e)); 
                                }
                            }
                        }
                    },
                    Err(e) => {
                        error!("请求失败，失败原因是: {}",e);
                        instance.add_failed_counter();
                        if *attempt < *max_retries {                            
                            // 重试延迟时间 = 基础延迟时间 * 2^尝试次数
                            let delay = BASE_DELAY_MS * 2u64.pow(*attempt as u32);
                            // 等待一段时间后重试
                            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                            // 返回类型是重试
                            return ResponseResult::Retry();
                        }else{
                            return ResponseResult::Error("请求重试已达上限".to_string());                            
                        }
                    }
                }
            };
        }                
    }else{
        error!("请求失败");
        return ResponseResult::Error(format!("Failed to get SERVICE_URL")); 
    }
}
