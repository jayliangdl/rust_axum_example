use serde::Serialize;
use crate::model::db::sku::Sku;
#[derive(Serialize,Debug)]
pub struct FindSku{
    pub sku_code:String,
    pub name:String,
}


impl FindSku{
    pub fn from_db_sku(sku_option:Option<Sku>)->Option<FindSku>{
        if let Some(sku)=sku_option{
            let find_sku_response = FindSku{
                sku_code:sku.sku_code.clone(),
                name:sku.name.clone(),
            };
            Some(find_sku_response)
        }else{
            None
        }
    }
}
