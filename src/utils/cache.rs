use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;
use moka:: Expiry;
use std::time::Instant;
use tracing::instrument;
use tracing::trace;
use uuid::Uuid;
use std::sync::Arc;

use super::load_balance::LoadBalance;

#[derive(Clone,Debug)]
pub enum CacheType {
    Str(String),
    I32(i32),
    DateTime(String),
    Sku(Option<crate::model::db::sku::Sku>),
    LoadBalance(Arc<LoadBalance>),
    Question(Option<crate::model::cache::qa::Question>),
}

// 定义一个全局的、懒加载的缓存实例
pub static CACHE: Lazy<Cache<String, (Expiration,CacheType)>> = Lazy::new(|| {
    let expiry = MyExpiry;
    let eviction_listener = |key, _value, cause| {
        trace!("Evicted key {key}. Cause: {cause:?}");
    };
    Cache::builder()
        .max_capacity(1000)
        .expire_after(expiry)
        .eviction_listener(eviction_listener)
        // .time_to_live(Duration::from_secs(300))
        .build()
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Expiration {
    /// The value never expires.
    Never,
    /// The value expires after a short time. (5 seconds in this example)
    AfterShortTime,
    /// The value expires after a long time. (15 seconds in this example)
    AfterLongTime,   
    Second5, 
}

impl Expiration {
    /// Returns the duration of this expiration.
    pub fn as_duration(&self) -> Option<Duration> {
        match self {
            Expiration::Never => None,
            Expiration::AfterShortTime => Some(Duration::from_secs(5)),
            Expiration::AfterLongTime => Some(Duration::from_secs(1500)),
            Expiration::Second5=>Some(Duration::from_secs(5)),
        }
    }
}

#[derive(Debug)]
pub struct MyExpiry;


impl Expiry<String, (Expiration, CacheType)> for MyExpiry {
    /// Returns the duration of the expiration of the value that was just
    /// created.
    #[instrument(name = "cache", fields(request_id = %Uuid::new_v4()))]
    //返回值在缓存中的有效时间
    fn expire_after_create(
        &self,
        _key: &String,
        value: &(Expiration, CacheType),
        _current_time: Instant,
    ) -> Option<Duration> {
        let duration = value.0.as_duration();
        tracing::trace!("MyExpiry: expire_after_create called with key {_key} and value {value:?}. Returning {duration:?}.");
        duration
    }

    /// 设置更新后的缓存项的过期时间
    #[instrument(name = "cache.expire_after_update", skip_all)]
    fn expire_after_update(
        &self,
        _key: &String,
        value: &(Expiration, CacheType),
        _current_time: Instant,
        _druation: Option<Duration>,
    ) -> Option<Duration> {
        let duration = value.0.as_duration();
        tracing::trace!(
            "MyExpiry: expire_after_update called with key `{}` and value {:?}. Returning {:?}.",
            _key, value, duration
        );
        duration
    }
}

pub mod key{
    pub fn get_service_list_key(service_name_clone:String) -> String {
        format!("{}:{}","nacos_service_list:",&service_name_clone)
    }
    pub fn get_service_instance_key(service_name:String)->String{
        let key=format!("load_balance_{}",&service_name);
        key
    }
}

