use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{de, Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::model::db::sku::Price as DbPrice;
use validator::Validate;
use bigdecimal::ToPrimitive;

#[derive(serde::Deserialize, Serialize,Validate,Clone)]
pub struct Price {
    // 价格序列号
    pub sequence: i32,
    #[serde(serialize_with = "serialize_big_decimal")]
    pub price: BigDecimal,
    #[serde(deserialize_with = "deserialize_custom_datetime")]
    pub start_date_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_custom_datetime")]
    pub end_date_time: DateTime<Utc>,
}

// 实现一个into_db_price方法，将Price转换为DbPrice
impl Price {
    pub fn into_db_price(&self, sku_code:&str) -> DbPrice {
        DbPrice {
            sku_code: sku_code.to_string(),
            sequence: self.sequence,    
            price: self.price.clone(),
            start_date_time: self.start_date_time,
            end_date_time: self.end_date_time,
            create_date_time: None,
            update_date_time: None,
        }
    }
}

// 自定义 Debug 实现
impl std::fmt::Debug for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 使用 to_string() 来获取标准的字符串表示
        let price_str = self.price.to_string();
        f.debug_struct("Price")
            .field("sequence", &self.sequence)
            .field("price", &price_str)  // 使用自定义的字符串表示
            .field("start_date_time", &self.start_date_time)
            .field("end_date_time", &self.end_date_time)
            .finish()
    }
}


//转换字符串格式 "YYYY-MM-DD HH:MM:SS" 到Utc时间
fn deserialize_custom_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the input as a String
    let s = String::deserialize(deserializer)?;

    // Define the expected datetime format
    let naive =
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(de::Error::custom)?;

    // Convert NaiveDateTime to DateTime<Utc>
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
}


// 自定义序列化器，将 BigDecimal 转换为 f64 或 i64 以保留原始小数位数
fn serialize_big_decimal<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{   
    //判断value的小数位数是否为0
    let scale = value.digits();
    if scale == 0 {
        if let Some(int_value) = value.to_i64() {
            serializer.serialize_i64(int_value)
        } else{
            Err(serde::ser::Error::custom(
                "Failed to convert BigDecimal to number",
            ))
        }
    }else{
        if let Some(float_value) = value.to_f64() {
            serializer.serialize_f64(float_value)
        } else {
            Err(serde::ser::Error::custom(
                "Failed to convert BigDecimal to number",
            ))
        }
    }
}

