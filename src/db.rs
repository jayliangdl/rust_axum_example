
use sqlx::mysql::MySqlPoolOptions;

pub async fn init_pool(database_url:&str) -> Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {

    // 创建连接池
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}