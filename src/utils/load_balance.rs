use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use tokio::sync::RwLock;
use std::error::Error;
use tracing::error;
use hyper::{Body, Client, Method, Request};
use serde_json::Value;
use tokio::time::Duration;
use std::env; 
use std::sync::Arc;
use crate::utils::cache::{CacheType,Expiration,CACHE};
use crate::services_dependence::get_services_dependence_list;

const REENABLE_DELAY_MS : u64 = 5000;//对于不可用的实例，超过这个时间，重置失败次数
const MAX_FAILED_TIMES:usize = 15;//最大失败次数，超过这个次数，就不再尝试访问这个实例
const REFRESH_INSTANCES_INTERVAL:u64 = 3000;//每3秒从Nacos中刷新一次实例列表

#[derive(Debug)]
pub struct Instance{
    pub ip:String,
    pub port:u16,
    pub failed_counter:Arc<AtomicUsize>,
    pub pod_is_available:Arc<AtomicBool>,
    pub last_failed_time:Arc<AtomicU64>,
}
impl Clone for Instance{
    fn clone(&self)->Self{
        Instance{
            ip:self.ip.clone(),
            port:self.port.clone(),
            failed_counter:self.failed_counter.clone(),
            pod_is_available:self.pod_is_available.clone(),
            last_failed_time:self.last_failed_time.clone(),
        }
    }
}
impl PartialEq for Instance{
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip && self.port == other.port
    }
}
impl Eq for Instance{}

impl Instance{
    pub fn new(ip:String,port:u16)->Self{
        Instance{
            ip,
            port,
            failed_counter:Arc::new(AtomicUsize::new(0)),
            pod_is_available:Arc::new(AtomicBool::new(true)),
            last_failed_time:Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn add_failed_counter(&self){
        self.failed_counter.fetch_add(1, Ordering::SeqCst);
        if self.pod_is_available.load(Ordering::SeqCst){
            if self.failed_counter.load(Ordering::SeqCst) > MAX_FAILED_TIMES {
                self.pod_is_available.store(false, Ordering::SeqCst);
                self.last_failed_time.store(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), Ordering::SeqCst);
            }
        }
    }

    //重置失败次数
    pub fn reset_failed_counter(&self){
        if self.failed_counter.load(Ordering::SeqCst) > 0{
            self.failed_counter.store(0, Ordering::SeqCst);
            self.pod_is_available.store(true, Ordering::SeqCst);
            self.last_failed_time.store(0, Ordering::SeqCst);
        }
    }
}

#[derive(Debug)]
pub struct LoadBalance{
    pub service_name:String,
    pub instances:RwLock<Vec<Instance>>,
    pub counter:AtomicUsize,
    pub failed_counter:AtomicUsize,
}

impl LoadBalance{
    fn new(service_name:String,instances:Vec<Instance>)->Self{
        LoadBalance{
            service_name,
            instances:RwLock::new(instances),
            counter: AtomicUsize::new(0),
            failed_counter: AtomicUsize::new(0),
        }
    }

    //获取下一个实例
    pub async fn next(&self)->Option<Instance>{
        let instances = self.instances.read().await;
        //过滤掉不可用的实例
        let instances = instances.iter()
            .filter(|instance|instance.pod_is_available.load(Ordering::SeqCst))
            .map(|instance|instance.clone())
            .collect::<Vec<Instance>>();
        let len = instances.len();
        if instances.is_empty() || len==0{
            None
        }else{
            //轮询获取实例
            let index = self.counter.fetch_add(1, Ordering::SeqCst) % len;
            if let Some(instance) = instances.get(index){
                return Some(instance.clone());
            }
            error!("所有实例都不可用");
            None
        }
    }

    //更新实例列表
    pub async fn update_instances(&self, new_instances:Vec<Instance>){
        let mut instances = self.instances.write().await;
        *instances = new_instances;
    }

    //移除实例
    pub async fn remove_instance(&self, instance_to_remove:&Instance){
        let mut instances = self.instances.write().await;
        instances.retain(|instance|instance_to_remove != instance);
    }

