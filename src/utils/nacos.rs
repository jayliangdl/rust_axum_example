use hyper::{Body, Client, Method, Request};
use hyper::header::CONTENT_TYPE;
use serde_json::json;
use tracing::{trace,info,error};
use std::error::Error;
use std::vec;
use tokio::sync::oneshot;
use std::collections::HashMap;
use uuid::Uuid;
use tokio::time::{sleep, Duration};
use crate::models::nacos_models::{DeregisterParams, NacosService};
// 向Nacos注册服务
async fn register_service_hyper(nacos_url: &str, service: &NacosService) -> Result<(), Box<dyn Error>> {
    //检查service是否为空，为空则返回错误
    if service.instances.is_none() {
        error!("Service instances is empty");
        return Err("Service instances is empty".into());
    }
    let instances = service.instances.as_ref().unwrap();
    //检查service的instances是否起码有一个元素，没有则返回错误
    if instances.len() < 1 {
        error!("Service instances must have at least one element");
        return Err("Service instances must have at least one element".into());
    }
    //检查service的instances每一个元素是否有ip和port，没有则返回错误
    for instance in instances {
        if instance.ip.is_empty() {
            error!("Instance IP is empty");
            return Err("Instance IP is empty".into());
        }
        if instance.port == 0 {
            error!("Instance port is 0");
            return Err("Instance port is 0".into());
        }
    }

    //循环service的instances每一个元素
    for instance in instances {
        let client = Client::new();
        let service_name = &service.name;
        let ip = &instance.ip;
        let port = &instance.port;

        let uri = format!("{}/nacos/v1/ns/instance?serviceName={}&ip={}&port={}", nacos_url, service_name, ip, port)
            .parse::<hyper::Uri>()
            .map_err(|e| format!("Failed to parse URI: {}", e))?;
        info!("uri: {}", uri);
        let json_body = serde_json::to_string(service)?;
        info!("Registering service: {}", json_body);
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(json_body))?;

        let res = client.request(req).await?;

        if res.status().is_success() {
            let instances = service.instances.as_ref().unwrap();
            info!("Service {} on port {} registered successfully", service.name, &instances[0].port);
        } else {
            let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
            error!("Failed to register service: {}", String::from_utf8_lossy(&body_bytes));
        }
    }
    Ok(())
}

// 从Nacos注销服务
async fn deregister_service_hyper(nacos_url: &str, params: &DeregisterParams) -> Result<(), Box<dyn Error>> {

    //检查参数是否为空，为空则返回错误
    if params.service_name.is_empty() {
        error!("Service name is empty");
        return Err("Service name is empty".into());
        
    }
    if params.ip.is_empty() {
        error!("IP is empty");
        return Err("IP is empty".into());
    }
    if params.port == 0 {
        error!("Port is 0");
        return Err("Port is 0".into());
    }
    let service_name = &params.service_name;
    let ip = &params.ip;
    let port = &params.port;

    let client = Client::new();

    let uri = format!("{}/nacos/v1/ns/instance?serviceName={}&ip={}&port={}", nacos_url, service_name, ip, port)
        .parse::<hyper::Uri>()
        .map_err(|e| format!("Failed to parse URI: {}", e))?;
    
    let form_body = serde_urlencoded::to_string(params)?;
    info!("Deregistering service: {}", &form_body);
    let req = Request::builder()
    .method(Method::DELETE)
    .uri(uri)
    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    .body(Body::from(form_body))?;

    let res = client.request(req).await?;
    if res.status().is_success() {
        info!("Service deregistered successfully");
    } else {
        let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
        error!("Failed to deregister service: {}", String::from_utf8_lossy(&body_bytes));
    }
    Ok(())
}

