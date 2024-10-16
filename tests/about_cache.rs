extern crate rust_axum_example;

#[cfg(test)]
mod test{
    use rust_axum_example::utils::cache::CACHE;
    use tracing::{info, Level};    
    use tracing_subscriber;
    use std::sync::Once;
    use rust_axum_example::utils::cache::{CacheType,Expiration};
    use chrono::prelude::*;
    static INIT: Once = Once::new();
    fn init_tracing(){
        INIT.call_once(||{
            tracing_subscriber::fmt().with_max_level(Level::INFO).init();
        });
    }
    #[tokio::test]    
    async fn test1()->Result<(),Box<dyn std::error::Error>>{
        init_tracing();
        let handle = tokio::spawn(            
            async move {
                let now = Local::now();
                let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
                CACHE.insert("now".to_string(), (Expiration::AfterShortTime,CacheType::Str(format_time.to_string())));
                loop {                    
                    info!("now::{:?}",CACHE.get("now").unwrap());
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    let now = Local::now();
                    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
                    CACHE.insert("now".to_string(), (Expiration::AfterShortTime,CacheType::Str(format_time.to_string())));
                }
            }
        );

        handle.await?;
        Ok(())
    }
}