    //监控实例状态，检查是否有不可用实例可以重置。如果实例不可用，且超过REENABLE_DELAY_MS时间，则重置失败次数
    pub async fn reenable_instances(&self){
        //每2秒检查一次实例状态
        loop{
            //检查实例状态(仅检查不可用的实例)
            let instances = self.instances.read().await;
            for instance in instances.iter(){
                //如果实例不可用，且超过了REENABLE_DELAY_MS时间，则重置失败次数
                if !instance.pod_is_available.load(Ordering::SeqCst){
                    let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
                    let last_failed_time = instance.last_failed_time.load(Ordering::SeqCst);
                    if current_time - last_failed_time > REENABLE_DELAY_MS{
                        instance.reset_failed_counter();
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(2000)).await;
        }
    }
}
//从Nacos中获取服务实例列表
async fn get_service_instances(nacos_url: &str, service_name: &str, group_name: &str, namespace_id: &str) -> Result<LoadBalance, Box<dyn Error>> {
    let client = Client::new();
    let uri = format!("{}/nacos/v1/ns/instance/list?serviceName={}&groupName={}&namespaceId={}", nacos_url, service_name, group_name, namespace_id)
        .parse::<hyper::Uri>()
        .map_err(|e| format!("Failed to parse URI: {}", e))?;
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())?;
    let res = client.request(req).await?;
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8_lossy(&body_bytes);    
    let services: Value = serde_json::from_str(&body_str)?;
    if let Value::Object(map) = services.clone() {
        let hash_map: HashMap<String, Value> = map.into_iter().collect();        
        let mut instances = vec![];
        if hash_map.contains_key("hosts"){
            let hosts = hash_map.get("hosts").unwrap().as_array().unwrap();
            for host in hosts {
                let ip =host.get("ip").unwrap_or(&Value::String("".to_string())).as_str().unwrap().to_string();
                let port = host.get("port").unwrap_or(&Value::Number(0.into())).as_u64().unwrap() as u16;
                let instance = Instance::new(ip, port);
                instances.push(instance);
            }
        }
        let load_balance: LoadBalance = LoadBalance::new(service_name.to_string().clone(), instances);
        Ok(load_balance)
    }else{
        Err("Failed to get service instances".into())
    }
    
}

//从nacos中获取其他微服务的实例列表，并添加到缓存中（每3秒从nacos中刷新一次）
pub async fn fetch_load_balance_from_nacos(){
    let nacos_url = env::var("NACOS_URL").unwrap_or_else(|_| "http://localhost:8848".to_string());
    let group_name = env::var("GROUP_NAME").unwrap_or_else(|_| "DEFAULT_GROUP".to_string());
    let namespace_id = env::var("NAMESPACE_ID").unwrap_or_else(|_| "public".to_string());
    let services_dependence_list = get_services_dependence_list();    
    async fn refresh_cache(nacos_url:&str,services_dependence_list: &Vec<String>,group_name: &str,namespace_id: &str){
        for service_name in services_dependence_list{
            let load_balance = get_service_instances(nacos_url,service_name,group_name,namespace_id).await;
            // .expect("Failed to start Nacos manager");
            match load_balance{
                Ok(load_balance)=>{
                    let key=format!("load_balance_{}",&service_name);
                    let arc_lb = Arc::new(load_balance);
                    let lb_with_cache_type = CacheType::LoadBalance(arc_lb.clone());
                    CACHE.insert(key, (Expiration::AfterLongTime,lb_with_cache_type));
                    tokio::spawn(
                async move{
                    arc_lb.clone().reenable_instances().await;
                }
            );
                },
                Err(e)=>{
                    error!("Failed to get service instances: {}",e);
                    continue;
                }
            }
            
        }
    }

    let _ = refresh_cache(&nacos_url,&services_dependence_list,&group_name,&namespace_id).await;
    tokio::spawn(
        async move{
            loop{
                refresh_cache(&nacos_url, &services_dependence_list, &group_name, &namespace_id).await;
                let _ = tokio::time::sleep(Duration::from_millis(REFRESH_INSTANCES_INTERVAL)).await;
            }
        }
    );
}

// 定义一个全局的、懒加载的缓存实例
// pub static CACHE: Lazy<RwLock<Option<LoadBalance>>> = Lazy::new(|| RwLock::new(None));

// pub async fn initialize_cache() {
//     let nacos_url = env::var("NACOS_URL").unwrap_or_else(|_| "http://localhost:8848".to_string());
//     let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "my_rust_service".to_string());
//     let group_name = env::var("GROUP_NAME").unwrap_or_else(|_| "DEFAULT_GROUP".to_string());
//     let namespace_id = env::var("NAMESPACE_ID").unwrap_or_else(|_| "public".to_string());

//     if let Ok(load_balance) = get_service_instances(&nacos_url, &service_name, &group_name, &namespace_id).await {
//         let lb = LoadBalance {
//             service_name: service_name.to_string().clone(),
//             instances: load_balance.instances,
//             counter: AtomicUsize::new(0),
//         };
//         let mut cache = CACHE.write().unwrap();
//         *cache = Some(lb);
//     } else {
//         error!("Failed to initialize cache");
//     }
// }

//应用启动时，获取本应用（作为客户端）关心的服务器端服务，启动一个独立的线程，每10秒从Nacos中获取一次服务实例列表
// pub async fn start_nacos_watch(nacos_url: &str, group_name: &str, namespace_id: &str) -> Result<(), Box<dyn Error>> {
//     let services_dependence_list = get_services_dependence_list();
//     let nacos_url_clone = nacos_url.to_string();
//     let group_name_clone = group_name.to_string();
//     let namespace_id_clone = namespace_id.to_string();
//     tokio::spawn(async move {
//         loop {
//             for service_name in &services_dependence_list {
//                 let service_name_clone = service_name.clone();                
//                 if let Ok(load_balance) = get_service_instances(&nacos_url_clone, &service_name_clone, &group_name_clone, &namespace_id_clone).await {
//                     //将获取到的服务实例列表存储到本地缓存中
//                     let cache_type_with_load_balance: CacheType = CacheType::LoadBalance(load_balance);
//                     // info!("Service instances: {:?}", services.clone());
//                     let key = key::get_service_list_key(service_name_clone);
//                     CACHE.insert(key.clone(), (Expiration::Second5,cache_type_with_load_balance));
//                 } else {
//                     error!("Failed to get service instances");
//                 }
//             }
//             sleep(Duration::from_secs(2)).await;
//         }
//     });
//     Ok(())
// }

