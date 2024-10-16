use serde::{Deserialize, Serialize};
#[derive(Deserialize,Debug,Serialize)]
pub struct FindSku{
    pub sku_code:String
}
