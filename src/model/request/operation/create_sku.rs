use std::collections::HashMap;
use serde::Deserialize;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use validator::{Validate, ValidationError};
use crate::model::db::sku::{
    Price as DbPrice,
    Sku as DbSku
};
use crate::model::request::operation::price::Price;
use crate::models::ApiResponse;
use crate::utils::error::ErrorCode;
use sqlx::MySqlPool;
use crate::model::response::operation::create_sku::CreateSku as ResponseCreateSku;
use crate::models::ErrorResponse;
use crate::dao::sku_dao::SkuDao;

#[derive(Deserialize, Debug, Validate)]
pub struct CreateSku {
    #[validate(length(min = 1, message = "sku_code不能为空"))]
    // #[validate(custom = "validate_alreay_exists_sku")]
    pub sku_code: String,
    #[validate(length(min = 1, message = "name不能为空"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(custom = "validate_price_list")]
    pub price_list: Vec<Price>,
}

fn validate_price_list(value: &Vec<Price>) -> Result<(), ValidationError> {
    if value.len() == 0 {
        return Err(ValidationError::new("价格列表不能为空"));
    }
    // let mut value = value.clone();
    // //先对价格列表进行排序（以价格序号为准，从小到大）
    // value.sort_by(|a, b| a.sequence.cmp(&b.sequence));
    for i in 0..value.len() - 1 {
        //校验价格列表中的上一次结束时间是否小于下一次开始时间
        if value[i].end_date_time > value[i + 1].start_date_time {
            return Err(ValidationError::new("价格列表中的上一次结束时间不能大于下一次开始时间"));
        }
        //价格序号不能重复
        if value[i].sequence == value[i + 1].sequence {
            return Err(ValidationError::new("价格序号不能重复"));
        }
        //价格序号必须是从小到大
        if value[i].sequence > value[i + 1].sequence {
            return Err(ValidationError::new("价格序号必须是从小到大"));
        }
    }
    for price in value {
        if price.start_date_time > price.end_date_time {
            return Err(ValidationError::new("开始时间不能大于结束时间"));
        }
        if price.price < BigDecimal::from(0) {
            return Err(ValidationError::new("价格不能是负数"));
        }
        //是一个符合要求的价格，例如小数不能超过两位
        if let Some(float_value) = price.price.to_f64() {
            let string_value = float_value.to_string();
            if string_value.contains(".") {
                    let price2 = string_value.split(".").collect::<Vec<&str>>();
                    if price2[1].len() > 2 {
                        return Err(ValidationError::new("价格小数位数不能超过2位"));
                    }
                }
        }
        //价格序号只能是大于等于1
        if price.sequence < 1 {
            return Err(ValidationError::new("价格序号只能是大于等于1"));
        }
    }
    Ok(())
}

impl CreateSku{
    pub fn into_db_sku(&self)->DbSku{
        let mut db_price_list = vec![];
        for price in &self.price_list{
            let db_price = DbPrice{
                sku_code:self.sku_code.clone(),
                sequence:price.sequence,
                price:price.price.clone(),
                start_date_time:price.start_date_time,
                end_date_time:price.end_date_time,
                create_date_time:None,
                update_date_time:None,
            };
            db_price_list.push(db_price);
        };
        let db_sku = DbSku{
            sku_code:self.sku_code.clone(),
            name:self.name.clone(),
            description:self.description.clone(),
        };
        db_sku
    }

    pub async fn custom_validate(&self,pool:&MySqlPool)->Result<(), ApiResponse<ResponseCreateSku>>{
        if let Err(errors) = self.validate(){
            let e: ApiResponse<ResponseCreateSku> = ErrorCode::InvalidParameter.to_response_from_validation_errors::<ResponseCreateSku>(errors);
            return Err(e);
        }
        if let Err(error_response) = self.validate_alreay_exists_sku(pool).await
        { 

            let api_response = error_response.to_api_response();
            return Err(api_response);
        }
        Ok(())
    }
    
    async fn validate_alreay_exists_sku(&self,pool:&MySqlPool) -> Result<(),ErrorResponse> {
        if let Ok(result) = SkuDao::find_sku(pool, &self.sku_code).await{
            if result.is_some(){
                let mut parameters= HashMap::new();
                parameters.insert("sku_code".to_string(), &self.sku_code);
                let e = ErrorCode::SkuAlreadyExists.to_error_response_from_parameters(parameters);
                return Err(e);
            }
        }else{
            let e = ErrorCode::InternalServerError.to_error_response_without_parameters();
            return Err(e);            
       }
        Ok(())
    }
}



#[cfg(test)]
mod test{
//为validate_price_list编写测试用例
    use super::*;
    use chrono::{Utc,TimeZone};
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    #[test]
    fn test_validate_price_list() {
        //模拟价格列表为空的情况
        let price_list = vec![];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的上一次结束时间大于下一次开始时间
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
            Price {
                sequence: 2,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 10, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2016, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格序号重复
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2016, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格序号不是从小到大
        let price_list = vec![
            Price {
                sequence: 2,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2016, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2016, 12, 31, 0, 0, 0).unwrap(),
            },
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的开始时间大于结束时间
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格为负数
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from(-100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格小数位数超过2位
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from_str("100.123").unwrap(),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格序号小于1
        let price_list = vec![
            Price {
                sequence: 0,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_err(), true);
        //模拟价格列表中的价格序号大于等于1
        let price_list = vec![
            Price {
                sequence: 1,
                price: BigDecimal::from(100),
                start_date_time: Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap(),
                end_date_time: Utc.with_ymd_and_hms(2015, 12, 31, 0, 0, 0).unwrap(),
            },
        ];
        let result = validate_price_list(&price_list);
        assert_eq!(result.is_ok(), true);
        
    }
}
