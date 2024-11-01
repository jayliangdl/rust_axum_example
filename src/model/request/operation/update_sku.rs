use sqlx::MySqlPool;
use serde::Deserialize;
use validator::Validate;
use crate::model::request::operation::price::Price;
use validator::ValidationError;
use std::collections::HashMap;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use crate::model::db::sku::{
    Price as DbPrice,
    Sku as DbSku
};
use crate::dao::sku_dao::SkuDao;
use crate::utils::error::BusinessError;

#[derive(Deserialize,Debug, Validate)]
pub struct UpdateSku{
    #[validate(length(min = 1, message = "sku_code不能为空"))]
    pub sku_code: String,
    pub name: Option<String>,
    pub description: Option<String>,
    #[validate(custom = "validate_price_list")]
    pub price_list: Option<Vec<Price>>,
}

impl UpdateSku{
    pub async fn custom_validate(&self,pool:&MySqlPool)->Result<(), BusinessError>{
        self.validate()?;
        self.validate_not_found_sku(pool).await?;
        Ok(())
    }

    //校验sku是否存在    
    async fn validate_not_found_sku(&self, pool:&MySqlPool)->Result<(),BusinessError>{
        if let Ok(result) = SkuDao::find_sku(pool, &self.sku_code).await{
            if result.is_none(){
                let mut parameters= HashMap::new();
                parameters.insert("sku_code".to_string(), self.sku_code.clone());
                return Err(BusinessError::SkuNotFound(
                    (None,Some(parameters))
                ));   
            }
        }
        else{
            return Err(BusinessError::InternalServerError((Some("校验sku是否存在-执行失败".to_string()),None)));            
        }
        Ok(())
    }
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

impl UpdateSku{ 
    pub fn into_db_sku(&self)->DbSku{
        let mut db_price_list = vec![];
        //self.price_list是Option类型，所以需要判断是否有值，若有值，则将其转换为DbPrice类型
        if let Some(price_list) = &self.price_list{
            for price in price_list{
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
        }

        let db_sku = DbSku{
            sku_code:self.sku_code.clone(),
            name:self.name.clone().unwrap_or_else(||"".to_string()),
            description:self.description.clone()
        };
        db_sku
    }
}
