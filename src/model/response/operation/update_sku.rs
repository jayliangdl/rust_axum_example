use serde::Serialize;
#[derive(Serialize,Debug,Clone)]
pub struct UpdateSku{
    pub sku_code:String
}