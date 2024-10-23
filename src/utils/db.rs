
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

pub async fn init_pool() -> Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    // 创建连接池
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[tokio::test]
    async fn test_init_pool() {
        let pool = init_pool().await.unwrap();
        assert_eq!(pool.is_closed(), false);
    }
}