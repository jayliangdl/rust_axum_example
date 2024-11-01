use sqlx::MySqlPool;
use crate::{model::db::sku::{Price, Sku}, utils::error::BusinessError};
use chrono::{Utc,DateTime};

use sqlx::QueryBuilder;
pub struct SkuDao;
impl SkuDao{
    pub async fn find_sku(pool:&MySqlPool, sku_code:&str)->Result<Option<Sku>,BusinessError> {
        let sku = sqlx::query_as::<_,Sku>(
            "select sku_code,name,description,create_date_time,update_date_time from co_sku where sku_code=?"
        )
        .bind(sku_code)
        .fetch_optional(pool)
        .await?;
        return Ok(sku);
    }

    pub async fn query_price_list_by_skucode(pool:&MySqlPool, sku_code:&str)->Result<Vec<Price>,BusinessError> {
        let price_list = sqlx::query_as::<_,Price>(
            "select * from co_sku_price where sku_code=?"
        )
        .bind(sku_code)
        .fetch_all(pool)
        .await?;        
        return Ok(price_list);
    }

    pub async fn insert_sku(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        sku: &Sku,
        current_time: DateTime<Utc>,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!(
       "INSERT INTO co_sku (sku_code, name, description, status, create_date_time, update_date_time) VALUES (?, ?, ?, ?, ?, ?)",
            sku.sku_code,
            sku.name,
            sku.description.clone().unwrap_or_else(|| "".to_string()),
            "1".to_string(),
            current_time,
            current_time
        );
        
        query.execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn insert_sku_log(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        sku_code: &str,
        content: serde_json::Value,
        current_time: DateTime<Utc>,
    ) -> Result<(), BusinessError> {
        // 执行插入操作，并忽略返回的结果
        let query = sqlx::query!(
            "INSERT INTO co_sku_log (sku_code, content, create_date_time) VALUES (?, ?, ?)",
            sku_code,
            content.to_string(),  // 将 JSON 转换为字符串
            current_time
        );
    
        // 使用 Transaction 的 execute 方法
        query.execute(&mut **transaction)
            .await?;
        Ok(())  // 返回成功的结果
    }

    pub async fn insert_sku_price_list(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        price_list: &Vec<Price>,
        current_time: DateTime<Utc>,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        for price in price_list{
            let query = sqlx::query!(
                "INSERT INTO co_sku_price (sku_code, sequence, price, start_date_time, end_date_time, create_date_time, update_date_time) VALUES (?, ?, ?, ?, ?, ?, ?)",
                price.sku_code,
                price.sequence,
                price.price,
                price.start_date_time,
                price.end_date_time,
                current_time,
                current_time
            );
            
            query.execute(&mut **transaction)
            .await?;
        }

        Ok(())
    }

    pub async fn update_sku(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        sku: &Sku,
        current_time: DateTime<Utc>,
    ) -> Result<(), BusinessError> {    
        let mut builder = QueryBuilder::<sqlx::MySql>::new("UPDATE co_sku SET ");

    // 使用 `SET` 的第一个字段时不需要添加逗号
    let mut first = true;

    // 更新 `update_date_time` 字段
    if first {
        first = false;
    } else {
        builder.push(", ");
    }
    builder.push("update_date_time = ");
    builder.push_bind(current_time);

    // 如果 `name` 有值，则添加到更新语句中
    let ref name = sku.name;
    builder.push(", name = ");
    builder.push_bind(name);
    
    let ref description = sku.description;
    builder.push(", description = ");
    builder.push_bind(description);

    // 未来添加更多可选字段时，可以按照以下方式继续添加
    /*
    if let Some(ref description) = sku.description {
        builder.push(", description = ");
        builder.push_bind(description);
    }

    if let Some(ref price) = sku.price {
        builder.push(", price = ");
        builder.push_bind(price);
    }
    // 依此类推...
    */

    // 添加 WHERE 子句
    builder.push(" WHERE sku_code = ");
    builder.push_bind(&sku.sku_code);

    // 构建并执行查询
    let query = builder.build();

    query.execute(&mut **transaction)
        .await?;

    Ok(())
    }
    

    pub async fn update_sku_price_list(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
        price_list: &Vec<Price>,
        current_time: DateTime<Utc>,
    )->Result<(), BusinessError>{

        // 执行插入操作，并忽略返回的结果
        for price in price_list{
            let query = sqlx::query!(
                "UPDATE co_sku_price SET price=?, start_date_time=?, end_date_time=?,create_date_time=?, update_date_time=? WHERE sku_code=? AND sequence=?",
                price.price,
                price.start_date_time,
                price.end_date_time,
                current_time,
                current_time,
                price.sku_code,
                price.sequence,
            );
            
            query.execute(&mut **transaction)
            .await?;
        }

        Ok(())
    }

}