// 发送心跳
async fn send_heartbeat_hyper(
    nacos_url: &str,
    service_name: &str,
    ip: &str,
    port: u16,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let uri = format!("{}/nacos/v1/ns/instance/beat?serviceName={}&ip={}&port={}", nacos_url, &service_name, &ip, &port)
        .parse::<hyper::Uri>()
        .map_err(|e| format!("Failed to parse URI: {}", e))?;

    let params: serde_json::Value = json!({
        "serviceName": service_name,
        "ip": ip,
        "port": port,
        // 可以根据需要添加更多参数，如 beatInterval, metadata 等
    });

    let json_body = serde_json::to_string(&params)?;
    let req = Request::builder()
        .method(Method::PUT)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(json_body))?;

    let res = client.request(req).await?;

    if res.status().is_success() {
        trace!("Heartbeat sent for service {} on port {}", service_name, port);
    } else {
        let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
        error!("Failed to send heartbeat: {}", String::from_utf8_lossy(&body_bytes));
    }

    Ok(())
}

// 管理 Nacos 的注册、心跳和注销
pub async fn start_nacos(
    nacos_url: &str,
    service_name: &str,
    group_name: &str,
    namespace_id: &str,
    ip: &str,
    port: u16,
) -> Result<NacosHandle, Box<dyn Error>> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // 创建实例 ID 和元数据
    let instance_id = Uuid::new_v4().to_string();
    let mut metadata = HashMap::new();
    metadata.insert("instance_id".to_string(), instance_id.clone());

    // 创建 NacosInstance 和 NacosService
    let instance = crate::models::nacos_models::NacosInstance {
        ip: ip.to_string(),
        port,
        service_name: service_name.to_string(),
        weight: 1.0,
        enable: true,
        healthy: true,
        ephemeral: true,
        metadata: Some(metadata),
    };

    let service = NacosService {
        name: service_name.to_string(),
        group_name: group_name.to_string(),
        clusters: "DEFAULT".to_string(),
        namespace_id: namespace_id.to_string(),
        instances: Some(vec![instance.clone()]),
    };

    // 创建 DeregisterParams
    let deregister_params = crate::models::nacos_models::DeregisterParams {
        ip: ip.to_string(),
        port,
        service_name: service_name.to_string(),
        cluster: "DEFAULT".to_string(),
        namespace_id: namespace_id.to_string(),
    };

    let nacos_url_clone = nacos_url.to_string();
    let service_clone = service.clone();
    let deregister_params_clone = deregister_params.clone();

    // 启动一个异步任务来管理 Nacos
    let join_handle = tokio::spawn(async move {
        // 注册服务
        if let Err(e) = register_service_hyper(&nacos_url_clone, &service_clone).await {
            error!("Error registering service on port {}: {}", port, e);
            // 可以在这里实现重试逻辑
            return;
        }

        // 声明 shutdown_rx 为可变的
        let mut shutdown_rx = shutdown_rx;

        // 心跳循环
        loop {
            tokio::select! {
                _ = sleep(Duration::from_secs(10)) => {
                    if let Some(instances) = service_clone.instances.as_ref() {
                        for instance in instances {
                            let nacos_url_clone = nacos_url_clone.clone();
                            let service_name = service_clone.name.clone();
                            let ip = instance.ip.clone();
                            let port = instance.port;
                            tokio::spawn(async move {
                                if let Err(e) = send_heartbeat_hyper(&nacos_url_clone, &service_name, &ip, port).await {
                                    error!("Error sending heartbeat for port {}: {}", port, e);
                                }
                            });
                        }
                    }
                    
                }
                _ = &mut shutdown_rx => {
                    // 接收到注销信号，执行注销
                    info!("Shutdown signal received, deregistering service on port {}", port);
                    if let Err(e) = deregister_service_hyper(&nacos_url_clone, &deregister_params_clone).await {
                        error!("Error deregistering service on port {}: {}", port, e);
                    }
                    break;
                }
            }
        }
    });

    Ok(NacosHandle {
        shutdown: shutdown_tx,
        join_handle,
    })
}

// 定义一个结构体来管理 Nacos 的注销信号和任务句柄

pub struct NacosHandle {
    pub shutdown: oneshot::Sender<()>,
    pub join_handle: tokio::task::JoinHandle<()>,
}


