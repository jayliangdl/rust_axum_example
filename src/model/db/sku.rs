use sqlx::prelude::FromRow;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};


#[derive(FromRow,Clone,Debug)]
#[allow(dead_code)]
pub struct Sku{
    pub sku_code:String,
    pub name:String,
    pub description:Option<String>, 
}


#[derive(FromRow)]
#[allow(dead_code)]
pub struct Price {
    pub sku_code: String,
    pub sequence: i32,
    pub price: BigDecimal,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
    pub create_date_time: Option<DateTime<Utc>>,
    pub update_date_time: Option<DateTime<Utc>>,
}