use serde::{Deserialize, Serialize};
#[derive(Serialize,Debug,Deserialize)]
pub struct FindSku{
    pub sku_code:String,
    pub name:String,
}